use crate::{
	ast::{BinaryOp, Constant, Expr, Module, UnaryOp, Value},
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
				Report::build(ariadne::ReportKind::Error, src_id.clone(), 0)
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

fn module_parser<'src>(
	src: &'src str,
) -> impl Parser<'src, TokenStream<'src>, Module, Extra<'src>> {
	constant_parser(expr_parser(src), src)
		.then_ignore(just(Token::Semi))
		.repeated()
		.collect::<Vec<Constant>>()
		.map(|constants| Module {
			// TODO
			name: "".into(),
			constants,
			children: vec![],
		})
}

fn constant_parser<'src>(
	expr: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone,
	src: &'src str,
) -> impl Parser<'src, TokenStream<'src>, Constant, Extra<'src>> + Clone {
	// constant ::= "const" indent "=" expr
	just(Token::Const)
		.ignore_then(indent_parser(src))
		.then_ignore(just(Token::Assign))
		.then(expr)
		.map(|(name, expr)| Constant {
			name: name.into(),
			data: Box::new(expr),
			// TODO typ: Type::Unknown,
		})
}

fn expr_parser<'src>(
	src: &'src str,
) -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(|value| Expr::Value(value));

		// atom ::= "(" expr ")" | literal | ident
		let atom = choice((
			literal,
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			indent_parser(src).map(|name| Expr::Ident(name.into())),
		))
		.labelled("atom");

		// unary ::= ("-" | "not")* atom
		let unary = choice((
			select! {
				Token::Minus => UnaryOp::Neg,
				Token::Not => UnaryOp::Not,
			}
			.repeated()
			.collect::<Vec<UnaryOp>>()
			.then(atom.clone())
			.map(|(op, expr)| {
				op.into_iter()
					.fold(expr, |accum, op| Expr::Unary(op, Box::new(accum)))
			}),
			atom,
		));

		// sum ::= unary (("*" | "/") unary)*
		let product = unary.clone().foldl_with(
			select! {
				Token::Star => BinaryOp::Mul,
				Token::Slash => BinaryOp::Div,
			}
			.then(unary)
			.repeated(),
			|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
		);

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

		// compare ::= sum (("==" | "!=") sum)*
		let compare = sum.clone().foldl_with(
			select! {
				Token::Eq => BinaryOp::Eq,
				Token::NotEq => BinaryOp::NotEq,
			}
			.then(sum)
			.repeated(),
			|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
		);

		// expr ::= compare | constant
		choice((
			compare,
			constant_parser(expr, src).map(|constant| Expr::Constant(constant)),
		))
		.labelled("expression")
	})
}

fn indent_parser<'src>(
	src: &'src str,
) -> impl Parser<'src, TokenStream<'src>, &'src str, Extra<'src>> + Clone {
	just(Token::Ident)
		.map_with(|_, extra| current_src(extra, src).into())
		.labelled("indentifier")
}

fn literal_parser<'src>(
	src: &'src str,
) -> impl Parser<'src, TokenStream<'src>, Value, Extra<'src>> + Clone {
	let integer = just(Token::Int)
		.map_with(|_, extra| {
			Value::Int(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse u64"),
			)
		})
		.labelled("integer");

	let float = just(Token::Float)
		.map_with(|_, extra| {
			Value::Float(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse f64"),
			)
		})
		.labelled("float");

	let string = just(Token::String)
		.map_with(|_, extra| Value::String(current_src(extra, src).into()))
		.labelled("string");

	choice((integer, float, string)).labelled("literal")
}

/// Retrieve the substring of source code at the current span.
fn current_src<'src>(
	extra: &mut MapExtra<'src, '_, TokenStream<'src>, Extra<'src>>,
	src: &'src str,
) -> &'src str {
	extra.span().src(src)
}
