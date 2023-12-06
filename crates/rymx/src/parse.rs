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
	// constant ::= "const" ident (":" expr)? "=" expr
	let constant = just(Token::Const)
		.ignore_then(ident_parser(src))
		.then_ignore(just(Token::Colon).ignore_then(expr_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr_parser(src))
		.then_ignore(just(Token::Semi))
		.map(
			|(name, expr)| (name.into(), expr),
			// TODO typ: Type::Unknown,
		);

	let function = function_parser(expr_parser(src), src)
		.map(|func| (func.name.clone().unwrap(), Expr::Function(func)));

	// module ::= (constant | function)*
	choice((constant, function))
		.repeated()
		.collect()
		.map(|constants| Module {
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

		// statement ::= expr ";"
		let statement = expr.clone().then_ignore(just(Token::Semi));
		// block ::= "{" statement* expr? "}"
		let block = statement
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
			});

		// atom ::= literal | ident | "(" expr ")" | block
		let atom = choice((
			literal,
			ident_parser(src).map(String::from).map(Expr::Ident),
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			block,
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

		let function = function_parser(expr, src).map(|function| Expr::Function(function));

		// expr ::= function | var | if_else | compare
		choice((function, var, if_else, compare))
			.labelled("expression")
			.boxed()
	})
}

// TODO Destinct parsing for function statements and expressions
fn function_parser<'src>(
	expr: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone,
	src: &'src str,
) -> impl Parser<TokenStream, Function, Extra> + Clone {
	// parameter ::= ident (":" __TODO__)?
	let parameter = ident_parser(src)
		.then(just(Token::Colon).ignore_then(ident_parser(src)).or_not())
		.map(|(name, _typ)| name);

	// function ::= fn "ident" "(" (parameter ("," parameter)*)? ")" path? expr
	let function = just(Token::Fn)
		.ignore_then(ident_parser(src).or_not())
		.then(
			parameter
				.separated_by(just(Token::Comma))
				.collect::<Vec<&str>>()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
		)
		.then(ident_parser(src).or_not())
		.then(expr)
		.map(|(((name, params), return_type), body)| Function {
			name: name.map(String::from),
			params: params.into_iter().map(|name| (name.into(), ())).collect(),
			return_type: return_type.map(String::from),
			body: Box::new(body),
		})
		.labelled("function");

	function
}

// fn path_parser(src: &str) -> impl Parser<TokenStream, Path, Extra> + Clone {
// 	let ident = ident_parser(src).map(String::from);

// 	// path ::= ("." ident)+ | ident ("." ident)*
// 	just(Token::Dot)
// 		.or_not()
// 		.rewind()
// 		.then(
// 			ident
// 				.clone()
// 				.separated_by(just(Token::Dot))
// 				.allow_leading()
// 				.at_least(1)
// 				.collect::<Vec<String>>()
// 				.map(|segements| segements),
// 		)
// 		.map(|(dot, segments)| {
// 			let is_incomplete = dot.is_some();
// 			match (is_incomplete, segments.len()) {
// 				(true, _) => Path::Incomplete(segments),
// 				(false, 1) => Path::Simple(segments[0].clone()),
// 				(false, _) => Path::Full(segments),
// 			}
// 		})
// 		.boxed()
// 		.labelled("path")
// }

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
