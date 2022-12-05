#![cfg(test)]

use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use smol_str::SmolStr;

use super::{Delimiter, LinearLexer, LitKind, Token, TokenKind};

#[track_caller]
fn assert_results(src: &str, expect: &[Token], diagnostics: &[Diagnostic]) {
	let handler = Handler::default();
	let got: Vec<_> = LinearLexer::new(src, &handler).collect();
	println!("{got:#?}");
	assert_eq!(expect, got);
	assert_eq!(diagnostics, &handler.collect());
}

#[track_caller]
fn assert_tokens(src: &str, expect: &[Token]) {
	let handler = Handler::default();
	let got: Vec<Token> = LinearLexer::new(src, &handler).collect();
	let errors = handler.collect();
	assert_eq!(errors, vec![], "Expected no errors got: {errors:?}");
	println!("{got:#?}");
	assert_eq!(got, expect);
}

#[track_caller]
fn assert_diagnostics(src: &str, expect: &[Diagnostic]) {
	let handler = Handler::default();
	let _: Vec<_> = LinearLexer::new(src, &handler).collect();
	let errors = handler.collect();
	println!("{errors:#?}",);
	assert_eq!(errors, expect)
}

#[track_caller]
fn assert_token_kinds(src: &str, expect: &[TokenKind]) {
	let handler = Handler::default();
	let got: Vec<TokenKind> = LinearLexer::new(src, &handler).map(|token| token.kind).collect();
	let errors = handler.collect();
	assert_eq!(errors, vec![], "Expected no errors got: {errors:?}");
	println!("{got:#?}");
	assert_eq!(got, expect);
}

#[test]
fn empty() {
	assert_results("", &[], &[])
}

#[test]
fn expr_if() {
	let src = r#"if test == true { "is correct" } else { "is not correct" };"#;
	assert_tokens(
		src,
		&[
			Token::new(TokenKind::If, Span::new(0, 2)),
			Token::new(TokenKind::Ident(SmolStr::new("test")), Span::new(3, 7)),
			Token::new(TokenKind::EqEq, Span::new(8, 10)),
			Token::new(TokenKind::Ident(SmolStr::new("true")), Span::new(11, 15)),
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(16, 17)),
			Token::new(TokenKind::Literal(LitKind::String("is correct".into())), Span::new(18, 30)),
			Token::new(TokenKind::CloseDelim(Delimiter::Brace), Span::new(31, 32)),
			Token::new(TokenKind::Else, Span::new(33, 37)),
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(38, 39)),
			Token::new(TokenKind::Literal(LitKind::String("is not correct".into())), Span::new(40, 56)),
			Token::new(TokenKind::CloseDelim(Delimiter::Brace), Span::new(57, 58)),
			Token::new(TokenKind::Semi, Span::new(58, 59)),
		],
	);
	assert_token_kinds(
		src,
		&[
			TokenKind::If,
			TokenKind::Ident(SmolStr::new("test")),
			TokenKind::EqEq,
			TokenKind::Ident(SmolStr::new("true")),
			TokenKind::OpenDelim(Delimiter::Brace),
			TokenKind::Literal(LitKind::String("is correct".into())),
			TokenKind::CloseDelim(Delimiter::Brace),
			TokenKind::Else,
			TokenKind::OpenDelim(Delimiter::Brace),
			TokenKind::Literal(LitKind::String("is not correct".into())),
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
			TokenKind::Literal(LitKind::Int(999_999_999)),
			TokenKind::Literal(LitKind::Float(1.284_324_68)),
			TokenKind::Literal(LitKind::Char('c')),
			TokenKind::Literal(LitKind::Char('\n')),
			TokenKind::Literal(LitKind::Char('A')),
			TokenKind::Literal(LitKind::Char('Ⓐ')),
			TokenKind::Literal(LitKind::Char('路')),
			TokenKind::Literal(LitKind::String("Hello World!\n".into())),
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

#[test]
fn reserved_char() {
	assert_diagnostics(
		"@^$#~?",
		&[
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(0, 1)),
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(1, 2)),
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(2, 3)),
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(3, 4)),
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(4, 5)),
			Diagnostic::new_spanned(Level::Error, "Reserved character", Span::new(5, 6)),
		],
	)
}

#[test]
fn invalid_char() {
	assert_diagnostics(
		"²³€",
		&[
			Diagnostic::new_spanned(Level::Error, "Invalid character", Span::new(0, 2)),
			Diagnostic::new_spanned(Level::Error, "Invalid character", Span::new(2, 4)),
			Diagnostic::new_spanned(Level::Error, "Invalid character", Span::new(4, 7)),
		],
	)
}

#[test]
fn unterminated() {
	assert_diagnostics(
		"/* *",
		&[Diagnostic::new_spanned(Level::Error, "Unterminated block comment", Span::new(0, 4))
			.sub_diagnostic(Level::Note, None, "Missing trailing `*/` to terminate the block comment")],
	);
	assert_diagnostics(
		"\"Hello World\n",
		&[Diagnostic::new_spanned(Level::Error, "Unterminated string literal", Span::new(0, 13))
			.sub_diagnostic(Level::Note, None, "Missing trailing `\"` to terminate the string literal")],
	);
	assert_diagnostics(
		"'\n'",
		&[
			Diagnostic::new_spanned(Level::Error, "Unterminated character literal", Span::new(0, 2))
				.sub_diagnostic(
					Level::Note,
					None,
					"Missing trailing `'` to terminate the character literal",
				),
			Diagnostic::new_spanned(Level::Error, "Unterminated character literal", Span::new(2, 3))
				.sub_diagnostic(
					Level::Note,
					None,
					"Missing trailing `'` to terminate the character literal",
				),
		],
	);
}

