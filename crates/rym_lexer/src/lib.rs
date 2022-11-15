mod cursor;
mod token;
pub use cursor::Cursor;
pub use token::{LiteralKind, Token, TokenKind};

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
	let mut cursor = Cursor::new(input);
	std::iter::from_fn(move || cursor.advance_token())
}

impl Cursor<'_> {
	pub fn advance_token(&mut self) -> Option<Token> {
		let first_char = match self.bump() {
			Some(char) => char,
			None => return None,
		};

		let token_kind = match first_char {
			// Slash, line or block comment.
			'/' => match self.peek() {
				'/' => {
					self.eat_while(|c| c != '\n');
					TokenKind::LineComment
				}
				'*' => self.eat_block_comment(),
				_ => TokenKind::Slash,
			},

			c if is_whitespace(c) => {
				self.eat_while(|c| c.is_whitespace());
				TokenKind::Whitespace
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
				TokenKind::Literal { kind: if is_int { LiteralKind::Integer } else { LiteralKind::Float } }
			}

			// String literal.
			'"' => {
				TokenKind::Literal { kind: LiteralKind::String { terminated: self.eat_string_literal() } }
			}

			// Character literal.
			'\'' => {
				TokenKind::Literal { kind: LiteralKind::Char { terminated: self.eat_char_literal() } }
			}

			// Indetifier token.
			c if is_ident_start(c) => {
				self.eat_while(is_ident_continue);
				TokenKind::Ident
			}

			// Punctuation tokens.
			';' => TokenKind::Semi,
			':' => TokenKind::Colon,
			',' => TokenKind::Comma,
			'.' => TokenKind::Dot,

			// One character tokens.
			'|' => TokenKind::Or,
			'&' => TokenKind::And,
			'+' => TokenKind::Plus,
			'-' => TokenKind::Minus,
			'*' => TokenKind::Star,
			'%' => TokenKind::Percent,
			'=' => TokenKind::Eq,
			'!' => TokenKind::Bang,

			// Currently unused one character tokens.
			'~' => TokenKind::Tilde,
			'?' => TokenKind::Question,
			'@' => TokenKind::At,
			'^' => TokenKind::Caret,
			'$' => TokenKind::Dollar,
			'#' => TokenKind::Pound,

			// Delimiter like tokens.
			'<' => TokenKind::LessThan,
			'>' => TokenKind::GreaterThan,
			'(' => TokenKind::OpenParen,
			')' => TokenKind::CloseParen,
			'{' => TokenKind::OpenBrace,
			'}' => TokenKind::CloseBrace,
			'[' => TokenKind::OpenBracket,
			']' => TokenKind::CloseBracket,

			_ => TokenKind::Unkown,
		};

		let result = Token::new(token_kind, self.len_consumed());
		self.reset_len_consumed();
		Some(result)
	}

	fn eat_block_comment(&mut self) -> TokenKind {
		let mut depth: usize = 1;
		while let Some(c) = self.bump() {
			match c {
				'/' if self.peek() == '*' => {
					self.bump();
					depth += 1;
				}
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
		TokenKind::BlockComment { terminated: depth == 0 }
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
	use super::{LiteralKind::*, TokenKind::*, *};

	fn assert_tokens(input: &str, expect: &[Token]) {
		let tokens: Vec<Token> = tokenize(input).collect();
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
				Token::new(Literal { kind: Integer }, 1),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Integer }, 1),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Integer }, 1),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Integer }, 11),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Integer }, 13),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Integer }, 15),
			],
		)
	}

	#[test]
	fn float() {
		assert_tokens(
			"0. 123. 2.222 4273.9387324 0000.234236932 999_999_999.999",
			&[
				Token::new(Literal { kind: Float }, 2),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Float }, 4),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Float }, 5),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Float }, 12),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Float }, 14),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: Float }, 15),
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
				Token::new(Literal { kind: String { terminated: true } }, 2),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: String { terminated: true } }, 6),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: String { terminated: true } }, 30),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: String { terminated: true } }, 21),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: String { terminated: true } }, 4),
				Token::new(Whitespace, 4),
			],
		);
		assert_tokens(
			r#" "\\" "\" "#,
			&[
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: String { terminated: true } }, 4),
				Token::new(Whitespace, 1),
				Token::new(Literal { kind: String { terminated: false } }, 4),
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
				Token::new(Literal { kind: Char { terminated: true } }, 2),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: Char { terminated: true } }, 3),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: Char { terminated: true } }, 21),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: Char { terminated: true } }, 3),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: Char { terminated: true } }, 4),
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
				Token::new(Literal { kind: Char { terminated: false } }, 2),
				Token::new(Whitespace, 4),
				Token::new(Literal { kind: Char { terminated: true } }, 4),
				Token::new(Whitespace, 5),
				Token::new(Literal { kind: Char { terminated: false } }, 4),
				Token::new(Whitespace, 3),
			],
		)
	}
}
