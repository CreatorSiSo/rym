use std::str::CharIndices;

use crate::token::{Token, TokenValue, KEYWORDS};

mod error;
mod test;
mod unescape;
use error::LexerError;
use unescape::unescape;

pub struct Lexer<'src> {
	source: &'src str,
	iter: CharIndices<'src>,

	c: char,
	start: usize,
	current: usize,

	line: usize,
	col: usize,
}

impl<'src> Lexer<'src> {
	pub fn new(source: &'src str) -> Self {
		Self {
			source,
			iter: source.char_indices(),

			c: '\0',
			start: 0,
			current: 0,

			line: 1,
			col: 1,
		}
	}

	pub fn next_token(&mut self) -> Result<Token<'src>, LexerError> {
		let token_value = loop {
			self.advance();
			if self.is_at_end() {
				break TokenValue::Eof;
			}

			let c = self.c;
			self.start = self.current;
			match c {
				' ' | '\t' | '\n' | '\r' => continue,

				// TODO: Comments
				'/' if self.matches('/') => self.consume_while(|c| c != '\n'),
				'/' if self.matches('*') => self.multiline_comment(),

				'+' => break TokenValue::Plus,
				'-' => break TokenValue::Minus,
				'*' => break TokenValue::Star,
				'/' => break TokenValue::Slash,

				'.' => break TokenValue::Dot,
				',' => break TokenValue::Comma,
				';' => break TokenValue::Semicolon,
				'(' => break TokenValue::LeftParen,
				')' => break TokenValue::RightParen,
				'{' => break TokenValue::LeftBrace,
				'}' => break TokenValue::RightBrace,

				'!' if self.matches('=') => break TokenValue::BangEqual,
				'!' => break TokenValue::Bang,
				'=' if self.matches('=') => break TokenValue::EqualEqual,
				'=' => break TokenValue::Equal,
				'>' if self.matches('=') => break TokenValue::GreaterEqual,
				'>' => break TokenValue::Greater,
				'<' if self.matches('=') => break TokenValue::LessEqual,
				'<' => break TokenValue::Less,

				'0'..='9' => break self.number()?,
				'"' => break self.string(),

				c if c.is_alphabetic() || c == '_' => break self.identifier(),
				_ => return LexerError::unexpected_char(&self),
			};
		};

		Ok(Token::new(token_value, self.start))
	}

	fn multiline_comment(&mut self) {
		let mut nested = 0;
		while !self.is_at_end() {
			if self.peek(1) == '/' && self.peek(2) == '*' {
				nested += 1;
			}
			if self.peek(1) == '*' && self.peek(2) == '/' {
				nested -= 1;
			}
			if nested < 0 {
				break;
			}
			self.advance();
		}
		// Consume */
		self.advance();
		self.advance();
	}

	fn number(&mut self) -> Result<TokenValue<'src>, LexerError> {
		let mut is_int = true;
		self.consume_while(|c| c.is_ascii_digit());

		if self.peek(1) == '.' && self.peek(2).is_ascii_digit() {
			is_int = false;
			self.advance(); // Consume .
			self.consume_while(|c| c.is_ascii_digit());
		}

		let text = &self.source[self.start..=self.current];
		if is_int {
			match text.parse::<i64>() {
				Ok(int) => Ok(TokenValue::Int(int)),
				Err(err) => LexerError::parse_int(&self, err),
			}
		} else {
			match text.parse::<f64>() {
				Ok(number) => Ok(TokenValue::Number(number)),
				Err(err) => LexerError::parse_float(&self, err),
			}
		}
	}

	fn string(&mut self) -> TokenValue<'src> {
		while !self.is_at_end() {
			if self.c == '\\' && self.matches('"') {
				self.advance();
			}
			if self.matches('"') {
				break;
			}
			self.advance();
		}

		TokenValue::String(unescape(&self.source[self.start + 1..self.current]))
	}

	fn identifier(&mut self) -> TokenValue<'src> {
		self.consume_while(|c| c.is_alphanumeric() || c == '_');

		let text = &self.source[self.start..=self.current];
		match KEYWORDS.iter().find(|(key, _)| key == &text) {
			Some((_, token_type)) => token_type.clone(),
			None => TokenValue::Identifier(text),
		}
	}
}

impl<'src> Lexer<'src> {
	fn consume_while<F>(&mut self, f: F)
	where
		F: Fn(char) -> bool,
	{
		while f(self.peek(1)) && !self.is_at_end() {
			self.advance()
		}
	}

	fn matches(&mut self, c: char) -> bool {
		if self.is_at_end() || self.peek(1) != c {
			return false;
		};

		self.advance();
		true
	}

	fn advance(&mut self) {
		if let Some((i, c)) = self.iter.next() {
			self.col += 1;
			self.current = i;
			self.c = c;
		} else {
			self.current = self.source.len();
			self.c = '\0'
		}

		if self.c == '\n' {
			self.line += 1;
			self.col = 1;
		}
	}

	fn peek(&self, n: usize) -> char {
		if let Some((_, c)) = self.iter.clone().nth(n - 1) {
			c
		} else {
			'\0'
		}
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}
}

impl<'src> Iterator for Lexer<'src> {
	type Item = Result<Token<'src>, LexerError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_at_end() {
			None
		} else {
			Some(self.next_token())
		}
	}
}
