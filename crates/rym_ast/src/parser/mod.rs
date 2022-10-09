use crate::ast::{BinaryOp, Block, Expr, Literal, Stmt, UnaryOp};
use crate::{Local, LogicalOp, Token, TokenType};

mod error;
use error::ParserError;

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
}

impl Parser {
	pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<ParserError>) {
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

	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}

	pub fn stmt(&mut self) -> Result<Stmt, ParserError> {
		if self.matches(TokenType::Semicolon) {
			return Ok(Stmt::Empty);
		}

		if self.matches(TokenType::Print) {
			let expr = self.expr()?;
			self.matches(TokenType::Semicolon);
			return Ok(Stmt::Print(expr));
		}

		if let Some(token) = self.matches_which(&[TokenType::Const, TokenType::Mut]) {
			let mutable = token.typ == TokenType::Mut;

			self.expect(TokenType::Identifier, "Expected identifier")?;
			let name = self
				.previous()
				.ident
				.clone()
				.expect("Internal Error: Identifier token has no name!")
				.name;

			self.expect(TokenType::Equal, "Expected `=`")?;

			let expr = self.expr()?;
			self.matches(TokenType::Semicolon);
			return Ok(Stmt::Local(if mutable {
				Local::Mut(name, expr)
			} else {
				Local::Const(name, expr)
			}));
		}

		let expr = self.expr()?;
		self.matches(TokenType::Semicolon);
		Ok(Stmt::Expr(expr))
	}

	fn expr(&mut self) -> Result<Expr, ParserError> {
		self.interrupts()
	}

	fn interrupts(&mut self) -> Result<Expr, ParserError> {
		// return => "return" expr?;
		if self.matches(TokenType::Return) {
			if self.matches(TokenType::Semicolon) {
				return Ok(Expr::Break(None));
			}

			return Ok(Expr::Break(Some(Box::new(self.expr()?))));
		}
		// break => "break" TODO: expr?
		if self.matches(TokenType::Break) {
			return Ok(Expr::Break(None));
		}
		// continue => "continue"
		if self.matches(TokenType::Continue) {
			return Ok(Expr::Continue);
		}

		self.assignment()
	}

	/// assignment => identifier "=" expr
	fn assignment(&mut self) -> Result<Expr, ParserError> {
		if self.peek(1).typ == TokenType::Equal {
			let expr_l = Box::new(self.primary()?);
			self.advance();
			let expr_r = Box::new(self.expr()?);

			return Ok(Expr::Assign(expr_l, expr_r));
		}

		self.if_()
	}

	/// if => "if" expression block ("else" (if | block))?
	fn if_(&mut self) -> Result<Expr, ParserError> {
		if self.matches(TokenType::If) {
			let expr = Box::new(self.expr()?);
			let then_block = self.block()?;
			let else_block = if self.matches(TokenType::Else) {
				if self.peek(0).typ == TokenType::If {
					return self.if_();
				}
				Some(self.block()?)
			} else {
				None
			};

			return Ok(Expr::If(expr, then_block, else_block));
		}

		self.loop_()
	}

	/// loop => "loop" block
	fn loop_(&mut self) -> Result<Expr, ParserError> {
		if self.matches(TokenType::Loop) {
			return Ok(Expr::Loop(self.block()?));
		}

		self.logic_or()
	}

	/// logic_or => logic_and ("&&" logic_and)*
	fn logic_or(&mut self) -> Result<Expr, ParserError> {
		let mut left = self.logic_and()?;

		while self.matches(TokenType::DoublePipe) {
			let right = Box::new(self.logic_and()?);
			left = Expr::Logical(Box::new(left), LogicalOp::Or, right)
		}

		Ok(left)
	}

	/// logic_and => equality ("&&" equality)*
	fn logic_and(&mut self) -> Result<Expr, ParserError> {
		let mut left = self.equality()?;

		while self.matches(TokenType::DoubleAmpersand) {
			let right = Box::new(self.equality()?);
			left = Expr::Logical(Box::new(left), LogicalOp::And, right)
		}

		Ok(left)
	}

	/// equality => comparison (("==" | "!=") comparison)*
	fn equality(&mut self) -> Result<Expr, ParserError> {
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
	fn comparison(&mut self) -> Result<Expr, ParserError> {
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
	fn term(&mut self) -> Result<Expr, ParserError> {
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
	fn factor(&mut self) -> Result<Expr, ParserError> {
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

	/// unary => ("!" | "-") (unary | call)
	fn unary(&mut self) -> Result<Expr, ParserError> {
		if self.matches(TokenType::Bang) {
			let expr = Box::new(self.expr()?);
			return Ok(Expr::Unary(UnaryOp::Not, expr));
		}
		if self.matches(TokenType::Minus) {
			let expr = Box::new(self.expr()?);
			return Ok(Expr::Unary(UnaryOp::Neg, expr));
		}
		self.call()
	}

	/// call => primary "(" arguments? ")"
	fn call(&mut self) -> Result<Expr, ParserError> {
		let expr = self.primary()?;

		if self.matches(TokenType::LeftParen) {
			self.expect(
				TokenType::RightParen,
				"Expected closing `)` after arguments",
			)?;
			return Ok(Expr::Call(Box::new(expr), Vec::new()));
		}

		Ok(expr)
	}

	/// primary => "(" expr ")", block, identifier, number | string | "true" | "false"
	fn primary(&mut self) -> Result<Expr, ParserError> {
		if self.peek_eq(TokenType::LeftBrace) {
			return self.block().map(Expr::Block);
		}

		if self.matches(TokenType::LeftParen) {
			let expr = Box::new(self.expr()?);
			self.expect(TokenType::RightParen, "Expected closing `)`")?;
			return Ok(Expr::Group(expr));
		}

		match self.matches_which(&[
			TokenType::False,
			TokenType::True,
			TokenType::Number,
			TokenType::String,
			TokenType::Identifier,
		]) {
			Some(Token {
				typ,
				literal,
				ident,
				..
			}) => Ok(match typ {
				TokenType::False => Expr::Literal(Literal::Bool(false)),
				TokenType::True => Expr::Literal(Literal::Bool(true)),
				TokenType::Number | TokenType::String => Expr::Literal(
					literal
						.clone()
						.expect("Internal Error: Literal token should have a value!"),
				),
				TokenType::Identifier => Expr::Identifier(
					ident
						.clone()
						.expect("Internal Error: Identifier token should have a value!"),
				),
				_ => unreachable!(),
			}),
			None => ParserError::token_mismatch(self.advance(), "Expected Literal"),
		}
	}

	/// block => "{" stmt* "}"
	fn block(&mut self) -> Result<Block, ParserError> {
		self.expect(TokenType::LeftBrace, "Expected `{`")?;

		let mut stmts = Vec::new();
		let closed = loop {
			if self.matches(TokenType::RightBrace) {
				break true;
			}
			if self.matches(TokenType::Eof) {
				break false;
			}
			let stmt = self.stmt()?;
			stmts.push(stmt);
		};

		if closed {
			Ok(Block { stmts })
		} else {
			ParserError::token_mismatch(self.previous(), "Unclosed block, expected `}`")
		}
	}
}

impl Parser {
	fn expect(&mut self, typ: TokenType, error_msg: &str) -> Result<&Token, ParserError> {
		if self.matches(typ) {
			return Ok(self.previous());
		}

		ParserError::token_mismatch(self.advance(), error_msg)
	}

	// fn expect_any(
	// 	&mut self,
	// 	types: &[TokenType],
	// 	error_msg: &str,
	// ) -> Result<&Token, ParserError> {
	// 	if self.matches_any(types) {
	// 		return Ok(self.previous());
	// 	}

	// 	ParserError::token_mismatch(self.advance(), error_msg)
	// }

	fn matches_any(&mut self, types: &[TokenType]) -> bool {
		for typ in types {
			if self.matches(typ.clone()) {
				return true;
			}
		}

		false
	}

	fn matches_which(&mut self, types: &[TokenType]) -> Option<&Token> {
		for typ in types {
			if self.matches(typ.clone()) {
				return Some(self.previous());
			}
		}

		None
	}

	fn peek_eq(&mut self, typ: TokenType) -> bool {
		self.peek(0).typ == typ
	}

	fn matches(&mut self, typ: TokenType) -> bool {
		if self.peek(0).typ == typ {
			self.advance();
			return true;
		}
		false
	}

	fn advance(&mut self) -> &Token {
		self.pos += 1;
		self.previous()
	}

	fn previous(&self) -> &Token {
		&self.tokens[self.pos - 1]
	}

	fn is_at_end(&self) -> bool {
		if self.peek(0).typ == TokenType::Eof {
			return true;
		}
		false
	}

	fn peek(&self, dist: usize) -> &Token {
		match self.tokens.get(self.pos + dist) {
			Some(token) => token,
			// TODO: Think about how this could be improved or if its fine
			// Should always return TokenType::Eof
			None => &self.tokens[self.tokens.len() - 1],
		}
	}
}

impl Iterator for Parser {
	type Item = Result<Stmt, ParserError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.tokens.is_empty() || self.is_at_end() {
			return None;
		}
		match self.stmt() {
			Ok(stmt) => Some(Ok(stmt)),
			Err(err) => Some(Err(err)),
		}
	}
}
