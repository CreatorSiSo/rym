#![cfg(test)]

use super::*;
use chumsky::Stream;
use indoc::indoc;
use rym_lexer::rich::Lexer;

macro_rules! insta_assert_parser {
	($parser:expr; $($src:expr),+ $(,)?) => {
		$({
			let token_stream = Stream::from_iter(0..0, Lexer::new($src));
			let maybe_items = $parser.parse(token_stream);

			let snapshot = format!("---\n{}\n---\n\n{:#?}", $src, &maybe_items);
			insta::assert_snapshot!(&snapshot);
		})*
	};
}

#[test]
fn literal_expressions() {
	insta_assert_parser! {
		expr_parser();
		"823_472_340",
		"823_472_340_983_273_327",
		"1.0",
		"0.",
		"444.444",
		"9_999_999_999_999.999_999_999",
		"'a'",
		"'\\n'",
		"'\\t'",
		"'\\r'",
		r#" "abc" "#,
		r#" "Hello World!\n" "#,
	}
}

#[test]
fn record_expressions() {
	insta_assert_parser! {
		expr_parser();
		r#"Record { name: "Record", fields: todo() }"#,
		r#".{ name: "Record", fields: todo() }"#,
		r#".{ }"#,
	}
}

#[test]
fn simple_expressions() {
	insta_assert_parser! {
		expr_parser();
		"indentifier_1",
		"(grouped_ident)",
		"func_name(2, \"Do stuff!!\", true)",
		"(make_func())()()()",
		"(((wrapped)))()",
		"(if true then 1 else 2) + 3",
	}
}

#[test]
fn unary_expressions() {
	insta_assert_parser! {
		expr_parser();
		"-255",
		"-1.0",
		"-0.",
		"---3",

		"!true",
		"!false",
		"!!!true",
		"!(lhs == rhs)",
		"!lhs == rhs",
	}
}

#[test]
fn binary_expressions() {
	insta_assert_parser! {
		expr_parser();
		"lhs == rhs",
		"lhs != rhs",
		"lhs + rhs",
		"lhs - rhs",
		"lhs * rhs",
		"lhs / rhs",


		"1 + 2 - 3 * 4 / 5",
		"1 == 1 + 2 / 3 != 0",
	}
}

#[test]
fn block_expressions() {
	insta_assert_parser! {
		expr_parser();
		indoc!("
		{
			testing;
			testing;
		}"),
		"{ call_me(); and_me(); }",
		"{ }",
	}
}

#[test]
fn variables() {
	insta_assert_parser! {
		expr_parser();
		indoc!(r#"
		{
			const msg = "Hello World!\n";
			mut counter = 0;

			// TODO: count from 0 to 10 and print f"{counter}: Hello World!\n" every time
		}"#),
	}
}

#[test]
fn if_expressions() {
	insta_assert_parser! {
		expr_parser();
		r#"if true then { print("Hello Universe!"); } else { print("Hello World!"); }"#,
		r#"if (true == false) then ("Hello Universe!") else ("Hello World!")"#,
		r#"if true then "Hello Universe!" else "Hello World!""#,
		indoc!(r#"
			if true then "Hello Universe!"
			else "Hello World!""#),
		indoc!(r#"
			if stuff_goes_right(1, 2, 9) then {
				print("Hello Universe!");
				20;
			} else {
				print("Hello World!");
				10;
			}"#),
	}
}
