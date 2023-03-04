#![cfg(test)]

mod expr;
mod functions;
mod modules;

#[macro_export]
macro_rules! insta_assert_parser {
	($parser:expr; $($src:expr),+ $(,)?) => {
		use chumsky::Parser;
		use crate::parse_str;
		$({
			let result = parse_str(|tokens| $parser.parse(tokens).into(), $src);
			let snapshot = format!(
				"--- Input ---\n{}\n---\n\n{:#?}\n\n--- Errors ---\n{:#?}\n---",
				$src, result.0, result.1
			);
			insta::assert_snapshot!(&snapshot);
		})*
	};
}
