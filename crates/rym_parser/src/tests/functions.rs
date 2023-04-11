use crate::{insta_assert_parser, parse_item};
#[allow(unused_imports)]
use chumsky::Parser;

#[test]
fn empty() {
	insta_assert_parser! {
		parse_item;

		"func empty() {}",
		"func empty();",
		// "func empty() = Unit",
	}
}

#[test]
fn params() {
	insta_assert_parser! {
		parse_item;

		"func params(one, two, three);",
		"func params(self, other);",
		// "func add(mut self, other);",
		// "func combine((l_start, l_end): (Usize, Usize), (r_start, r_end): (Usize, Usize));",
		// "func combine(Record { inner }: Record, .{ inner }: Record);",
	}
}
