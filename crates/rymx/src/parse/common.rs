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

pub fn literal_parser(src: &str) -> impl Parser<TokenStream, Literal, Extra> + Clone {
	let integer = integer_parser(src).map(Literal::Int);

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

pub fn integer_parser(src: &str) -> impl Parser<TokenStream, i64, Extra> + Clone {
	just(Token::Int)
		.map_with(|_, extra| {
			current_src(extra, src)
				.parse()
				.expect("Internal Error: Failed to parse u64")
		})
		.labelled("integer")
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
