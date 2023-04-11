use crate::{expr_parser, insta_assert_parser};
#[allow(unused_imports)]
use chumsky::Parser;
use indoc::indoc;

#[test]
fn literal_expressions() {
	insta_assert_parser! {
		expr_parser();
		"823_472_340",
		"823_472_340_983_273_327",
		"1.0",
		"0.",
		"444.444",
		// TODO This should actually store 9999999999999.999999999 instead of 10000000000000.0
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
			let msg = "Hello World!\n";
			let /* mut */ counter = 0;

			// TODO: count from 0 to 10 and print f"{counter}: Hello World!\n" every time
		}"#),
	}
}

#[test]
fn if_expressions() {
	insta_assert_parser! {
		expr_parser();

		// conditional function call
		r#"if true then { print("Hello Universe!"); } else { print("Hello World!"); }"#,
		r#"if true then print("Hello Universe!") else print("Hello World!")"#,

		// conditional value
		r#"if (true == false) then ("Hello Universe!") else ("Hello World!")"#,
		r#"if true == false then "Hello Universe!" else "Hello World!""#,

		// multiline
		indoc!(r#"
		if true then "Hello Universe!"
		else "Hello World!""#),

		// multiline with block
		indoc!(r#"
		if stuff_goes_right(1, 2, 9) then {
			print("Hello Universe!");
			20;
		} else {
			print("Hello World!");
			10;
		}"#),

		// without else
		r#"if true then print("Hello Universe!")"#,
		r#"if true then { print("Hello Universe!"); }"#,
	}
}

#[test]
fn loop_expressions() {
	insta_assert_parser!(
		expr_parser();

		// infinite
		indoc! {r#"
			{
				loop print("To infinity and beyond!");
				loop (print("To infinity and beyond!"));
				loop { print("To infinity and beyond!"); };
			}"#
		},

		// counter
		indoc! {"
			{
				let /* mut */ counter = 0;

				loop
					if counter > 10 then
						break
					else
						counter = counter + 1;

				loop (
					if (counter > 10) then (break) else (counter = counter + 1)
				);

				loop {
					if counter > 10 then {
						break;
					} else {
						counter = counter + 1;
					};
				};
			}"
		},
	);
}

#[test]
fn recover_group() {
	insta_assert_parser!(
		expr_parser();

		// invalid semicolon
		"(testing;)",

		// missing rhs
		"(1 +)",
		"( / 888)",

		// TODO start testing this once nested_delimiters works again
		// unclosed group
		// "((testing)",
		// "(testing}",
	);
}

#[test]
fn recover_block() {
	insta_assert_parser!(
		expr_parser();

		// gibberish
		"{ /&$/&/$ }",

		// unclosed
		"{ (testing; }",
		// TODO: improve error recovery so that it keeps at least one block
		"{ {testing; }",


		// additional closing delimiter
		"{ testing ]}",
		"{ testing )}",

		// TODO start testing this once nested_delimiters works again
		// wrong closing delimiter
		// "{ testing ]",
		// "{ testing )",
	);
}

#[test]
fn recover_call() {
	insta_assert_parser!(
		expr_parser();

		// missing args
		"testing(,)",
		"testing(,)(correct_one, 2)(, aerr, \"stuff\")",

		// missing commas
		"testing(1 2 3)",
		"testing(1 + / 2)",

		// unclosed
		"testing(1, {2, 3)",
		// "testing(1, (2, 3)",
	);
}

#[test]
fn recover_if() {
	insta_assert_parser!(
		expr_parser();

		// missing args
		"if testing(,) then _",

		// unclosed
		"if (1} then _",
		"if (1] then _",
		"if {1;) then _",
		"if testing(1, {2, 3) then _",
		"if testing(1, (2, 3) then _",
	);
}
