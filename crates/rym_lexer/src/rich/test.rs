#![cfg(test)]

use rym_errors::{Diagnostic, Level};
use rym_span::Span;

use super::{token::LexerError, Token};

#[track_caller]
fn assert_results(src: &str, expect: &[(Token, Span)], errors: &[LexerError]) {
	// TODO Make tests work or add insta snapshot testing
	// let got: Vec<_> = super::Lexer::new(src).collect();
	// println!("{got:#?}");
	// assert_eq!(expect, got);
	// assert_eq!(errors, &handler.collect());
}

#[track_caller]
fn assert_tokens(src: &str, expect: &[(Token, Span)]) {
	// let handler = DiagnosticHandler::default();
	// let got: Vec<Token> = super::Lexer::new(src, &handler).collect();
	// let errors = handler.collect();
	// assert_eq!(errors, vec![], "Expected no errors got: {errors:?}");
	// println!("{got:#?}");
	// assert_eq!(got, expect);
}

#[track_caller]
fn assert_diagnostics(src: &str, expect: &[Diagnostic]) {
	// let handler = DiagnosticHandler::default();
	// let _: Vec<_> = super::Lexer::new(src, &handler).collect();
	// let errors = handler.collect();
	// println!("{errors:#?}",);
	// assert_eq!(errors, expect)
}

#[track_caller]
fn assert_token_kinds(src: &str, expect: &[Token]) {
	// let handler = DiagnosticHandler::default();
	// let got: Vec<TokenKind> = super::Lexer::new(src, &handler).map(|token| token.kind).collect();
	// let errors = handler.collect();
	// assert_eq!(errors, vec![], "Expected no errors got: {errors:?}");
	// println!("{got:#?}");
	// assert_eq!(got, expect);
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
			(Token::If, Span::new(0, 2)),
			(Token::Ident("test".to_string()), Span::new(3, 7)),
			(Token::EqEq, Span::new(8, 10)),
			(Token::Ident("true".to_string()), Span::new(11, 15)),
			(Token::OpenBrace, Span::new(16, 17)),
			(Token::String("is correct".into()), Span::new(18, 30)),
			(Token::CloseBrace, Span::new(31, 32)),
			(Token::Else, Span::new(33, 37)),
			(Token::OpenBrace, Span::new(38, 39)),
			(Token::String("is not correct".into()), Span::new(40, 56)),
			(Token::CloseBrace, Span::new(57, 58)),
			(Token::Semi, Span::new(58, 59)),
		],
	);
	assert_token_kinds(
		src,
		&[
			Token::If,
			Token::Ident("test".to_string()),
			Token::EqEq,
			Token::Ident("true".to_string()),
			Token::OpenBrace,
			Token::String("is correct".into()),
			Token::CloseBrace,
			Token::Else,
			Token::OpenBrace,
			Token::String("is not correct".into()),
			Token::CloseBrace,
			Token::Semi,
		],
	);
}

#[test]
fn literals() {
	assert_token_kinds(
		r#"999_999_999 1.284_324_68 'c' '\n' '\x41' '\u24B6' '\u8DEF' "Hello World!\n""#,
		&[
			Token::Int(999_999_999),
			Token::Float("1.284_324_68".into()),
			Token::Char('c'),
			Token::Char('\n'),
			Token::Char('A'),
			Token::Char('Ⓐ'),
			Token::Char('路'),
			Token::String("Hello World!\n".into()),
		],
	)
}

#[test]
fn path() {
	assert_token_kinds(
		r#"std::__test::me: ::global_type"#,
		&[
			Token::Ident("std".to_string()),
			Token::ColonColon,
			Token::Ident("__test".to_string()),
			Token::ColonColon,
			Token::Ident("me".to_string()),
			Token::Colon,
			Token::ColonColon,
			Token::Ident("global_type".to_string()),
		],
	)
}

