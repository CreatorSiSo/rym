use std::str::CharIndices;

use crate::token::{Token, TokenValue, KEYWORDS};

mod error;
use error::LexerError;

pub struct Lexer<'src> {
	input: &'src str,
	iter: CharIndices<'src>,
	c: char,
	i: usize,
}

impl<'src> Lexer<'src> {
	pub fn new(source: &'src str) -> Self {
		Self {
			input: source,
			iter: source.char_indices(),
			c: '\0',
			i: 0,
		}
	}

	pub fn next_token(&mut self) -> Result<Token, LexerError> {
		self.advance();
		self.consume_whitespace();
		if self.is_at_end() {
			return Ok(Token::new(TokenValue::Eof, self.i));
		}

		let c = self.c;
		let token_value = match c {
			'+' => TokenValue::Plus,
			'-' => TokenValue::Minus,
			'*' => TokenValue::Star,
			'/' => TokenValue::Slash, // TODO: Comments // '/' if self.matches('*') => self.multiline_comment(), // '/' if self.matches('/') => self.line_comment(),

			'.' => TokenValue::Dot,
			',' => TokenValue::Comma,
			';' => TokenValue::Semicolon,
			'(' => TokenValue::LeftParen,
			')' => TokenValue::RightParen,
			'{' => TokenValue::LeftBrace,
			'}' => TokenValue::RightBrace,

			'!' if self.matches('=') => TokenValue::BangEqual,
			'!' => TokenValue::Bang,
			'=' if self.matches('=') => TokenValue::EqualEqual,
			'=' => TokenValue::Equal,
			'>' if self.matches('=') => TokenValue::GreaterEqual,
			'>' => TokenValue::Greater,
			'<' if self.matches('=') => TokenValue::LessEqual,
			'<' => TokenValue::Less,

			'0'..='9' => self.number()?,
			// '\'' => self.char(),
			'"' => self.string(),

			c if c.is_alphabetic() || c == '_' => self.identifier(),
			c => return LexerError::err(format!("Unexpected character {c}"), 0, 0),
		};

		Ok(Token::new(token_value, self.i))
	}

	fn number(&mut self) -> Result<TokenValue<'src>, LexerError> {
		let start = self.i;
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

		let text = &self.input[start..=self.i];
		println!("{text}");

		match text.parse::<f64>() {
			Ok(number) => Ok(TokenValue::Number(number)),
			Err(err) => LexerError::err(format!("{:?}", err), 0, 0),
		}
	}

	fn string(&mut self) -> TokenValue<'src> {
		// Consume "
		self.advance();

		let start = self.i;
		// TODO: Make \" work
		while self.peek(1) != '"' {
			self.advance()
		}

		// Consume "
		self.advance();

		TokenValue::String(&self.input[start..self.i])
	}

	fn identifier(&mut self) -> TokenValue<'src> {
		let start = self.i;
		while self.peek(1).is_alphanumeric() || self.peek(1) == '_' {
			self.advance();
			// println!("{}, {}", self.c, self.c.is_alphanumeric());
		}

		let text = &self.input[start..=self.i];

		match KEYWORDS.iter().find(|(key, _)| key == &text) {
			Some((_, token_type)) => token_type.clone(),
			None => TokenValue::Identifier(text),
		}
	}
}

impl<'src> Lexer<'src> {
	fn consume_whitespace(&mut self) {
		while self.c == ' ' || self.c == '\t' || self.c == '\n' || self.c == '\r' {
			self.advance();
		}
	}

	fn matches(&mut self, c: char) -> bool {
		if self.is_at_end() {
			return false;
		};
		if self.peek(1) != c {
			return false;
		};

		self.advance();
		true
	}

	fn advance(&mut self) {
		if let Some((i, c)) = self.iter.next() {
			self.i = i;
			self.c = c;
		// println!("({i}, '{c}')");
		} else {
			self.i = self.input.len();
			self.c = '\0'
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
		self.i >= self.input.len()
	}
}

#[test]
fn test() {
	let source = "ident ifier
		- + / *
		. , ; ( ) { }
		! != = == > >= < <=

		0 1 2 3 4 5 6 7 8 9
		12392347.230873460 1 23428934
		34957533457 96

		\"\"\"asdfsd\"\"\"
		\"WOoooOOoooWEEeee!\"
		\"
		- + / *
		. , ; ( ) { }
		! != = == > >= < <=
		if else for while loop return break
 		false true and or
 		fn const mut
 		struct self
 		print
		\"

		if else for while loop return break
		false true and or
		fn const mut
		struct self
		print

		testing(arg1,, arg2)

		\t\t
		\n \r \n\r
	";
	let mut lexer = Lexer::new(source);
	loop {
		let token = lexer.next_token().unwrap();
		if token.value == TokenValue::Eof {
			break;
		}
		println!("{:?}", token)
	}
}
