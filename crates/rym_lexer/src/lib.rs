mod cursor;
mod token;
use cursor::Cursor;
use token::{LiteralKind, Token, TokenKind};

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
	let mut cursor = Cursor::new(input);
	std::iter::from_fn(move || {
		let token = cursor.advance_token();
		if token.kind != TokenKind::Eof {
			Some(token)
		} else {
			None
		}
	})
}

impl Cursor<'_> {
	pub fn advance_token(&mut self) -> Token {
		let first_char = match self.bump() {
			Some(char) => char,
			None => return Token::new(TokenKind::Eof, 0),
		};

		let token_kind = match first_char {
			// Slash, line or block comment.
			'/' => match self.peek_1() {
				'/' => {
					self.eat_while(|c| c == '\n');
					TokenKind::LineComment
				}
				'*' => TokenKind::BlockComment,
				_ => TokenKind::Slash,
			},

			c if is_whitespace(c) => {
				self.eat_while(|c| c.is_whitespace());
				TokenKind::Whitespace
			}

			// Number literal.
			'0'..='9' => {
				self.eat_while(|c| c.is_numeric() || matches!(c, '_' | '.'));
				TokenKind::Literal { kind: LiteralKind::Number }
			}

			// String literal.
			'"' => {
				self.eat_while(|c| c == '"');
				// TODO
				TokenKind::Literal { kind: LiteralKind::String { terminated: true } }
			}

			// Character literal.
			'\'' => {
				self.eat_while(|c| c == '\'');
				TokenKind::Literal { kind: LiteralKind::Char { terminated: true } }
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
		result
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
fn is_ident_start(c: char) -> bool {
	c == '_' || unicode_ident::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
fn is_ident_continue(c: char) -> bool {
	unicode_ident::is_xid_continue(c)
}

#[cfg(test)]
mod test {
	use super::*;

	fn assert_tokens(input: &str, expect: &[Token]) {
		let tokens: Vec<Token> = tokenize(input).collect();
		assert_eq!(&tokens, expect);
	}

	#[test]
	fn empty() {
		assert_tokens("", &[])
	}

	#[test]
	fn ident() {
		assert_tokens(
			"π _tst__ing stµff",
			&[
				Token::new(TokenKind::Ident, 2),
				Token::new(TokenKind::Whitespace, 1),
				Token::new(TokenKind::Ident, 9),
				Token::new(TokenKind::Whitespace, 1),
				Token::new(TokenKind::Ident, 6),
			],
		)
	}

	#[test]
	fn line_end() {
		assert_tokens(
			" ;\n",
			&[
				Token::new(TokenKind::Whitespace, 1),
				Token::new(TokenKind::Semi, 1),
				Token::new(TokenKind::Whitespace, 1),
			],
		)
	}

	#[test]
	fn one_char() {
		assert_tokens(
			"\n \t \r;:,. |&+-*/%=! @^$# <>(){}[]",
			&[
				// Whitespace
				Token::new(TokenKind::Whitespace, 5),
				// Punctuation
				Token::new(TokenKind::Semi, 1),
				Token::new(TokenKind::Colon, 1),
				Token::new(TokenKind::Comma, 1),
				Token::new(TokenKind::Dot, 1),
				Token::new(TokenKind::Whitespace, 1),
				// Used
				Token::new(TokenKind::Or, 1),
				Token::new(TokenKind::And, 1),
				Token::new(TokenKind::Plus, 1),
				Token::new(TokenKind::Minus, 1),
				Token::new(TokenKind::Star, 1),
				Token::new(TokenKind::Slash, 1),
				Token::new(TokenKind::Percent, 1),
				Token::new(TokenKind::Eq, 1),
				Token::new(TokenKind::Bang, 1),
				Token::new(TokenKind::Whitespace, 1),
				// Unused
				Token::new(TokenKind::At, 1),
				Token::new(TokenKind::Caret, 1),
				Token::new(TokenKind::Dollar, 1),
				Token::new(TokenKind::Pound, 1),
				Token::new(TokenKind::Whitespace, 1),
				// Delimiter
				Token::new(TokenKind::LessThan, 1),
				Token::new(TokenKind::GreaterThan, 1),
				Token::new(TokenKind::OpenParen, 1),
				Token::new(TokenKind::CloseParen, 1),
				Token::new(TokenKind::OpenBrace, 1),
				Token::new(TokenKind::CloseBrace, 1),
				Token::new(TokenKind::OpenBracket, 1),
				Token::new(TokenKind::CloseBracket, 1),
			],
		)
	}
}
