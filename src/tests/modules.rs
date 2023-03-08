use crate::{expr_parser, insta_assert_parser, item_parser, module_file_parser};
#[allow(unused_imports)]
use chumsky::Parser;
use indoc::indoc;

#[test]
fn empty() {
	insta_assert_parser! {
	item_parser(expr_parser());

	"mod name {}",
	"mod {}",

	"mod name",
	"mod name;",

	"mod",
	"mod;",
	}
}

#[test]
fn mixed() {
	insta_assert_parser! {
		item_parser(expr_parser());

		indoc! {r#"
		mod name {
			func testing() {}
			let this = testing();
		}"#
		},
	}
}

#[test]
fn file() {
	insta_assert_parser!(
		module_file_parser();

		indoc!(r#"
		/// useless function for adding two values together
		func add(x, y) {
			x + y; // here we add them
		}

		func multiply(x, y) {
			// here we are multiplying x by y
			x * y;
		}

		func main() {
			print("Hello World!");
			add(2, 3);
			multiply(3, 2);
		}"#)
	);
}

#[test]
fn recover_block() {
	insta_assert_parser! {
		item_parser(expr_parser());

		"mod out { let in_1 = 0; }",
		"mod out { let in_1 = 0; ]",

		"mod out { let in_1 = 0;   mod test   let in_1 = 0; }",
		"mod out { mod let in_1 = 0; }",

		"mod out { mod lvl_1 { mod lvl_2 { let in_3 = 0; }   mod lvl_2   let in_2 = 0; } }",
		"mod out { mod lvl_1 { mod lvl_2 { let in_3 = 0; ]   mod lvl_2   let in_2 = 0; } }", // only outer module is recovered
		"mod out { mod lvl_1 { mod lvl_2 { let in_3 = 0; )   mod lvl_2   let in_2 = 0; } }", // only outer module is recovered

		"mod name { mod name mod name {] }",
		"mod name { mod name; mod name {] }",
	}
}