#[test]
fn operators() {
	assert_token_kinds(
		r#"| || & && + += - -= * *= / /= % %= = == ! != < <= > >="#,
		&[
			Token::Pipe,
			Token::PipePipe,
			Token::And,
			Token::AndAnd,
			Token::Plus,
			Token::PlusEq,
			Token::Minus,
			Token::MinusEq,
			Token::Star,
			Token::StarEq,
			Token::Slash,
			Token::SlashEq,
			Token::Percent,
			Token::PercentEq,
			Token::Eq,
			Token::EqEq,
			Token::Bang,
			Token::BangEq,
			Token::LessThan,
			Token::LessThanEq,
			Token::GreaterThan,
			Token::GreaterThanEq,
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
		&[
			Diagnostic::new_spanned(Level::Error, "Unterminated block comment", Span::new(0, 4))
				.sub_diagnostic(
					Level::Note,
					None,
					"Missing trailing `*/` to terminate the block comment",
				),
		],
	);
	assert_diagnostics(
		"\"Hello World\n",
		&[Diagnostic::new_spanned(
			Level::Error,
			"Unterminated string literal",
			Span::new(0, 13),
		)
		.sub_diagnostic(
			Level::Note,
			None,
			"Missing trailing `\"` to terminate the string literal",
		)],
	);
	assert_diagnostics(
		"'\n'",
		&[
			Diagnostic::new_spanned(
				Level::Error,
				"Unterminated character literal",
				Span::new(0, 2),
			)
			.sub_diagnostic(
				Level::Note,
				None,
				"Missing trailing `'` to terminate the character literal",
			),
			Diagnostic::new_spanned(
				Level::Error,
				"Unterminated character literal",
				Span::new(2, 3),
			)
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
			(Token::Ident("empty_call".to_string()), Span::new(0, 10)),
			(Token::OpenParen, Span::new(10, 11)),
			(Token::CloseParen, Span::new(11, 12)),
		],
	);
	assert_tokens(
		"call(arg_1: float, arg_2: bool = true)",
		&[
			(Token::Ident("call".to_string()), Span::new(0, 4)),
			(Token::OpenParen, Span::new(4, 5)),
			(Token::Ident("arg_1".to_string()), Span::new(5, 10)),
			(Token::Colon, Span::new(10, 11)),
			(Token::Ident("float".to_string()), Span::new(12, 17)),
			(Token::Comma, Span::new(17, 18)),
			(Token::Ident("arg_2".to_string()), Span::new(19, 24)),
			(Token::Colon, Span::new(24, 25)),
			(Token::Ident("bool".to_string()), Span::new(26, 30)),
			(Token::Eq, Span::new(31, 32)),
			(Token::Ident("true".to_string()), Span::new(33, 37)),
			(Token::CloseParen, Span::new(37, 38)),
		],
	);
	assert_token_kinds(
		"call(arg_1: float, arg_2: bool = true)",
		&[
			Token::Ident("call".to_string()),
			Token::OpenParen,
			Token::Ident("arg_1".to_string()),
			Token::Colon,
			Token::Ident("float".to_string()),
			Token::Comma,
			Token::Ident("arg_2".to_string()),
			Token::Colon,
			Token::Ident("bool".to_string()),
			Token::Eq,
			Token::Ident("true".to_string()),
			Token::CloseParen,
		],
	)
}

#[test]
fn function() {
	assert_token_kinds(
		"fn add(a: int, b: int) -> int { a + b }",
		&[
			Token::Func,
			Token::Ident("add".to_string()),
			Token::OpenParen,
			Token::Ident("a".to_string()),
			Token::Colon,
			Token::Ident("int".to_string()),
			Token::Comma,
			Token::Ident("b".to_string()),
			Token::Colon,
			Token::Ident("int".to_string()),
			Token::CloseParen,
			Token::ThinArrow,
			Token::Ident("int".to_string()),
			Token::OpenBrace,
			Token::Ident("a".to_string()),
			Token::Plus,
			Token::Ident("b".to_string()),
			Token::CloseBrace,
		],
	)
}

#[test]
fn unclosed() {
	// TODO Add multiline variants of tests
	assert_results(
		"({[",
		&[
			(Token::OpenParen, Span::new(0, 1)),
			(Token::OpenBrace, Span::new(1, 2)),
			(Token::OpenBracket, Span::new(2, 3)),
		],
		&[],
	);
	assert_results(
		"{ a + b",
		&[
			(Token::OpenBrace, Span::new(0, 1)),
			(Token::Ident("a".to_string()), Span::new(2, 3)),
			(Token::Plus, Span::new(4, 5)),
			(Token::Ident("b".to_string()), Span::new(6, 7)),
		],
		&[],
	);
	assert_results(
		"{ a + (b)",
		&[
			(Token::OpenBrace, Span::new(0, 1)),
			(Token::Ident("a".to_string()), Span::new(2, 3)),
			(Token::Plus, Span::new(4, 5)),
			(Token::OpenParen, Span::new(6, 7)),
			(Token::Ident("b".to_string()), Span::new(7, 8)),
			(Token::CloseParen, Span::new(8, 9)),
		],
		&[],
	)
}
