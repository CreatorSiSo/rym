use crate::ast::Stmt;
use crate::{Token, TokenValue};

mod error;
use error::ParserError;

pub struct Parser<'src> {
	tokens: Vec<Token<'src>>,
	token: Token<'src>,
	pos: usize,
}

impl<'src> Parser<'src> {
	pub fn parse(tokens: Vec<Token<'src>>) -> (Vec<Stmt>, Vec<ParserError>) {
		let mut stmts = Vec::new();
		let mut errors = Vec::new();

		for result in Parser::new(tokens) {
			match result {
				Ok(stmt) => {
					stmts.push(stmt);
				}
				Err(err) => {
					errors.push(err);
				}
			}
		}

		(stmts, errors)
	}

	pub fn new(tokens: Vec<Token<'src>>) -> Self {
		Self {
			tokens,
			token: Token::new(TokenValue::Eof, 0),
			pos: 0,
		}
	}

	pub fn next_stmt(&mut self) -> Result<Stmt, ParserError<'src>> {
		self.advance();
		if self.is_at_end() {
			return Ok(Stmt::Empty);
		}

		if self.matches(TokenValue::Const) {
			return self.local(false);
		}
		if self.matches(TokenValue::Mut) {
			return self.local(true);
		}
		self.stmt()
	}

	fn local(&mut self, is_mutable: bool) -> Result<Stmt, ParserError<'src>> {
		todo!()
	}

	fn stmt(&self) -> Result<Stmt, ParserError<'src>> {
		todo!()
	}
}

impl<'src> Parser<'src> {
	fn matches(&mut self, value: TokenValue) -> bool {
		if self.peek(1) == &value {
			self.advance();
			return true;
		}
		false
	}

	fn peek(&self, n: usize) -> &TokenValue {
		if let Some(token) = self.tokens.get(self.pos + n) {
			&token.value
		} else {
			&TokenValue::Eof
		}
	}

	fn advance(&mut self) {
		self.pos += 1;
		if let Some(token) = self.tokens.get(self.pos) {
			self.token = token.clone();
		} else {
			self.pos = self.tokens.len();
		}
	}

	fn is_at_end(&self) -> bool {
		if self.token.value == TokenValue::Eof {
			return true;
		}
		false
	}
}

impl<'src> Iterator for Parser<'src> {
	type Item = Result<Stmt, ParserError<'src>>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_at_end() {
			None
		} else {
			Some(self.next_stmt())
		}
	}
}