#[test]
fn call() {
	assert_tokens(
		"empty_call()",
		&[
			Token::new(TokenKind::Ident(SmolStr::new("empty_call")), Span::new(0, 10)),
			Token::new(TokenKind::OpenDelim(Delimiter::Paren), Span::new(10, 11)),
			Token::new(TokenKind::CloseDelim(Delimiter::Paren), Span::new(11, 12)),
		],
	);
	assert_tokens(
		"call(arg_1: float, arg_2: bool = true)",
		&[
			Token::new(TokenKind::Ident(SmolStr::new("call")), Span::new(0, 4)),
			Token::new(TokenKind::OpenDelim(Delimiter::Paren), Span::new(4, 5)),
			Token::new(TokenKind::Ident(SmolStr::new("arg_1")), Span::new(5, 10)),
			Token::new(TokenKind::Colon, Span::new(10, 11)),
			Token::new(TokenKind::Ident(SmolStr::new("float")), Span::new(12, 17)),
			Token::new(TokenKind::Comma, Span::new(17, 18)),
			Token::new(TokenKind::Ident(SmolStr::new("arg_2")), Span::new(19, 24)),
			Token::new(TokenKind::Colon, Span::new(24, 25)),
			Token::new(TokenKind::Ident(SmolStr::new("bool")), Span::new(26, 30)),
			Token::new(TokenKind::Eq, Span::new(31, 32)),
			Token::new(TokenKind::Ident(SmolStr::new("true")), Span::new(33, 37)),
			Token::new(TokenKind::CloseDelim(Delimiter::Paren), Span::new(37, 38)),
		],
	);
	assert_token_kinds(
		"call(arg_1: float, arg_2: bool = true)",
		&[
			TokenKind::Ident(SmolStr::new("call")),
			TokenKind::OpenDelim(Delimiter::Paren),
			TokenKind::Ident(SmolStr::new("arg_1")),
			TokenKind::Colon,
			TokenKind::Ident(SmolStr::new("float")),
			TokenKind::Comma,
			TokenKind::Ident(SmolStr::new("arg_2")),
			TokenKind::Colon,
			TokenKind::Ident(SmolStr::new("bool")),
			TokenKind::Eq,
			TokenKind::Ident(SmolStr::new("true")),
			TokenKind::CloseDelim(Delimiter::Paren),
		],
	)
}

#[test]
fn function() {
	assert_token_kinds(
		"fn add(a: int, b: int) -> int { a + b }",
		&[
			TokenKind::Fn,
			TokenKind::Ident(SmolStr::new("add")),
			TokenKind::OpenDelim(Delimiter::Paren),
			TokenKind::Ident(SmolStr::new("a")),
			TokenKind::Colon,
			TokenKind::Ident(SmolStr::new("int")),
			TokenKind::Comma,
			TokenKind::Ident(SmolStr::new("b")),
			TokenKind::Colon,
			TokenKind::Ident(SmolStr::new("int")),
			TokenKind::CloseDelim(Delimiter::Paren),
			TokenKind::ThinArrow,
			TokenKind::Ident(SmolStr::new("int")),
			TokenKind::OpenDelim(Delimiter::Brace),
			TokenKind::Ident(SmolStr::new("a")),
			TokenKind::Plus,
			TokenKind::Ident(SmolStr::new("b")),
			TokenKind::CloseDelim(Delimiter::Brace),
		],
	)
}

#[test]
fn unclosed() {
	// TODO Add multiline variants of tests
	assert_results(
		"({[",
		&[
			Token::new(TokenKind::OpenDelim(Delimiter::Paren), Span::new(0, 1)),
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(1, 2)),
			Token::new(TokenKind::OpenDelim(Delimiter::Bracket), Span::new(2, 3)),
		],
		&[],
	);
	assert_results(
		"{ a + b",
		&[
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(0, 1)),
			Token::new(TokenKind::Ident(SmolStr::new("a")), Span::new(2, 3)),
			Token::new(TokenKind::Plus, Span::new(4, 5)),
			Token::new(TokenKind::Ident(SmolStr::new("b")), Span::new(6, 7)),
		],
		&[],
	);
	assert_results(
		"{ a + (b)",
		&[
			Token::new(TokenKind::OpenDelim(Delimiter::Brace), Span::new(0, 1)),
			Token::new(TokenKind::Ident(SmolStr::new("a")), Span::new(2, 3)),
			Token::new(TokenKind::Plus, Span::new(4, 5)),
			Token::new(TokenKind::OpenDelim(Delimiter::Paren), Span::new(6, 7)),
			Token::new(TokenKind::Ident(SmolStr::new("b")), Span::new(7, 8)),
			Token::new(TokenKind::CloseDelim(Delimiter::Paren), Span::new(8, 9)),
		],
		&[],
	)
}
