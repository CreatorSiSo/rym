#![cfg(test)]

use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use rym_tt::{Delimiter, LitKind};
use smol_str::SmolStr;

use super::linear::{LinearLexer, LinearToken, LinearTokenKind};

#[track_caller]
fn assert_results(src: &str, expect: &[LinearToken], diagnostics: &[Diagnostic]) {
	let handler = Handler::default();
	let got: Vec<_> = LinearLexer::new(src, &handler).collect();
	println!("{got:#?}");
	assert_eq!(expect, got);
	assert_eq!(diagnostics, &handler.collect());
}

#[track_caller]
fn assert_tokens(src: &str, expect: &[LinearToken]) {
	let handler = Handler::default();
	let got: Vec<LinearToken> = LinearLexer::new(src, &handler).collect();
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
fn assert_token_kinds(src: &str, expect: &[LinearTokenKind]) {
	let handler = Handler::default();
	let got: Vec<LinearTokenKind> = LinearLexer::new(src, &handler).map(|token| token.kind).collect();
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
			LinearToken::new(LinearTokenKind::Ident(SmolStr::new("if")), Span::new(0, 2)),
			LinearToken::new(LinearTokenKind::Ident(SmolStr::new("test")), Span::new(3, 7)),
			LinearToken::new(LinearTokenKind::EqEq, Span::new(8, 10)),
			LinearToken::new(LinearTokenKind::Ident(SmolStr::new("true")), Span::new(11, 15)),
			LinearToken::new(LinearTokenKind::OpenDelim(Delimiter::Brace), Span::new(16, 17)),
			LinearToken::new(
				LinearTokenKind::Literal(LitKind::String("is correct".into())),
				Span::new(18, 30),
			),
			LinearToken::new(LinearTokenKind::CloseDelim(Delimiter::Brace), Span::new(31, 32)),
			LinearToken::new(LinearTokenKind::Ident(SmolStr::new("else")), Span::new(33, 37)),
			LinearToken::new(LinearTokenKind::OpenDelim(Delimiter::Brace), Span::new(38, 39)),
			LinearToken::new(
				LinearTokenKind::Literal(LitKind::String("is not correct".into())),
				Span::new(40, 56),
			),
			LinearToken::new(LinearTokenKind::CloseDelim(Delimiter::Brace), Span::new(57, 58)),
			LinearToken::new(LinearTokenKind::Semi, Span::new(58, 59)),
		],
	);
	assert_token_kinds(
		src,
		&[
			LinearTokenKind::Ident(SmolStr::new("if")),
			LinearTokenKind::Ident(SmolStr::new("test")),
			LinearTokenKind::EqEq,
			LinearTokenKind::Ident(SmolStr::new("true")),
			LinearTokenKind::OpenDelim(Delimiter::Brace),
			LinearTokenKind::Literal(LitKind::String("is correct".into())),
			LinearTokenKind::CloseDelim(Delimiter::Brace),
			LinearTokenKind::Ident(SmolStr::new("else")),
			LinearTokenKind::OpenDelim(Delimiter::Brace),
			LinearTokenKind::Literal(LitKind::String("is not correct".into())),
			LinearTokenKind::CloseDelim(Delimiter::Brace),
			LinearTokenKind::Semi,
		],
	);
}

#[test]
fn literals() {
	assert_token_kinds(
		r#"999_999_999 1.284_324_68 'c' '\n' '\x41' '\u24B6' '\u8DEF' "Hello World!\n""#,
		&[
			LinearTokenKind::Literal(LitKind::Integer(999_999_999)),
			LinearTokenKind::Literal(LitKind::Float(1.284_324_68)),
			LinearTokenKind::Literal(LitKind::Char('c')),
			LinearTokenKind::Literal(LitKind::Char('\n')),
			LinearTokenKind::Literal(LitKind::Char('A')),
			LinearTokenKind::Literal(LitKind::Char('Ⓐ')),
			LinearTokenKind::Literal(LitKind::Char('路')),
			LinearTokenKind::Literal(LitKind::String("Hello World!\n".into())),
		],
	)
}

#[test]
fn path() {
	assert_token_kinds(
		r#"std::__test::me: ::global_type"#,
		&[
			LinearTokenKind::Ident(SmolStr::new("std")),
			LinearTokenKind::ColonColon,
			LinearTokenKind::Ident(SmolStr::new("__test")),
			LinearTokenKind::ColonColon,
			LinearTokenKind::Ident(SmolStr::new("me")),
			LinearTokenKind::Colon,
			LinearTokenKind::ColonColon,
			LinearTokenKind::Ident(SmolStr::new("global_type")),
		],
	)
}

#[test]
fn operators() {
	assert_token_kinds(
		r#"| || & && + += - -= * *= / /= % %= = == ! != < <= > >="#,
		&[
			LinearTokenKind::Or,
			LinearTokenKind::OrOr,
			LinearTokenKind::And,
			LinearTokenKind::AndAnd,
			LinearTokenKind::Plus,
			LinearTokenKind::PlusEq,
			LinearTokenKind::Minus,
			LinearTokenKind::MinusEq,
			LinearTokenKind::Star,
			LinearTokenKind::StarEq,
			LinearTokenKind::Slash,
			LinearTokenKind::SlashEq,
			LinearTokenKind::Percent,
			LinearTokenKind::PercentEq,
			LinearTokenKind::Eq,
			LinearTokenKind::EqEq,
			LinearTokenKind::Bang,
			LinearTokenKind::BangEq,
			LinearTokenKind::LessThan,
			LinearTokenKind::LessThanEq,
			LinearTokenKind::GreaterThan,
			LinearTokenKind::GreaterThanEq,
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
