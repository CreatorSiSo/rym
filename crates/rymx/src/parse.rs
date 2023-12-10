use crate::{
	ast::{BinaryOp, Expr, Literal, Module, UnaryOp, VariableKind},
	interpret::Function,
	span::{SourceSpan, Span},
	tokenize::Token,
	SourceId,
};
use ariadne::{Label, Report};
use chumsky::{
	extra,
	input::{Input, MapExtra, SpannedInput},
	prelude::*,
};

pub fn parse_module<'a>(
	tokens: &'a [(Token, Span)],
	src: &'a str,
	src_id: SourceId,
) -> Result<Module, Vec<Report<'a, SourceSpan>>> {
	let parse_result = module_parser(src).parse(tokens.spanned(Span {
		start: src.len(),
		end: src.len(),
	}));

	map_parse_result(parse_result, src_id)
}

pub fn parse_expr<'a>(
	tokens: &'a [(Token, Span)],
	src: &'a str,
	src_id: SourceId,
) -> Result<Expr, Vec<Report<'a, SourceSpan>>> {
	let parse_result = expr_parser(src).parse(tokens.spanned(Span {
		start: src.len(),
		end: src.len(),
	}));

	map_parse_result(parse_result, src_id)
}

fn map_parse_result<T>(
	parse_result: ParseResult<T, Rich<'_, Token, Span>>,
	src_id: SourceId,
) -> Result<T, Vec<Report<'_, SourceSpan>>> {
	parse_result.into_result().map_err(|errs| {
		errs
			.into_iter()
			.map(|err| {
				Report::build(ariadne::ReportKind::Error, src_id.clone(), err.span().start)
					.with_label(
						Label::new(SourceSpan(src_id.clone(), *err.span()))
							.with_message(format!("{:?}", err.reason())),
					)
					.finish()
			})
			.collect()
	})
}

type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
type Extra<'src> = extra::Full<Rich<'src, Token, Span>, (), &'src str>;

