mod cursor;
mod token;
pub use cursor::Cursor;
pub use token::{PrimitiveLitKind, PrimitiveToken, PrimitiveTokenKind};

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = PrimitiveToken> + '_ {
	let mut cursor = Cursor::new(input);
	std::iter::from_fn(move || cursor.next_token())
}

impl Cursor<'_> {
	pub fn next_token(&mut self) -> Option<PrimitiveToken> {
		let first_char = match self.bump() {
			Some(char) => char,
			None => return None,
		};

		let token_kind = match first_char {
			// Slash, line or block comment.
			'/' => match self.peek() {
				'/' => {
					self.eat_while(|c| c != '\n');
					PrimitiveTokenKind::LineComment
				}
				'*' => self.eat_block_comment(),
				_ => PrimitiveTokenKind::Slash,
			},

			c if is_whitespace(c) => {
				self.eat_while(|c| c.is_whitespace());
				PrimitiveTokenKind::Whitespace
			}

			// Number literal.
			'0'..='9' => {
				let mut is_int = true;
				self.eat_while(|c| {
					if c == '.' {
						is_int = false;
					};
					c.is_numeric() || matches!(c, '_' | '.')
				});
				PrimitiveTokenKind::Literal {
					kind: if is_int { PrimitiveLitKind::Integer } else { PrimitiveLitKind::Float },
				}
			}

			// String literal.
			'"' => PrimitiveTokenKind::Literal {
				kind: PrimitiveLitKind::String { terminated: self.eat_string_literal() },
			},

			// Character literal.
			'\'' => PrimitiveTokenKind::Literal {
				kind: PrimitiveLitKind::Char { terminated: self.eat_char_literal() },
			},

			// Indetifier token.
			c if is_ident_start(c) => {
				self.eat_while(is_ident_continue);
				PrimitiveTokenKind::Ident
			}

			// Punctuation tokens.
			';' => PrimitiveTokenKind::Semi,
			':' => PrimitiveTokenKind::Colon,
			',' => PrimitiveTokenKind::Comma,
			'.' => PrimitiveTokenKind::Dot,

			// One character tokens.
			'|' => PrimitiveTokenKind::Or,
			'&' => PrimitiveTokenKind::And,
			'+' => PrimitiveTokenKind::Plus,
			'-' => PrimitiveTokenKind::Minus,
			'*' => PrimitiveTokenKind::Star,
			'%' => PrimitiveTokenKind::Percent,
			'=' => PrimitiveTokenKind::Eq,
			'!' => PrimitiveTokenKind::Bang,

			// Currently unused one character tokens.
			'~' => PrimitiveTokenKind::Tilde,
			'?' => PrimitiveTokenKind::Question,
			'@' => PrimitiveTokenKind::At,
			'^' => PrimitiveTokenKind::Caret,
			'$' => PrimitiveTokenKind::Dollar,
			'#' => PrimitiveTokenKind::Pound,

			// Delimiter like tokens.
			'<' => PrimitiveTokenKind::LessThan,
			'>' => PrimitiveTokenKind::GreaterThan,
			'(' => PrimitiveTokenKind::OpenParen,
			')' => PrimitiveTokenKind::CloseParen,
			'{' => PrimitiveTokenKind::OpenBrace,
			'}' => PrimitiveTokenKind::CloseBrace,
			'[' => PrimitiveTokenKind::OpenBracket,
			']' => PrimitiveTokenKind::CloseBracket,

			_ => PrimitiveTokenKind::Unkown,
		};

		let result = PrimitiveToken::new(token_kind, self.len_consumed());
		self.reset_len_consumed();
		Some(result)
	}

	fn eat_block_comment(&mut self) -> PrimitiveTokenKind {
		let mut depth: usize = 1;
		while let Some(c) = self.bump() {
			match c {
				// Open nested block comment
				'/' if self.peek() == '*' => {
					self.bump();
					depth += 1;
				}
				// Close nested block comment or outer comment
				'*' if self.peek() == '/' => {
					self.bump();
					depth -= 1;
					if depth == 0 {
						break;
					}
				}
				_ => continue,
			}
		}
		PrimitiveTokenKind::BlockComment { terminated: depth == 0 }
	}

	fn eat_string_literal(&mut self) -> bool {
		while let Some(c) = self.bump() {
			match c {
				// Escaped quote or backslash, ignore them.
				'\\' if self.peek() == '"' || self.peek() == '\\' => {
					self.bump();
					continue;
				}
				// Final quote, stop eating.
				'"' => return true,
				_ => continue,
			}
		}
		false
	}

	fn eat_char_literal(&mut self) -> bool {
		while let Some(c) = self.bump() {
			match c {
				// Newline, stop eating.
				'\n' => return false,
				// Final quote, stop eating.
				'\'' => return true,
				// Escaped quote or backslash, ignore them.
				'\\' if self.peek() == '\'' || self.peek() == '\\' => {
					self.bump();
					continue;
				}
				_ => continue,
			}
		}
		false
	}
}

/// True if `c` is a character that has the Pattern_White_Space Unicode property.
/// (https://www.unicode.org/reports/tr31/)
pub fn is_whitespace(c: char) -> bool {
	// This is Pattern_White_Space.
	//
	// Note that this set is stable (ie, it doesn't change with different
	// Unicode versions), so it's ok to just hard-code the values.

	matches!(
		c,
		// Usual ASCII suspects
		'\u{0009}'   // \t
			| '\u{000A}' // \n
			| '\u{000B}' // vertical tab
			| '\u{000C}' // form feed
			| '\u{000D}' // \r
			| '\u{0020}' // space

			// NEXT LINE from latin1
			| '\u{0085}'

			// Bidi markers
			| '\u{200E}' // LEFT-TO-RIGHT MARK
			| '\u{200F}' // RIGHT-TO-LEFT MARK

			// Dedicated whitespace characters from Unicode
			| '\u{2028}' // LINE SEPARATOR
			| '\u{2029}' // PARAGRAPH SEPARATOR
	)
}

