use crate::ast::{BinaryOp, Block, Expr, Literal, Stmt, UnaryOp};
use crate::{Token, TokenType};

mod error;
use error::ParserError;

pub struct Parser<'src> {
	tokens: Vec<Token<'src>>,
	pos: usize,
	finished: bool,
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
			pos: 0,
			finished: false,
		}
	}

	pub fn stmt(&mut self) -> Result<Stmt<'src>, ParserError<'src>> {
		if self.matches(TokenType::Semicolon) {
			return Ok(Stmt::Empty);
		}

		let expr = self.expr()?;
		self.expect(TokenType::Semicolon, "Expected `;`")?;
		Ok(Stmt::Expr(expr))
	}

	fn expr(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		self.logic_or()
	}

	/// logic_or => logic_and ("and" logic_and)*
	fn logic_or(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.logic_and()?;

		while self.matches(TokenType::Or) {
			let right = Box::new(self.logic_and()?);
			left = Expr::Binary(Box::new(left), BinaryOp::Or, right)
		}

		Ok(left)
	}

	/// logic_and => equality ("and" equality)*
	fn logic_and(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.equality()?;

		while self.matches(TokenType::And) {
			let right = Box::new(self.equality()?);
			left = Expr::Binary(Box::new(left), BinaryOp::And, right)
		}

		Ok(left)
	}

	/// equality => comparison (("==" | "!=") comparison)*
	fn equality(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.comparison()?;

		while self.matches_any(&[TokenType::EqualEqual, TokenType::BangEqual]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.comparison()?);

			left = if typ == TokenType::EqualEqual {
				Expr::Binary(Box::new(left), BinaryOp::Eq, right)
			} else {
				Expr::Binary(Box::new(left), BinaryOp::Ne, right)
			}
		}

		Ok(left)
	}

	/// comparison => term ((">" | ">=" | "<" | "<=") term)*
	fn comparison(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.term()?;

		while self.matches_any(&[
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.term()?);

			left = Expr::Binary(
				Box::new(left),
				match typ {
					TokenType::Greater => BinaryOp::Gt,
					TokenType::GreaterEqual => BinaryOp::Ge,
					TokenType::Less => BinaryOp::Lt,
					_ => BinaryOp::Le,
				},
				right,
			);
		}

		Ok(left)
	}

	/// term => factor (("+" | "-") factor)*
	fn term(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.factor()?);

			left = if typ == TokenType::Plus {
				Expr::Binary(Box::new(left), BinaryOp::Add, right)
			} else {
				Expr::Binary(Box::new(left), BinaryOp::Sub, right)
			}
		}

		Ok(left)
	}

	/// factor => unary (("/" | "*") unary)*
	fn factor(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut left = self.unary()?;

		while self.matches_any(&[TokenType::Star, TokenType::Slash]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.unary()?);

			left = if typ == TokenType::Star {
				Expr::Binary(Box::new(left), BinaryOp::Mul, right)
			} else {
				Expr::Binary(Box::new(left), BinaryOp::Div, right)
			};
		}

		Ok(left)
	}

	/// unary => ("!" | "-") unary
	fn unary(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		if self.matches(TokenType::Bang) {
			let expr = Box::new(self.expr()?);
			return Ok(Expr::Unary(UnaryOp::Not, expr));
		}
		if self.matches(TokenType::Minus) {
			let expr = Box::new(self.expr()?);
			return Ok(Expr::Unary(UnaryOp::Neg, expr));
		}
		self.primary()
	}

	/// primary => "(" expr ")", block, identifier, number | string | "true" | "false"
	fn primary(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		if self.matches(TokenType::LeftBrace) {
			return self.block();
		}

		if self.matches(TokenType::LeftParen) {
			let expr = Box::new(self.expr()?);
			self.expect(TokenType::RightParen, "Expected `)`")?;
			return Ok(Expr::Group(expr));
		}

		match self.matches_which(&[
			TokenType::False,
			TokenType::True,
			TokenType::Number,
			TokenType::String,
			TokenType::Identifier,
		]) {
			Some(token) => match &token.typ {
				TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
				TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
				TokenType::Number | TokenType::String | TokenType::Identifier => Ok(Expr::Literal(
					token
						.literal
						.to_owned()
						.expect("Internal Error: Literal token has no value!"),
				)),
				_ => panic!("Internal Error: Should never be reached!"),
			},
			None => ParserError::token_mismatch(self.advance(), "Expected Literal"),
		}
	}

	/// block => "{" stmt* "}"
	fn block(&mut self) -> Result<Expr<'src>, ParserError<'src>> {
		let mut stmts = Vec::new();
		loop {
			let stmt = self.stmt()?;
			stmts.push(stmt);
			if self.matches_any(&[TokenType::RightBrace, TokenType::Semicolon, TokenType::Eof]) {
				break;
			}
		}
		Ok(Expr::Block(Block { stmts }))
	}
}

impl<'src> Parser<'src> {
	fn expect(&mut self, typ: TokenType, error_msg: &str) -> Result<&Token, ParserError<'src>> {
		if self.matches(typ) {
			return Ok(self.previous());
		}

		ParserError::token_mismatch(self.advance(), error_msg)
	}

	fn matches_any(&mut self, types: &[TokenType]) -> bool {
		for typ in types {
			if self.matches(typ.clone()) {
				return true;
			}
		}

		false
	}

	fn matches_which(&mut self, types: &[TokenType]) -> Option<&Token<'src>> {
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
		if self.finished {
			return None;
		}
		match self.stmt() {
			Ok(stmt) => {
				if self.is_at_end() {
					self.finished = true;
				}
				Some(Ok(stmt))
			}
			Err(err) => Some(Err(err)),
		}
	}
}
