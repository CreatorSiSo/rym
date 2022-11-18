#![cfg(test)]

use rym_lexer::Cursor;
use rym_span::Span;
use rym_tt::{Delimiter, LitKind, Token, TokenKind};
use smol_str::SmolStr;

use super::ConvertLinear;

fn assert_results(src: &str, expect: &[Result<Token, Span>]) {
	let got: Vec<_> = ConvertLinear::new(src, Cursor::new(src)).collect();
	assert_eq!(expect, got)
}

fn assert_tokens(src: &str, expect: &[Token]) {
	let got: Vec<Token> = ConvertLinear::new(src, Cursor::new(src))
		.map(|result| match result {
			Ok(token) => token,
			Err(err) => panic!("Expected no errors got: {err:?}"),
		})
		.collect();
	assert_eq!(got, expect)
}

fn assert_token_kinds(src: &str, expect: &[TokenKind]) {
	let got: Vec<TokenKind> = ConvertLinear::new(src, Cursor::new(src))
		.map(|result| match result {
			Ok(token) => token.kind,
			Err(err) => panic!("Expected no errors got: {err:?}"),
		})
		.collect();
	assert_eq!(got, expect)
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
			Token::new(TokenKind::EqEq, Span::new(8, 10)),
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
			TokenKind::EqEq,
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

#[test]
fn path() {
	assert_token_kinds(
		r#"std::__test::me: ::global_type"#,
		&[
			TokenKind::Ident(SmolStr::new("std")),
			TokenKind::ColonColon,
			TokenKind::Ident(SmolStr::new("__test")),
			TokenKind::ColonColon,
			TokenKind::Ident(SmolStr::new("me")),
			TokenKind::Colon,
			TokenKind::ColonColon,
			TokenKind::Ident(SmolStr::new("global_type")),
		],
	)
}

#[test]
fn operators() {
	assert_token_kinds(
		r#"| || & && + += - -= * *= / /= % %= = == ! != < <= > >="#,
		&[
			TokenKind::Or,
			TokenKind::OrOr,
			TokenKind::And,
			TokenKind::AndAnd,
			TokenKind::Plus,
			TokenKind::PlusEq,
			TokenKind::Minus,
			TokenKind::MinusEq,
			TokenKind::Star,
			TokenKind::StarEq,
			TokenKind::Slash,
			TokenKind::SlashEq,
			TokenKind::Percent,
			TokenKind::PercentEq,
			TokenKind::Eq,
			TokenKind::EqEq,
			TokenKind::Bang,
			TokenKind::BangEq,
			TokenKind::LessThan,
			TokenKind::LessThanEq,
			TokenKind::GreaterThan,
			TokenKind::GreaterThanEq,
		],
	)
}
