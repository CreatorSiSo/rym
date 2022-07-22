use crate::ast::Literal;
use crate::token::{Token, TokenType, KEYWORDS};
use std::str::CharIndices;

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
	pub fn lex(source: &'src str) -> (Vec<Token<'src>>, Vec<LexerError>) {
		let mut tokens = Vec::new();
		let mut errors = Vec::new();

		for result in Lexer::new(source) {
			match result {
				Ok(token) => {
					tokens.push(token);
				}
				Err(err) => {
					errors.push(err);
				}
			}
		}

		(tokens, errors)
	}

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
				break TokenType::Eof;
			}

			let c = self.c;
			self.start = self.current;
			match c {
				' ' | '\t' | '\n' | '\r' => continue,

				// TODO: Comments
				'/' if self.matches('/') => self.consume_while(|c| c != '\n'),
				'/' if self.matches('*') => self.multiline_comment(),

				'+' => break TokenType::Plus,
				'-' => break TokenType::Minus,
				'*' => break TokenType::Star,
				'/' => break TokenType::Slash,

				'.' => break TokenType::Dot,
				',' => break TokenType::Comma,
				';' => break TokenType::Semicolon,
				'(' => break TokenType::LeftParen,
				')' => break TokenType::RightParen,
				'{' => break TokenType::LeftBrace,
				'}' => break TokenType::RightBrace,

				'!' if self.matches('=') => break TokenType::BangEqual,
				'!' => break TokenType::Bang,
				'=' if self.matches('=') => break TokenType::EqualEqual,
				'=' => break TokenType::Equal,
				'>' if self.matches('=') => break TokenType::GreaterEqual,
				'>' => break TokenType::Greater,
				'<' if self.matches('=') => break TokenType::LessEqual,
				'<' => break TokenType::Less,

				'0'..='9' => return self.number(),
				'"' => return self.string(),

				c if c.is_alphabetic() || c == '_' => return self.identifier(),
				_ => return LexerError::unexpected_char(self),
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

	fn number(&mut self) -> Result<Token<'src>, LexerError> {
		self.consume_while(|c| c.is_ascii_digit());

		if self.peek(1) == '.' && self.peek(2).is_ascii_digit() {
			self.advance(); // Consume .
			self.consume_while(|c| c.is_ascii_digit());
		}

		let text = &self.source[self.start..=self.current];
		let value = match text.parse::<f64>() {
			Ok(number) => number,
			Err(err) => return LexerError::parse_float(self, err),
		};

		Ok(Token::literal(
			TokenType::Number,
			Literal::Number(value),
			self.start,
		))
	}

	fn string(&mut self) -> Result<Token<'src>, LexerError> {
		while !self.is_at_end() {
			if self.c == '\\' && self.matches('"') {
				self.advance();
			}
			if self.matches('"') {
				break;
			}
			self.advance();
		}

		Ok(Token::literal(
			TokenType::String,
			Literal::String(unescape(&self.source[self.start + 1..self.current])),
			self.start,
		))
	}

	fn identifier(&mut self) -> Result<Token<'src>, LexerError> {
		self.consume_while(|c| c.is_alphanumeric() || c == '_');

		let text = &self.source[self.start..=self.current];
		let typ = match KEYWORDS.iter().find(|(key, _)| key == &text) {
			Some((_, token_type)) => token_type.clone(),
			None => TokenType::Identifier,
		};
		Ok(Token::literal(typ, Literal::Identifier(text), self.start))
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
