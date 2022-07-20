use std::str::CharIndices;

use crate::token::{Token, TokenValue, KEYWORDS};

mod error;
mod test;
use error::LexerError;

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
			col: 0,
		}
	}

	pub fn token(&mut self) -> Result<Token<'src>, LexerError> {
		let token_value = loop {
			self.advance();
			if self.is_at_end() {
				break TokenValue::Eof;
			}

			let c = self.c;
			self.start = self.current;
			match c {
				'\n' | ' ' | '\t' | '\r' => continue,

				// TODO: Comments
				// '/' if self.matches('/') => break self.line_comment(),
				// '/' if self.matches('*') => break self.multiline_comment(),
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
				c => return LexerError::err(format!("Unexpected character `{c}`"), self.line, self.col),
			};
		};

		Ok(Token::new(token_value, self.start))
	}

	fn number(&mut self) -> Result<TokenValue<'src>, LexerError> {
		while self.peek(1).is_ascii_digit() && !self.is_at_end() {
			self.advance();
		}

		if self.peek(1) == '.' && self.peek(2).is_ascii_digit() {
			// Consume .
			self.advance();

			while self.peek(1).is_ascii_digit() {
				self.advance();
			}
		}

		let text = &self.source[self.start..=self.current];

		match text.parse::<f64>() {
			Ok(number) => Ok(TokenValue::Number(number)),
			Err(err) => LexerError::err(format!("{:?}", err), 0, 0),
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

		TokenValue::String(&self.source[self.start + 1..self.current])
	}

	fn identifier(&mut self) -> TokenValue<'src> {
		while self.peek(1).is_alphanumeric() || self.peek(1) == '_' {
			self.advance();
		}

		let text = &self.source[self.start..=self.current];

		match KEYWORDS.iter().find(|(key, _)| key == &text) {
			Some((_, token_type)) => token_type.clone(),
			None => TokenValue::Identifier(text),
		}
	}
}

impl<'src> Lexer<'src> {
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
			self.col = 0;
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
			Some(self.token())
		}
	}
}
