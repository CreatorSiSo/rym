use crate::{expr_parser, insta_assert_parser, item_parser};
use indoc::indoc;

#[test]
fn empty() {
	insta_assert_parser! {
		item_parser(expr_parser());

		"mod name {}",

		// TODO: Should still parse (name = None)
		"mod {}",

		// TODO: Should still parse (body = vec![])
		"mod name",

		// TODO: Should still parse (name = None, body = vec![])
		"mod",
	}
}

#[test]
fn mixed() {
	insta_assert_parser! {
		item_parser(expr_parser());

		indoc! {r#"
		mod name {
			func testing() {}
			const this = out
		}"#
		},
	}
}
