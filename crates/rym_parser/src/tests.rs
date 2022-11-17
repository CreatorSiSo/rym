#![cfg(test)]

use rym_lexer::Cursor;
use rym_span::Span;
use rym_tt::{Delimiter, LitKind, Token, TokenKind};
use smol_str::SmolStr;

use crate::*;

fn assert_results(src: &str, results: &[Result<Token, Span>]) {
	let result: Vec<_> = TokenConverter::new(src, Cursor::new(src)).collect();
	assert_eq!(result, results)
}

fn assert_tokens(src: &str, tokens_1: &[Token]) {
	let tokens_2: Vec<Token> = TokenConverter::new(src, Cursor::new(src))
		.map(|result| match result {
			Ok(token) => token,
			Err(err) => panic!("Expected no errors got: {err:?}"),
		})
		.collect();
	assert_eq!(tokens_1, tokens_2)
}

fn assert_token_kinds(src: &str, token_kinds_1: &[TokenKind]) {
	let token_kinds_2: Vec<TokenKind> = TokenConverter::new(src, Cursor::new(src))
		.map(|result| match result {
			Ok(token) => token.kind,
			Err(err) => panic!("Expected no errors got: {err:?}"),
		})
		.collect();
	assert_eq!(token_kinds_1, token_kinds_2)
}

#[test]
fn empty() {
	assert_results("", &[])
}

#[test]
fn expr_if() {
	let src = r#"if test == true { "is correct" } else { "is not correct" };"#;
	assert_tokens(
		src,
		&[
			Token::new(TokenKind::Ident(SmolStr::new("if")), Span::new(0, 2)),
			Token::new(TokenKind::Ident(SmolStr::new("test")), Span::new(3, 7)),
			Token::new(TokenKind::Eq, Span::new(8, 9)),
			Token::new(TokenKind::Eq, Span::new(9, 10)),
			Token::new(TokenKind::Ident(SmolStr::new("true")), Span::new(11, 15)),
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(16, 17)),
			Token::new(
				TokenKind::Literal(LitKind::String(SmolStr::new("is correct"))),
				Span::new(18, 30),
			),
			Token::new(TokenKind::CloseDelim(Delimiter::Brace), Span::new(31, 32)),
			Token::new(TokenKind::Ident(SmolStr::new("else")), Span::new(33, 37)),
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(38, 39)),
			Token::new(
				TokenKind::Literal(LitKind::String(SmolStr::new("is not correct"))),
				Span::new(40, 56),
			),
			Token::new(TokenKind::CloseDelim(Delimiter::Brace), Span::new(57, 58)),
			Token::new(TokenKind::Semi, Span::new(58, 59)),
		],
	);
	assert_token_kinds(
		src,
		&[
			TokenKind::Ident(SmolStr::new("if")),
			TokenKind::Ident(SmolStr::new("test")),
			TokenKind::Eq,
			TokenKind::Eq,
			TokenKind::Ident(SmolStr::new("true")),
			TokenKind::OpenDelim(Delimiter::Brace),
			TokenKind::Literal(LitKind::String(SmolStr::new("is correct"))),
			TokenKind::CloseDelim(Delimiter::Brace),
			TokenKind::Ident(SmolStr::new("else")),
			TokenKind::OpenDelim(Delimiter::Brace),
			TokenKind::Literal(LitKind::String(SmolStr::new("is not correct"))),
			TokenKind::CloseDelim(Delimiter::Brace),
			TokenKind::Semi,
		],
	);
}

#[test]
fn literals() {
	assert_token_kinds(
		r#"999_999_999 1.284_324_68 'c' '\n' '\x41' '\u24B6' '\u8DEF' "Hello World!\n""#,
		&[
			TokenKind::Literal(LitKind::Integer(999_999_999)),
			TokenKind::Literal(LitKind::Float(1.284_324_68)),
			TokenKind::Literal(LitKind::Char('c')),
			TokenKind::Literal(LitKind::Char('\n')),
			TokenKind::Literal(LitKind::Char('A')),
			TokenKind::Literal(LitKind::Char('Ⓐ')),
			TokenKind::Literal(LitKind::Char('路')),
			TokenKind::Literal(LitKind::String(SmolStr::new("Hello World!\n"))),
		],
	)
}
