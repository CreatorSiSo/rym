use std::str::CharIndices;

mod error;
mod unescape;
use ast::{Literal, Spanned, Token, TokenType, KEYWORDS};
pub use error::LexError;
use unescape::unescape;

pub type LexResult<T> = Result<T, LexError>;

pub struct Lexer<'src> {
	src: &'src str,
	iter: CharIndices<'src>,

	c: char,
	start: usize,
	current: usize,

	// TODO: Are these still being used?
	line: usize,
	col: usize,
}

impl<'src> Lexer<'src> {
	pub fn lex(source: &'src str) -> (Vec<Spanned<Token>>, Vec<LexError>) {
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

	pub fn new(src: &'src str) -> Self {
		Self {
			src,
			iter: src.char_indices(),

			c: '\0',
			start: 0,
			current: 0,

			line: 1,
			col: 0,
		}
	}

	fn next_token(&mut self) -> LexResult<Spanned<Token>> {
		let token_value = loop {
			self.advance();
			if self.is_at_end() {
				break TokenType::Eof;
			}

			let c = self.c;
			self.start = self.current;
			match c {
				' ' | '\t' | '\r' => continue,
				'\n' => break TokenType::Newline,

				// TODO: Comments
				'/' if self.matches('/') => self.consume_while(|c| c != '\n'),
				'/' if self.matches('*') => self.multiline_comment(),

				'&' if self.matches('&') => break TokenType::DoubleAmpersand,
				'|' if self.matches('|') => break TokenType::DoublePipe,

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
				_ => return LexError::invalid_char(self),
			};
		};

		Ok(Spanned(self.start..self.current, Token::new(token_value)))
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

	fn number(&mut self) -> LexResult<Spanned<Token>> {
		self.consume_while(|c| c.is_ascii_digit());

		if self.peek(1) == '.' && self.peek(2).is_ascii_digit() {
			self.advance(); // Consume .
			self.consume_while(|c| c.is_ascii_digit());
		}

		let text = &self.src[self.start..=self.current];
		let value = match text.parse::<f64>() {
			Ok(number) => number,
			Err(err) => return LexError::parse_float(self, err),
		};

		Ok(Spanned(
			self.start..self.current,
			Token::literal(TokenType::Number, Literal::Number(value)),
		))
	}

	fn string(&mut self) -> LexResult<Spanned<Token>> {
		while !self.is_at_end() {
			if self.c == '\\' && self.matches('"') {
				self.advance();
			}
			if self.matches('"') {
				break;
			}
			self.advance();
		}

		Ok(Spanned(
			self.start..self.current,
			Token::literal(
				TokenType::String,
				Literal::String(unescape(&self.src[self.start + 1..self.current])),
			),
		))
	}

	fn identifier(&mut self) -> LexResult<Spanned<Token>> {
		self.consume_while(|c| c.is_alphanumeric() || c == '_');

		let name = String::from(&self.src[self.start..=self.current]);
		let typ = match KEYWORDS.iter().find(|(key, _)| key == &name) {
			Some((_, token_type)) => token_type.clone(),
			None => TokenType::Identifier,
		};
		Ok(Spanned(self.start..self.current, Token::ident(typ, name)))
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
			self.current = self.src.len();
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
		self.current >= self.src.len()
	}
}

impl<'src> Iterator for Lexer<'src> {
	type Item = LexResult<Spanned<Token>>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_at_end() {
			None
		} else {
			let next = self.next_token();
			if let Ok(Spanned(_, tok)) = &next {
				if tok.typ == TokenType::Eof {
					return None;
				}
			}
			Some(next)
		}
	}
}
