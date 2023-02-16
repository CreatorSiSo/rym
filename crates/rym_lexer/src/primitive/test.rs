#![cfg(test)]

use super::{TokenKind::*, *};

fn assert_tokens(input: &str, expect: &[Token]) {
	let tokens: Vec<Token> = lex(input).collect();
	assert_eq!(&tokens, expect);
}

#[test]
fn empty() {
	assert_tokens("", &[])
}

#[test]
fn line_comment() {
	assert_tokens("// ² $ line @ comment", &[Token::new(LineComment, 22)]);
}

#[test]
fn block_comment() {
	assert_tokens(
		r#"/* € testing */ /*
			sdasd³
			/* 832³7 */
			testing
			*/"#,
		&[
			Token::new(BlockComment { terminated: true }, 17),
			Token::new(Whitespace, 1),
			Token::new(BlockComment { terminated: true }, 46),
		],
	);
	assert_tokens(
		r#"/* testing *_ /*
			sdasd
			/* 8327 */
			testing
			*/"#,
		&[Token::new(BlockComment { terminated: false }, 56)],
	)
}

#[test]
fn ident() {
	assert_tokens(
		"π _tst__ing stµff",
		&[
			Token::new(Ident, 2),
			Token::new(Whitespace, 1),
			Token::new(Ident, 9),
			Token::new(Whitespace, 1),
			Token::new(Ident, 6),
		],
	)
}

#[test]
fn line_end() {
	assert_tokens("\n", &[Token::new(Whitespace, 1)]);
	assert_tokens("\r\n", &[Token::new(Whitespace, 2)]);
	assert_tokens(";\n", &[Token::new(Semi, 1), Token::new(Whitespace, 1)])
}

#[test]
fn one_char() {
	assert_tokens(
		"\n \t \r;:,. |&+-*/%=! ~?@^$# <>(){}[]",
		&[
			// Whitespace
			Token::new(Whitespace, 5),
			// Punctuation
			Token::new(Semi, 1),
			Token::new(Colon, 1),
			Token::new(Comma, 1),
			Token::new(Dot, 1),
			Token::new(Whitespace, 1),
			// Used
			Token::new(Or, 1),
			Token::new(And, 1),
			Token::new(Plus, 1),
			Token::new(Minus, 1),
			Token::new(Star, 1),
			Token::new(Slash, 1),
			Token::new(Percent, 1),
			Token::new(Eq, 1),
			Token::new(Bang, 1),
			Token::new(Whitespace, 1),
			// Unused
			Token::new(Tilde, 1),
			Token::new(Question, 1),
			Token::new(At, 1),
			Token::new(Caret, 1),
			Token::new(Dollar, 1),
			Token::new(Pound, 1),
			Token::new(Whitespace, 1),
			// Delimiter
			Token::new(LessThan, 1),
			Token::new(GreaterThan, 1),
			Token::new(OpenParen, 1),
			Token::new(CloseParen, 1),
			Token::new(OpenBrace, 1),
			Token::new(CloseBrace, 1),
			Token::new(OpenBracket, 1),
			Token::new(CloseBracket, 1),
		],
	)
}

#[test]
fn integer() {
	assert_tokens(
		"0 1 2 42739387324 0000234236932 999_999_999_999",
		&[
			Token::new(Integer, 1),
			Token::new(Whitespace, 1),
			Token::new(Integer, 1),
			Token::new(Whitespace, 1),
			Token::new(Integer, 1),
			Token::new(Whitespace, 1),
			Token::new(Integer, 11),
			Token::new(Whitespace, 1),
			Token::new(Integer, 13),
			Token::new(Whitespace, 1),
			Token::new(Integer, 15),
		],
	)
}

#[test]
fn float() {
	assert_tokens(
		"0. 123. 2.222 4273.9387324 0000.234236932 999_999_999.999",
		&[
			Token::new(Float, 2),
			Token::new(Whitespace, 1),
			Token::new(Float, 4),
			Token::new(Whitespace, 1),
			Token::new(Float, 5),
			Token::new(Whitespace, 1),
			Token::new(Float, 12),
			Token::new(Whitespace, 1),
			Token::new(Float, 14),
			Token::new(Whitespace, 1),
			Token::new(Float, 15),
		],
	)
}

#[test]
fn string() {
	assert_tokens(
		r#"
				""
				"test"
				"
					Hello
					World!
				"
				"\n@²³§½ÄÖÜ\\"
				"\""
			"#,
		&[
			Token::new(Whitespace, 5),
			Token::new(String { terminated: true }, 2),
			Token::new(Whitespace, 5),
			Token::new(String { terminated: true }, 6),
			Token::new(Whitespace, 5),
			Token::new(String { terminated: true }, 30),
			Token::new(Whitespace, 5),
			Token::new(String { terminated: true }, 21),
			Token::new(Whitespace, 5),
			Token::new(String { terminated: true }, 4),
			Token::new(Whitespace, 4),
		],
	);
	assert_tokens(
		r#" "\\" "\" "#,
		&[
			Token::new(Whitespace, 1),
			Token::new(String { terminated: true }, 4),
			Token::new(Whitespace, 1),
			Token::new(String { terminated: false }, 4),
		],
	)
}

#[test]
fn char() {
	assert_tokens(
		r#"
				''
				't'
				'\n@²³§½ÄÖÜ\\'
				'"'
				'\''
			"#,
		&[
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: true }, 2),
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: true }, 3),
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: true }, 21),
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: true }, 3),
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: true }, 4),
			Token::new(Whitespace, 4),
		],
	);
	assert_tokens(
		r#"
				'
				'\\'
				'\'
			"#,
		&[
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: false }, 2),
			Token::new(Whitespace, 4),
			Token::new(Char { terminated: true }, 4),
			Token::new(Whitespace, 5),
			Token::new(Char { terminated: false }, 4),
			Token::new(Whitespace, 3),
		],
	)
}