/// True if `c` is valid as a first character of an identifier.
pub fn is_ident_start(c: char) -> bool {
	c == '_' || unicode_ident::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
pub fn is_ident_continue(c: char) -> bool {
	unicode_ident::is_xid_continue(c)
}

#[cfg(test)]
mod test {
	use super::{PrimitiveLitKind::*, PrimitiveTokenKind::*, *};

	fn assert_tokens(input: &str, expect: &[PrimitiveToken]) {
		let tokens: Vec<PrimitiveToken> = tokenize(input).collect();
		assert_eq!(&tokens, expect);
	}

	#[test]
	fn empty() {
		assert_tokens("", &[])
	}

	#[test]
	fn line_comment() {
		assert_tokens("// ² $ line @ comment", &[PrimitiveToken::new(LineComment, 22)]);
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
				PrimitiveToken::new(BlockComment { terminated: true }, 17),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(BlockComment { terminated: true }, 46),
			],
		);
		assert_tokens(
			r#"/* testing *_ /*
			sdasd
			/* 8327 */
			testing
			*/"#,
			&[PrimitiveToken::new(BlockComment { terminated: false }, 56)],
		)
	}

	#[test]
	fn ident() {
		assert_tokens(
			"π _tst__ing stµff",
			&[
				PrimitiveToken::new(Ident, 2),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Ident, 9),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Ident, 6),
			],
		)
	}

	#[test]
	fn line_end() {
		assert_tokens("\n", &[PrimitiveToken::new(Whitespace, 1)]);
		assert_tokens("\r\n", &[PrimitiveToken::new(Whitespace, 2)]);
		assert_tokens(";\n", &[PrimitiveToken::new(Semi, 1), PrimitiveToken::new(Whitespace, 1)])
	}

	#[test]
	fn one_char() {
		assert_tokens(
			"\n \t \r;:,. |&+-*/%=! ~?@^$# <>(){}[]",
			&[
				// Whitespace
				PrimitiveToken::new(Whitespace, 5),
				// Punctuation
				PrimitiveToken::new(Semi, 1),
				PrimitiveToken::new(Colon, 1),
				PrimitiveToken::new(Comma, 1),
				PrimitiveToken::new(Dot, 1),
				PrimitiveToken::new(Whitespace, 1),
				// Used
				PrimitiveToken::new(Or, 1),
				PrimitiveToken::new(And, 1),
				PrimitiveToken::new(Plus, 1),
				PrimitiveToken::new(Minus, 1),
				PrimitiveToken::new(Star, 1),
				PrimitiveToken::new(Slash, 1),
				PrimitiveToken::new(Percent, 1),
				PrimitiveToken::new(Eq, 1),
				PrimitiveToken::new(Bang, 1),
				PrimitiveToken::new(Whitespace, 1),
				// Unused
				PrimitiveToken::new(Tilde, 1),
				PrimitiveToken::new(Question, 1),
				PrimitiveToken::new(At, 1),
				PrimitiveToken::new(Caret, 1),
				PrimitiveToken::new(Dollar, 1),
				PrimitiveToken::new(Pound, 1),
				PrimitiveToken::new(Whitespace, 1),
				// Delimiter
				PrimitiveToken::new(LessThan, 1),
				PrimitiveToken::new(GreaterThan, 1),
				PrimitiveToken::new(OpenParen, 1),
				PrimitiveToken::new(CloseParen, 1),
				PrimitiveToken::new(OpenBrace, 1),
				PrimitiveToken::new(CloseBrace, 1),
				PrimitiveToken::new(OpenBracket, 1),
				PrimitiveToken::new(CloseBracket, 1),
			],
		)
	}

	#[test]
	fn integer() {
		assert_tokens(
			"0 1 2 42739387324 0000234236932 999_999_999_999",
			&[
				PrimitiveToken::new(Literal { kind: Integer }, 1),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Integer }, 1),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Integer }, 1),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Integer }, 11),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Integer }, 13),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Integer }, 15),
			],
		)
	}

	#[test]
	fn float() {
		assert_tokens(
			"0. 123. 2.222 4273.9387324 0000.234236932 999_999_999.999",
			&[
				PrimitiveToken::new(Literal { kind: Float }, 2),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Float }, 4),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Float }, 5),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Float }, 12),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Float }, 14),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: Float }, 15),
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
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 2),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 6),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 30),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 21),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 4),
				PrimitiveToken::new(Whitespace, 4),
			],
		);
		assert_tokens(
			r#" "\\" "\" "#,
			&[
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: String { terminated: true } }, 4),
				PrimitiveToken::new(Whitespace, 1),
				PrimitiveToken::new(Literal { kind: String { terminated: false } }, 4),
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
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 2),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 3),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 21),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 3),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 4),
				PrimitiveToken::new(Whitespace, 4),
			],
		);
		assert_tokens(
			r#"
				'
				'\\'
				'\'
			"#,
			&[
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: false } }, 2),
				PrimitiveToken::new(Whitespace, 4),
				PrimitiveToken::new(Literal { kind: Char { terminated: true } }, 4),
				PrimitiveToken::new(Whitespace, 5),
				PrimitiveToken::new(Literal { kind: Char { terminated: false } }, 4),
				PrimitiveToken::new(Whitespace, 3),
			],
		)
	}
}
