use crate::{
	ast::{Expr, Value},
	span::Span,
	tokenize::Token,
};
use chumsky::{
	extra,
	input::{Input, MapExtra, SpannedInput},
	prelude::*,
};

pub fn parse_module(tokens: &[(Token, Span)], src: &str) {
	module_parser().parse(tokens.spanned(Span {
		start: src.len(),
		end: src.len(),
	}));
}

pub fn parse_expr(tokens: &[(Token, Span)], src: &str) {
	expr_parser().parse(tokens.spanned(Span {
		start: src.len(),
		end: src.len(),
	}));
}

type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
type Extra<'src> = extra::Full<EmptyErr, (), &'src str>;

fn module_parser<'src>() -> impl Parser<'src, TokenStream<'src>, (), Extra<'src>> {
	end()
}

fn expr_parser<'src>() -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> {
	let literal = choice((
		just(Token::String).map_with(|_, extra| Value::String(current_src(extra).into())),
		// just(Token::Float).map(|_| Value::String(value)),
	))
	.map(|value| Expr::Value(value));

	literal
}

/// Retrieve the substring of source code at the current span.
fn current_src<'src>(extra: &mut MapExtra<'src, '_, TokenStream<'src>, Extra<'src>>) -> &'src str {
	let src: &str = *extra.ctx();
	let span: Span = extra.span();
	span.src(src)
}
