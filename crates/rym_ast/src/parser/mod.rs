use crate::ast::{Expr, Literal, Stmt};
use crate::{Token, TokenType};

mod error;
use error::ParserError;

pub struct Parser<'src> {
	tokens: Vec<Token<'src>>,
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
		Self { tokens, pos: 0 }
	}

	pub fn next_stmt(&mut self) -> Result<Stmt<'src>, ParserError<'src>> {
		if self.is_at_end() {
			return Ok(Stmt::Eof);
		}

		if self.matches(TokenType::Semicolon) {
			return Ok(Stmt::Empty);
		}
		self.expr_stmt()
	}

	fn expr_stmt(&mut self) -> Result<Stmt<'src>, ParserError<'src>> {
		let expr = self.expr()?;
		self.expect(TokenType::Semicolon, "Expected `;`")?;

		Ok(Stmt::Expr(expr))
	}

	fn expr(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		self.literal()
	}

	fn literal(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		match self.matches_any(&[TokenType::False, TokenType::True]) {
			Some(token) => match &token.typ {
				TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
				TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
				_ => ParserError::token_mismatch(token, "Expected Literal"),
			},
			None => ParserError::token_mismatch(self.advance(), "Expected Literal"),
		}
	}
}

impl<'src> Parser<'src> {
	fn expect(&mut self, typ: TokenType, error_msg: &str) -> Result<&Token, ParserError<'src>> {
		if self.matches(typ) {
			return Ok(self.previous());
		}

		ParserError::token_mismatch(self.advance(), error_msg)
	}

	fn matches_any(&mut self, types: &[TokenType]) -> Option<&Token<'src>> {
		for typ in types {
			if self.matches(typ.clone()) {
				return Some(self.previous());
			}
		}

		None
	}

	fn matches(&mut self, typ: TokenType) -> bool {
		if self.peek().typ == typ {
			self.advance();
			return true;
		}
		false
	}

	fn advance(&mut self) -> &Token<'src> {
		if !self.is_at_end() {
			self.pos += 1;
		}
		self.previous()
	}

	fn previous(&self) -> &Token<'src> {
		&self.tokens[self.pos - 1]
	}

	fn is_at_end(&self) -> bool {
		if self.peek().typ == TokenType::Eof {
			return true;
		}
		false
	}

	fn peek(&self) -> &Token<'src> {
		&self.tokens[self.pos]
	}
}

impl<'src> Iterator for Parser<'src> {
	type Item = Result<Stmt<'src>, ParserError<'src>>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.next_stmt() {
			Ok(stmt) if stmt == Stmt::Eof => None,
			Ok(stmt) => Some(Ok(stmt)),
			Err(err) => Some(Err(err)),
		}
	}
}
