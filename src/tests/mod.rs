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

#[test]
fn nested() {
	use crate::ast::{Expr, Stmt};
	use crate::parse_str;
	use crate::{ExtraAlias, InputAlias, Spanned};
	use rym_lexer::rich::Token;

	use chumsky::prelude::*;

	fn recover_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone
	{
		let main_s = Token::OpenBrace;
		let main_e = Token::CloseBrace;
		let others = [
			(Token::OpenBracket, Token::CloseBracket),
			(Token::OpenParen, Token::CloseParen),
		];
		let fallback = |span| Spanned(Expr::Block(vec![Stmt::Error]), span);

		// let err_unexpected = Error::<ParserInput<'a>>::expected_found;

		/* custom(move |input_ref| {
			let first_offset = input_ref.offset();
			// SAFETY: first_offset was obtained via input_ref.offset() => is a valid offset of input_ref
			let first_span = unsafe { input_ref.span_since(first_offset) };

			let maybe_first_token = input_ref.next_token();

			match maybe_first_token {
				Some(token) if token == start => (),
				Some(found) => {
					return Err(err_unexpected(
						[Some(MaybeRef::Val(start.clone()))],
						Some(MaybeRef::Val(found)),
						first_span,
					))
				}
				None => {
					return Err(err_unexpected(
						[Some(MaybeRef::Val(start.clone()))],
						None,
						first_span,
					))
				}
			};

			while let Some(token) = input_ref.next_token() {
				if token == end {
					// SAFETY: see first_span
					return Ok(fallback(unsafe { input_ref.span_since(first_offset) }));
				}
			}

			Err(err_unexpected(
				[Some(MaybeRef::Val(end.clone()))],
				None,
				0..0,
			))
		}) */

		// 	let all_delims: Vec<Token> = [main_s.clone(), main_e.clone()]
		// 	.into_iter()
		// 	.chain(others.clone().into_iter().flat_map(|(s, e)| [s, e]))
		// 	.collect();

		// let all_blocks = [(main_s.clone(), main_e.clone())]
		// 	.into_iter()
		// 	// .chain(others.clone())
		// 	.chain(others.clone().into_iter().map(|(_, e)| (main_s.clone(), e)))
		// 	.map(|(s, e)| block.clone().delimited_by(just(s), just(e)).boxed())
		// 	.reduce(|accum, current| accum.or(current).boxed())
		// 	.unwrap();

		// none_of([main_s.clone(), main_e.clone()])
		// 	.repeated()
		// 	.delimited_by(just(main_s), just(main_e))
		// 	.or(all_blocks.or(none_of(all_delims).repeated()))

		// recursive(|block| {}).map_with_span(move |_, span| fallback(span))
		nested_delimiters(main_s, main_e, others, fallback)
	}

	#[track_caller]
	fn assert_output(src: &str, expected: &crate::ParseResult<Spanned<Expr>>) {
		let result = parse_str(|tokens| recover_parser().parse(tokens).into(), src);
		assert_eq!(&result, expected);
	}

	let correct_result =
		|span| crate::ParseResult(Some(Spanned(Expr::Block(vec![Stmt::Error]), span)), vec![]);

	assert_output("{ (test; }", &correct_result(0..10));
	assert_output("{{]}", &correct_result(0..0));
	assert_output("{}", &correct_result(0..2));
	assert_output("{()}", &correct_result(0..4));

	// unrecoverable
	// assert_output("; }", &correct_result(0..0));
}
