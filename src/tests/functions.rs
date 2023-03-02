use crate::{expr_parser, insta_assert_parser, item_parser};

#[test]
fn empty() {
	insta_assert_parser! {
		item_parser(expr_parser());

		"func empty() {}",
		"func empty();",
		// "func empty() = Unit",
	}
}

#[test]
fn params() {
	insta_assert_parser! {
		item_parser(expr_parser());

		"func params(one, two, three);",
		"func params(self, other);",
		// "func add(mut self, other);",
		// "func combine((l_start, l_end): (Usize, Usize), (r_start, r_end): (Usize, Usize));",
		// "func combine(Record { inner }: Record, .{ inner }: Record);",
	}
}
