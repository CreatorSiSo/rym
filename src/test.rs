#![cfg(test)]

use super::*;
use chumsky::Stream;
use rym_lexer::rich::Lexer;

macro_rules! insta_assert_parser {
	($parser:expr; $($src:expr),+ $(,)?) => {
		$({
			let token_stream = Stream::from_iter(0..0, Lexer::new($src));
			let maybe_items = $parser.parse(token_stream);
			insta::assert_debug_snapshot!(maybe_items);
		})*
	};
}

#[test]
fn parse_item() {
	insta_assert_parser!(
		item_parser();
		"func testing(param_0, param_1, param_n,)\n",
		"func testing();\n",
	);
}
