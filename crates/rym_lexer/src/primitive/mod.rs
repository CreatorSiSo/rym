mod test;
mod token;
pub use self::token::{Token, TokenKind};

/// Creates an iterator that produces tokens from the input string.
pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
	let mut lexer = Lexer::new(input);
	std::iter::from_fn(move || lexer.next_token())
}

///	Iterator that turns `char`s into `Token`s
///
/// ## Internals
///
/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
#[derive(Clone, Debug)]
pub struct Lexer<'a> {
	len_remaining: usize,
	chars: std::str::Chars<'a>,
}

impl Lexer<'_> {
	pub fn next_token(&mut self) -> Option<Token> {
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
				if is_int {
					TokenKind::Integer
				} else {
					TokenKind::Float
				}
			}

			// String literal.
			'"' => TokenKind::String { terminated: self.eat_string_literal() },

			// Character literal.
			'\'' => TokenKind::Char { terminated: self.eat_char_literal() },

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
			'|' => TokenKind::Pipe,
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

impl<'a> Lexer<'a> {
	pub fn new(src: &'a str) -> Self {
		Self { len_remaining: src.len(), chars: src.chars() }
	}

	pub(super) fn bump(&mut self) -> Option<char> {
		self.chars.next()
	}

	/// Eats chars while predicate returns true or until the end of file is reached.
	pub(super) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
		while predicate(self.peek()) && !self.is_eof() {
			self.bump();
		}
	}

	/// Peeks the next symbol from the input stream without consuming it.
	/// Returns `'\0'` if the requested position doesn't exist.
	pub(super) fn peek(&self) -> char {
		self.chars.clone().next().unwrap_or('\0')
	}

	/// Returns amount of already consumed symbols.
	pub(super) fn len_consumed(&self) -> u32 {
		(self.len_remaining - self.chars.as_str().len()) as u32
	}

	/// Resets the number of bytes consumed to 0.
	pub(super) fn reset_len_consumed(&mut self) {
		self.len_remaining = self.chars.as_str().len();
	}

	/// Checks if there is nothing more to consume.
	pub(super) fn is_eof(&self) -> bool {
		self.chars.as_str().is_empty()
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
