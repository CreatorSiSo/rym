#![feature(trait_alias)]

mod ast;
mod test;

use ast::{Expr, Item};
use chumsky::prelude::*;
use rym_lexer::rich::{Delimiter, Token};

type Span = std::ops::Range<usize>;
type Spanned<T> = (T, Span);

pub trait TokenParser<O> = Parser<Token, O, Error = Simple<Token>>;

// fn lit() -> impl TokenParser<Spanned<Expr>> {
// 	select! { Token::(ident) => ident.clone() }.labelled("identifier")
// }

fn line_end() -> impl TokenParser<()> {
	just(Token::Newline).or(just(Token::Semi)).ignored()
}

fn ident() -> impl TokenParser<Spanned<String>> {
	select! { Token::Ident(ident) => ident.clone() }
		.map_with_span(|ident, span| (ident, span))
		.labelled("identifier")
}

pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> {
	ident().map(|(ident, span)| (Expr::Ident(ident), span))
}

pub fn item_parser() -> impl TokenParser<Item> {
	let args = ident()
		// .then_ignore(just(Token::Colon))
		.separated_by(just(Token::Comma))
		.allow_trailing()
		.delimited_by(
			just(Token::OpenDelim(Delimiter::Paren)),
			just(Token::CloseDelim(Delimiter::Paren)),
		)
		.labelled("function args");

	let func_decl = just(Token::Func)
		.ignore_then(ident())
		.then(args)
		.map(|(name, params)| Item::Func {
			name,
			params,
			body: None,
		});

	let type_decl = just(Token::Type)
		.ignore_then(ident())
		.map(|name| Item::Type { name });

	func_decl.or(type_decl).then_ignore(line_end())
}
