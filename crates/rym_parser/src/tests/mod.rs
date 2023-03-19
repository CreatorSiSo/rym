#![cfg(test)]

mod examples;
mod expr;
mod functions;
mod modules;

use crate::ast::Expr;
use crate::{custom_nested_delimiters, parse_str, Spanned};
use rym_lexer::rich::Token;

use chumsky::prelude::*;
use indoc::indoc;

#[macro_export]
macro_rules! insta_assert_parser {
	($parser:expr; $($src:expr),+ $(,)?) => {
		use chumsky::Parser;
		use crate::{parse_str, ParseResult};
		$({
			let ParseResult { ast, errors } = parse_str(|tokens| $parser.parse(tokens).into(), $src);
			let snapshot = format!(
				"--- Input ---\n{}\n---\n\n{:#?}\n\n--- Errors ---\n{:#?}\n---",
				$src, ast, errors
			);
			insta::assert_snapshot!(&snapshot);
		})*
	};
}

#[test]
fn nested() {
	#[track_caller]
	fn assert_output(src: &str) {
		let result = parse_str(
			|tokens| {
				custom_nested_delimiters(
					Token::OpenBrace,
					Token::CloseBrace,
					[(Token::OpenBracket, Token::CloseBracket), (Token::OpenParen, Token::CloseParen)],
					|span| Spanned(Expr::Block(vec![]), span),
				)
				.then_ignore(any().repeated())
				.parse(tokens)
				.into()
			},
			src,
		);
		insta::assert_snapshot!(format!("---\n{src}\n---\n\n{result:?}"));
	}

	assert_output("{}                              ()");
	assert_output("{()}                            ()");
	assert_output("{()[]({}){}()()}                ()");

	assert_output("{ let testing = 23 + 0; }       ()");
	assert_output("{ 200 + (66 - 9) * 33 }         ()");
	assert_output(indoc! {"{
		fn(a, b) {
			get_fn[](arg0, { 0 });
			{ inside }(arg1)(arg2);
		}
	}                                              ()"});

	// TODO start testing this once nested_delimiters works again
	// assert_output("{{]}                            ()");
	// assert_output("{ (test; }                      ()");
}
