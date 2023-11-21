use crate::{
	ast::{BinaryOp, Expr, UnaryOp, Value},
	span::Span,
	tokenize::Token,
};
use chumsky::{
	extra,
	input::{Input, MapExtra, SpannedInput},
	prelude::*,
};

pub fn parse_module(tokens: &[(Token, Span)], src: &str) {
	module_parser()
		.parse(tokens.spanned(Span {
			start: src.len(),
			end: src.len(),
		}))
		.unwrap()
}

pub fn parse_expr(tokens: &[(Token, Span)], src: &str) -> Expr {
	expr_parser(src)
		.parse(tokens.spanned(Span {
			start: src.len(),
			end: src.len(),
		}))
		.unwrap()
}

type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
type Extra<'src> = extra::Full<Rich<'src, Token, Span>, (), &'src str>;

fn module_parser<'src>() -> impl Parser<'src, TokenStream<'src>, (), Extra<'src>> {
	end()
}

fn expr_parser<'src>(src: &'src str) -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(|value| Expr::Value(value));

		// atom ::= "(" expr ")" | literal | identifier
		let atom = choice((
			literal,
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			indentifier_parser(src).map(|name| Expr::Ident(name.into())),
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

		compare.labelled("expression")
	})
}

fn indentifier_parser<'src>(
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
