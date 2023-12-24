use super::type_parser;
use crate::{ast::*, tokenize::Token, Span};
use chumsky::{
	input::{MapExtra, SpannedInput},
	prelude::*,
};

pub(super) type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
pub(super) type Extra<'src> = extra::Full<Rich<'src, Token, Span>, (), &'src str>;

pub(super) fn parameters_parser<'src>(
	src: &'src str,
) -> impl Parser<TokenStream, Vec<&str>, Extra> + Clone {
	// parameter ::= ident (":" type)?
	let parameter = ident_parser(src)
		.then(just(Token::Colon).ignore_then(type_parser(src)).or_not())
		.map(|(name, _typ)| name)
		.labelled("parameter");

	// parameters ::= (parameter ("," parameter)*)?
	let parameters = parameter
		.separated_by(just(Token::Comma))
		.allow_trailing()
		.collect::<Vec<&str>>();

	parameters
}

pub(super) fn block_parser<'src>(
	expr: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone + 'src,
	other: impl Parser<'src, TokenStream<'src>, Stmt, Extra<'src>> + Clone + 'src,
) -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone {
	// statement ::= expr ";" | variable | function
	let statement = choice((
		expr.clone().map(Stmt::Expr).then_ignore(just(Token::Semi)),
		other,
	));

	// block ::= "{" statement* expr? "}"
	statement
		.repeated()
		.collect::<Vec<Stmt>>()
		.then(
			expr.or_not(), // FIXME Not working
		)
		.delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
		.map(|(mut statements, final_expr)| {
			if let Some(expr) = final_expr {
				if !matches!(expr, Expr::Return(..) | Expr::Break(..)) {
					statements.push(Stmt::Expr(Expr::Break(Box::new(expr))));
				}
			}
			Expr::Block(statements)
		})
		.boxed()
}

pub fn literal_parser(src: &str) -> impl Parser<TokenStream, Literal, Extra> + Clone {
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

pub(super) fn ident_parser(src: &str) -> impl Parser<TokenStream, &str, Extra> + Clone {
	just(Token::Ident)
		.map_with(|_, extra| current_src(extra, src))
		.labelled("identifier")
}

/// Retrieve the substring of source code at the current span.
pub(super) fn current_src<'src>(
	extra: &mut MapExtra<'src, '_, TokenStream<'src>, Extra<'src>>,
	src: &'src str,
) -> &'src str {
	extra.span().src(src)
}