fn module_parser(src: &str) -> impl Parser<TokenStream, Module, Extra> {
	let function_stmt = recursive(|function_stmt| {
		// function_stmt ::= "fn" ident "(" parameters ")" type? block
		just(Token::Fn)
			.ignore_then(ident_parser(src))
			.then(parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
			.then(ident_parser(src).or_not())
			.then(block_parser(expr_parser(src), function_stmt))
			.map(|(((name, params), return_type), body)| {
				Expr::Function(make_function(
					FnKind::Stmt,
					Some(name),
					params,
					return_type,
					body,
				))
			})
	})
	.map(|expr| match expr {
		Expr::Function(ref func) => (func.name.clone().unwrap(), expr),
		_ => unreachable!(),
	});

	// constant ::= "const" ident (":" type) "=" expr ";"
	let constant = just(Token::Const)
		.ignore_then(ident_parser(src))
		.then(just(Token::Colon).ignore_then(ident_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr_parser(src))
		.then_ignore(just(Token::Semi))
		.map(|((name, _typ), expr)| (name.to_string(), expr));

	// global ::= constnt | function_stmt
	let global = constant.or(function_stmt);

	// module ::= (global)*
	global.repeated().collect().map(|constants| Module {
		// TODO
		name: "".into(),
		constants,
		children: vec![],
	})
}

fn expr_parser(src: &str) -> impl Parser<TokenStream, Expr, Extra> + Clone {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(Expr::Literal);

		// function_stmt ::= fn "ident" "(" parameters ")" ident? block
		let function_stmt = recursive(|function_stmt| {
			just(Token::Fn)
				.ignore_then(ident_parser(src))
				.then(parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
				.then(ident_parser(src).or_not())
				.then(block_parser(expr.clone(), function_stmt).map(|expr| expr))
				.map(|(((name, params), return_type), body)| {
					Expr::Function(make_function(
						FnKind::Stmt,
						Some(name),
						params,
						return_type,
						body,
					))
				})
				.labelled("function statement")
		});

		// function_expr ::= fn "(" parameters ")" ident? "=>" expr
		let function_expr = just(Token::Fn)
			.ignore_then(
				parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			)
			.then(ident_parser(src).or_not())
			.then(just(Token::ThickArrow).ignore_then(expr.clone()))
			.map(|((params, return_type), body)| {
				Expr::Function(make_function(FnKind::Expr, None, params, return_type, body))
			})
			.labelled("function expression");

		// atom ::= literal | ident | "(" expr ")" | block
		let atom = choice((
			literal,
			ident_parser(src).map(String::from).map(Expr::Ident),
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			block_parser(expr.clone(), function_stmt),
		))
		.labelled("atom");

		// call ::= atom ("(" (expr ("," expr)*)? ")")?
		let call = atom
			.then(
				expr
					.clone()
					.separated_by(just(Token::Comma))
					.collect::<Vec<Expr>>()
					.delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
					.or_not(),
			)
			.map(|(lhs, args)| match args {
				Some(args) => Expr::Call(Box::new(lhs), args),
				None => lhs,
			})
			.boxed();

		// chain ::= call ("." call)*
		let chain = call
			.clone()
			.foldl(just(Token::Dot).ignore_then(call).repeated(), |lhs, rhs| {
				Expr::Chain(Box::new(lhs), Box::new(rhs))
			});

		// unary ::= ("-" | "not")* chain
		let unary = select! {
			Token::Minus => UnaryOp::Neg,
			Token::Not => UnaryOp::Not,
		}
		.repeated()
		.at_least(1)
		.collect::<Vec<UnaryOp>>()
		.or_not()
		.then(chain)
		.map(|(maybe_op, expr)| {
			let Some(op) = maybe_op else {
				return expr;
			};
			op.into_iter()
				.fold(expr, |accum, op| Expr::Unary(op, Box::new(accum)))
		});

		// sum ::= unary (("*" | "/") unary)*
		let product = unary
			.clone()
			.foldl_with(
				select! {
					Token::Star => BinaryOp::Mul,
					Token::Slash => BinaryOp::Div,
				}
				.then(unary)
				.repeated(),
				|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
			)
			.boxed();

		// sum ::= product (("+" | "-") product)*
		let sum = product.clone().foldl_with(
			select! {
				Token::Plus => BinaryOp::Add,
				Token::Minus => BinaryOp::Sub,
			}
			.then(product)
			.repeated(),
			|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
		);

		// compare ::= sum (("==" | "!=" | "<" | "<=" | ">" | ">=") sum)*
		let compare = sum
			.clone()
			.foldl_with(
				select! {
					Token::Eq => BinaryOp::Eq,
					Token::NotEq => BinaryOp::NotEq,
					Token::LessThan => BinaryOp::LessThan,
					Token::LessThanEq => BinaryOp::LessThanEq,
					Token::GreaterThan => BinaryOp::GreaterThan,
					Token::GreaterThanEq => BinaryOp::GreaterThanEq,
				}
				.then(sum)
				.repeated(),
				|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
			)
			.boxed();

		// var ::= ("const" | "let" | "let mut") indent (":" expr)? "=" expr
		let var = choice((
			just(Token::Const).to(VariableKind::Const),
			just(Token::Let)
				.then(just(Token::Mut))
				.to(VariableKind::LetMut),
			just(Token::Let).to(VariableKind::Let),
		))
		.then(ident_parser(src))
		.then_ignore(just(Token::Colon).ignore_then(expr.clone()).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr.clone())
		.map(
			|((kind, name), rhs)| Expr::Var(kind, name.into(), Box::new(rhs)),
			// TODO typ: Type::Unknown,
		)
		.boxed()
		.labelled("variable");

		let if_else = just(Token::If)
			.ignore_then(expr.clone())
			.then_ignore(just(Token::Then))
			.then(expr.clone())
			.then(just(Token::Else).ignore_then(expr.clone()).or_not())
			.map(|((cond, then_branch), else_branch)| {
				Expr::IfElse(
					Box::new(cond),
					Box::new(then_branch),
					Box::new(else_branch.unwrap_or(Expr::Unit)),
				)
			});

		// return ::= "return" expr?
		let r#return = just(Token::Return)
			.ignore_then(expr.or_not())
			.map(|inner| Expr::Return(Box::new(inner.unwrap_or(Expr::Unit))));

		// expr ::= function | var | if_else | compare | return
		choice((function_expr, var, if_else, compare, r#return))
			.labelled("expression")
			.boxed()
	})
}

enum FnKind {
	// All types except the return type must be written down
	// default return type is unit
	Stmt,
	// Types will be inferred
	// default return type is inferred/unkown
	Expr,
}

fn make_function(
	fn_kind: FnKind,
	name: Option<&str>,
	params: Vec<&str>,
	return_type: Option<&str>,
	body: Expr,
) -> Function {
	Function {
		name: name.map(String::from),
		params: params.into_iter().map(|name| name.to_string()).collect(),
		return_type: match fn_kind {
			FnKind::Stmt => return_type.unwrap_or("()").into(),
			FnKind::Expr => return_type.unwrap_or("Unknown").into(),
		},
		body: Box::new(body),
	}
}

fn parameters_parser<'src>(src: &'src str) -> impl Parser<TokenStream, Vec<&str>, Extra> + Clone {
	// parameter ::= ident (":" __TODO__)?
	let parameter = ident_parser(src)
		.then(just(Token::Colon).ignore_then(ident_parser(src)).or_not())
		.map(|(name, _typ)| name)
		.labelled("parameter");

	// parameters ::= (parameter ("," parameter)*)?
	let parameters = parameter
		.separated_by(just(Token::Comma))
		.allow_trailing()
		.collect::<Vec<&str>>();

	parameters
}

fn block_parser<'src>(
	expr: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone + 'src,
	function_stmt: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone + 'src,
) -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone {
	// statement ::= expr ";" | function
	let statement = choice((expr.clone().then_ignore(just(Token::Semi)), function_stmt));

	// block ::= "{" statement* expr? "}"
	statement
		.repeated()
		.collect::<Vec<Expr>>()
		.then(expr.clone().or_not())
		.delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
		.map(|(mut exprs, last)| {
			if let Some(last) = last {
				if !matches!(last, Expr::Return(..) | Expr::Break(..)) {
					exprs.push(Expr::Break(Box::new(last)));
				}
			}
			Expr::Block(exprs)
		})
		.boxed()
}

fn ident_parser(src: &str) -> impl Parser<TokenStream, &str, Extra> + Clone {
	just(Token::Ident)
		.map_with(|_, extra| current_src(extra, src))
		.labelled("indentifier")
}

fn literal_parser(src: &str) -> impl Parser<TokenStream, Literal, Extra> + Clone {
	let integer = just(Token::Int)
		.map_with(|_, extra| {
			Literal::Int(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse u64"),
			)
		})
		.labelled("integer");

	let float = just(Token::Float)
		.map_with(|_, extra| {
			Literal::Float(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse f64"),
			)
		})
		.labelled("float");

	let string = just(Token::String)
		.map_with(|_, extra| {
			Literal::String({
				let mut span: Span = extra.span();
				span.start += 1;
				span.end -= 1;
				span.src(src).into()
			})
		})
		.labelled("string");

	choice((integer, float, string)).labelled("literal").boxed()
}

/// Retrieve the substring of source code at the current span.
fn current_src<'src>(
	extra: &mut MapExtra<'src, '_, TokenStream<'src>, Extra<'src>>,
	src: &'src str,
) -> &'src str {
	extra.span().src(src)
}
