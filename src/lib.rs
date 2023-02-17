#![feature(trait_alias)]

mod ast;
mod test;

use ast::{Expr, Item, Stmt};
use chumsky::prelude::*;
use rym_lexer::rich::Token;

type Span = std::ops::Range<usize>;
type Spanned<T> = (T, Span);

pub trait TokenParser<O> = Parser<Token, O, Error = Simple<Token>>;

/// line_end => ";" | EOF
fn line_end() -> impl TokenParser<()> {
	// TODO Look into
	// TODO   - automatic semicolon insertion eg. Js
	// TODO   - rules for omitting semicolons eg. Go

	choice((just(Token::Semi), just(Token::Eof))).ignored()
}

fn ident() -> impl TokenParser<String> {
	select! { Token::Ident(ident) => ident }
		// .map_with_span(|ident, span| (ident, span))
		.labelled("identifier")
}

/// item => func_decl | type_decl | var_decl
// pub fn item() -> impl TokenParser<Item> {
// 	recursive(|item| {
// 		let stmt = choice((
// 			item.map(|item| Stmt::Item(item)),
// 			// expr()
// 			// 	.then_ignore(line_end())
// 			// 	.map(|(expr, _)| Stmt::Expr(expr)),
// 		));

// 		let block = stmt
// 			.repeated()
// 			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
// 			.collect::<Vec<Stmt>>()
// 			.map_with_span(|stmts, span| (Expr::Block(stmts), span))
// 			.labelled("block");

// 		let params = ident()
// 			// .then_ignore(just(Token::Colon))
// 			.separated_by(just(Token::Comma))
// 			.allow_trailing()
// 			.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
// 			.labelled("function params");

// 		// func_decl => "func" IDENT "(" params ")" (block | line_end)
// 		let func_decl = just(Token::Func)
// 			.ignore_then(ident())
// 			.then(params)
// 			// TODO: return type
// 			// .then_ignore(just(Token::ThinArrow))
// 			// .then(type_expr)
// 			.then(block.map(|block| Some(block)).or(line_end().map(|_| None)))
// 			.map(|((name, params), body)| Item::Func { name, params, body });

// 		// let type_decl = just(Token::Type)
// 		// 	.ignore_then(ident())
// 		// 	.map(|name| Item::Type { name });

// 		func_decl /* .or(type_decl) */
// 	})
// }

pub fn expr_parser() -> impl TokenParser<Spanned<Expr>> {
	recursive(|expr| {
		let raw_expr = recursive(|raw_expr| {
			let literal = select! {
				Token::Int(val) => Expr::Int(val),
				Token::Float(r_val, l_val) => Expr::Float(r_val, l_val),
				Token::Char(val) => Expr::Char(val),
				Token::String(val) => Expr::String(val),
			};

			let atom = choice((literal, ident().map(Expr::Ident)))
				.map_with_span(|expr, span| (expr, span))
				.or(raw_expr.delimited_by(just(Token::OpenParen), just(Token::CloseParen)));

			// List of expressions without delimiters
			let items = expr.separated_by(just(Token::Comma)).allow_trailing();

			let call = atom
				.then(
					items
						.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
						.map_with_span(|args, span| (args, span))
						.repeated(),
				)
				.foldl(|spanned_func, (args, args_span)| {
					let span = spanned_func.1.start..args_span.end;
					(Expr::Call(Box::new(spanned_func), args), span)
				});

			call
		});

		raw_expr
	})
}
