#![cfg(test)]

mod examples;
mod expr;
mod functions;
mod modules;

#[macro_export]
macro_rules! insta_assert_parser {
	($parse_fn:expr; $($src:expr),+ $(,)?) => {
		use crate::{ParseResult};
		use rym_lexer::rich::Lexer;
		$({
			let tokens: Vec<_> = Lexer::new($src).collect();
			let ParseResult { ast, errors } = $parse_fn(&tokens);
			let snapshot = format!(
				"--- Input ---\n{}\n---\n\n{:#?}\n\n--- Errors ---\n{:#?}\n---",
				$src, ast, errors
			);
			insta::assert_snapshot!(&snapshot);
		})*
	};
}

// #[test]
// fn nested() {
// 	#[track_caller]
// 	fn assert_output(src: &str) {
// 		let result = parse_str(
// 			|tokens| {
// 				custom_nested_delimiters(
// 					Token::OpenBrace,
// 					Token::CloseBrace,
// 					[
// 						(Token::OpenBracket, Token::CloseBracket),
// 						(Token::OpenParen, Token::CloseParen),
// 					],
// 					|span| Spanned(Expr::Block(Spanned(vec![], span.clone())), span),
// 				)
// 				.then_ignore(any().repeated())
// 				.parse(tokens)
// 				.into()
// 			},
// 			src,
// 		);
// 		insta::assert_snapshot!(format!("---\n{src}\n---\n\n{result:?}"));
// 	}

// 	assert_output("{}                              ()");
// 	assert_output("{()}                            ()");
// 	assert_output("{()[]({}){}()()}                ()");

// 	assert_output("{ let testing = 23 + 0; }       ()");
// 	assert_output("{ 200 + (66 - 9) * 33 }         ()");
// 	assert_output(indoc! {"{
// 		fn(a, b) {
// 			get_fn[](arg0, { 0 });
// 			{ inside }(arg1)(arg2);
// 		}
// 	}                                              ()"});

// 	// TODO start testing this once nested_delimiters works again
// 	// assert_output("{{]}                            ()");
// 	// assert_output("{ (test; }                      ()");
// }
