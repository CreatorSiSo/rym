#![cfg(test)]

mod expr;
mod functions;
mod modules;
mod variables;

#[macro_export]
macro_rules! insta_assert_parser {
	($parser:expr; $($src:expr),+ $(,)?) => {
		$({
			let (ast, reports) = crate::parse_recovery($parser, $src);
			let snapshot = format!(
				"--- Input ---\n{}\n---\n\n{:#?}\n\n--- Errors ---\n{:#?}\n---",
				$src, &ast, &reports
			);
			insta::assert_snapshot!(&snapshot);
		})*
	};
}
