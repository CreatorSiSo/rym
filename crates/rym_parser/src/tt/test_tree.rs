#![cfg(test)]

use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use rym_tt::{DelimSpan, Delimiter, Token, TokenKind, TokenStream, TokenTree};
use smol_str::SmolStr;

use super::TreeLexer;

fn map_tokenstream_errors(src: &str) -> (TokenStream, Vec<Diagnostic>) {
	let mut error_handler = Handler::default();
	let token_stream: TokenStream = TreeLexer::new(src, &mut error_handler).collect();
	(token_stream, error_handler.collect())
}

#[track_caller]
fn assert_tokentrees_errs(src: &str, ts: impl Into<TokenStream>, errs: Vec<Diagnostic>) {
	assert_eq!(map_tokenstream_errors(src), (ts.into(), errs));
}

#[track_caller]
fn assert_kinds_errs(src: &str, kinds: Vec<TokenKindTree>, errs: Vec<Diagnostic>) {
	fn transform(token_stream: TokenStream) -> Vec<TokenKindTree> {
		token_stream.into_iter().fold(vec![], |mut accum, tt| {
			match tt {
				TokenTree::Token(token) => accum.push(TokenKindTree::Kind(token.kind)),
				TokenTree::Delimited(_, delim, delim_ts) => {
					accum.push(TokenKindTree::Delimited(delim, transform(delim_ts)))
				}
			};
			accum
		})
	}

	let got = map_tokenstream_errors(src);
	assert_eq!((got.1, transform(got.0)), (errs, kinds));
}

#[derive(Debug, PartialEq)]
pub enum TokenKindTree {
	/// Single token.
	Kind(TokenKind),
	/// Delimited sequence of token trees.
	Delimited(Delimiter, Vec<TokenKindTree>),
}

#[test]
fn call() {
	assert_tokentrees_errs(
		"empty_call()",
		vec![
			TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("empty_call")), Span::new(0, 10))),
			TokenTree::Delimited(
				DelimSpan { open: Span::new(10, 11), close: Span::new(11, 12), entire: Span::new(10, 12) },
				rym_tt::Delimiter::Paren,
				TokenStream(vec![]),
			),
		],
		vec![],
	);
	assert_tokentrees_errs(
		"call(arg_1: float, arg_2: bool = true)",
		vec![
			TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("call")), Span::new(0, 4))),
			TokenTree::Delimited(
				DelimSpan { open: Span::new(4, 5), close: Span::new(37, 38), entire: Span::new(4, 38) },
				rym_tt::Delimiter::Paren,
				vec![
					TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("arg_1")), Span::new(5, 10))),
					TokenTree::Token(Token::new(TokenKind::Colon, Span::new(10, 11))),
					TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("float")), Span::new(12, 17))),
					TokenTree::Token(Token::new(TokenKind::Comma, Span::new(17, 18))),
					TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("arg_2")), Span::new(19, 24))),
					TokenTree::Token(Token::new(TokenKind::Colon, Span::new(24, 25))),
					TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("bool")), Span::new(26, 30))),
					TokenTree::Token(Token::new(TokenKind::Eq, Span::new(31, 32))),
					TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("true")), Span::new(33, 37))),
				]
				.into(),
			),
		],
		vec![],
	);
	assert_kinds_errs(
		"call(arg_1: float, arg_2: bool = true)",
		vec![
			TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("call"))),
			TokenKindTree::Delimited(
				Delimiter::Paren,
				vec![
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("arg_1"))),
					TokenKindTree::Kind(TokenKind::Colon),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("float"))),
					TokenKindTree::Kind(TokenKind::Comma),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("arg_2"))),
					TokenKindTree::Kind(TokenKind::Colon),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("bool"))),
					TokenKindTree::Kind(TokenKind::Eq),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("true"))),
				],
			),
		],
		vec![],
	)
}

#[test]
fn function() {
	assert_kinds_errs(
		"fn add(a: int, b: int) -> int { a + b }",
		vec![
			TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("fn"))),
			TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("add"))),
			TokenKindTree::Delimited(
				Delimiter::Paren,
				vec![
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("a"))),
					TokenKindTree::Kind(TokenKind::Colon),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("int"))),
					TokenKindTree::Kind(TokenKind::Comma),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("b"))),
					TokenKindTree::Kind(TokenKind::Colon),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("int"))),
				],
			),
			TokenKindTree::Kind(TokenKind::Minus),
			TokenKindTree::Kind(TokenKind::GreaterThan),
			TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("int"))),
			TokenKindTree::Delimited(
				Delimiter::Brace,
				vec![
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("a"))),
					TokenKindTree::Kind(TokenKind::Plus),
					TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("b"))),
				],
			),
		],
		vec![],
	)
}

#[test]
fn unclosed() {
	// TODO Add multiline variants of tests
	assert_kinds_errs(
		"({[",
		vec![TokenKindTree::Delimited(
			Delimiter::Paren,
			vec![TokenKindTree::Delimited(
				Delimiter::Brace,
				vec![TokenKindTree::Delimited(Delimiter::Bracket, vec![])],
			)],
		)],
		vec![
			Diagnostic::new_spanned(Level::Error, "Unclosed delimiter", Span::new(2, 3)),
			Diagnostic::new_spanned(Level::Error, "Unclosed delimiter", Span::new(1, 3)),
			Diagnostic::new_spanned(Level::Error, "Unclosed delimiter", Span::new(0, 3)),
		],
	);
	assert_kinds_errs(
		"{ a + b",
		vec![TokenKindTree::Delimited(
			Delimiter::Brace,
			vec![
				TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("a"))),
				TokenKindTree::Kind(TokenKind::Plus),
				TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("b"))),
			],
		)],
		vec![Diagnostic::new_spanned(Level::Error, "Unclosed delimiter", Span::new(0, 7))],
	);
	assert_kinds_errs(
		"{ a + (b)",
		vec![TokenKindTree::Delimited(
			Delimiter::Brace,
			vec![
				TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("a"))),
				TokenKindTree::Kind(TokenKind::Plus),
				TokenKindTree::Delimited(
					Delimiter::Paren,
					vec![TokenKindTree::Kind(TokenKind::Ident(SmolStr::new("b")))],
				),
			],
		)],
		vec![Diagnostic::new_spanned(Level::Error, "Unclosed delimiter", Span::new(0, 9))],
	)
}
