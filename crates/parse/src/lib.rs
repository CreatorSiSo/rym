mod error;

use ast::*;
use error::ParseError;
type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
	tokens: Vec<SpannedToken>,
	pos: usize,
}

impl Parser {
	pub fn parse(tokens: Vec<SpannedToken>) -> (Vec<Stmt>, Vec<ParseError>) {
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

	pub fn new(tokens: Vec<SpannedToken>) -> Self {
		Self { tokens, pos: 0 }
	}

	pub fn stmt(&mut self) -> ParseResult<Stmt> {
		if self.matches(TokenType::Semicolon) {
			return Ok(Stmt::Empty);
		}

		if let Some(token) = self.matches_which(&[TokenType::Const, TokenType::Mut]) {
			let mutable = token.0.typ == TokenType::Mut;

			self.expect(TokenType::Identifier, "Expected identifier")?;
			let name = self
				.previous()
				.0
				.ident
				.clone()
				.expect("Internal Error: Identifier token has no name!")
				.name;

			self.expect(TokenType::Equal, "Expected `=`")?;

			let expr = self.expr()?;
			self.matches(TokenType::Semicolon);
			return Ok(Stmt::Decl(if mutable {
				Decl::Mut(name, expr)
			} else {
				Decl::Const(name, expr)
			}));
		}

		// TODO: Refactor
		// fn => "fn" identifier()
		if self.matches(TokenType::Fn) {
			let name = match self
				.expect(TokenType::Identifier, "Expected function name")?
				.0
				.ident
				.clone()
			{
				Some(ident) => ident.name,
				_ => unreachable!("Internal Error: Identifier Token has no value!"),
			};

			if self.matches(TokenType::LeftParen) {
				let params = if self.matches(TokenType::RightParen) {
					Vec::new()
				} else {
					let mut params = Vec::new();
					params.push(
						self
							.expect(TokenType::Identifier, "Expected function parameter")?
							.0
							.ident
							.clone()
							.expect("Internal Error: Identifier Token has no value!")
							.name,
					);
					while self.matches(TokenType::Comma) {
						let string = self
							.expect(TokenType::Identifier, "Expected function parameter")?
							.0
							.ident
							.clone()
							.expect("Internal Error: Identifier Token has no value!")
							.name;
						params.push(string);
					}
					self.expect(
						TokenType::RightParen,
						"Expected closing `)` after parameters",
					)?;
					params
				};
				return Ok(Stmt::Decl(Decl::Fn(name, params, self.expr()?)));
			}
		}

		let expr = self.expr()?;
		self.matches(TokenType::Semicolon);
		Ok(Stmt::Expr(expr))
	}

	fn expr(&mut self) -> ParseResult<Expr> {
		self.interrupts()
	}

	fn interrupts(&mut self) -> ParseResult<Expr> {
		// return => "return" expr?;
		if self.matches(TokenType::Return) {
			if self.matches(TokenType::Semicolon) {
				return Ok(Expr::Break(Box::new(None)));
			}

			return Ok(Expr::Return(Box::new(self.expr()?)));
		}
		// break => "break" TODO: expr?
		if self.matches(TokenType::Break) {
			return Ok(Expr::Break(Box::new(None)));
		}
		// continue => "continue"
		if self.matches(TokenType::Continue) {
			return Ok(Expr::Continue);
		}

		self.assignment()
	}

	/// assignment => identifier "=" expr
	fn assignment(&mut self) -> ParseResult<Expr> {
		if self.peek_eq(1, TokenType::Equal) {
			let expr_l = Box::new(self.primary()?);
			self.advance();
			let expr_r = Box::new(self.expr()?);

			return Ok(Expr::Assign(expr_l, expr_r));
		}

		self.if_()
	}

	/// if => "if" expression block ("else" (if | block))?
	fn if_(&mut self) -> ParseResult<Expr> {
		if self.matches(TokenType::If) {
			let expr = Box::new(self.expr()?);
			let then_block = self.block()?;
			let else_block = if self.matches(TokenType::Else) {
				if self.peek_eq(0, TokenType::If) {
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
	fn loop_(&mut self) -> ParseResult<Expr> {
		if self.matches(TokenType::Loop) {
			return Ok(Expr::Loop(self.block()?));
		}

		self.logic_or()
	}

	/// logic_or => logic_and ("&&" logic_and)*
	fn logic_or(&mut self) -> ParseResult<Expr> {
		let mut left = self.logic_and()?;

		while self.matches(TokenType::DoublePipe) {
			let right = Box::new(self.logic_and()?);
			left = Expr::Logical(Box::new(left), LogicalOp::Or, right)
		}

		Ok(left)
	}

	/// logic_and => equality ("&&" equality)*
	fn logic_and(&mut self) -> ParseResult<Expr> {
		let mut left = self.equality()?;

		while self.matches(TokenType::DoubleAmpersand) {
			let right = Box::new(self.equality()?);
			left = Expr::Logical(Box::new(left), LogicalOp::And, right)
		}

		Ok(left)
	}

	/// equality => comparison (("==" | "!=") comparison)*
	fn equality(&mut self) -> ParseResult<Expr> {
		let mut left = self.comparison()?;

		while self.matches_any(&[TokenType::EqualEqual, TokenType::BangEqual]) {
			let typ = self.previous().0.typ.clone();
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
	fn comparison(&mut self) -> ParseResult<Expr> {
		let mut left = self.term()?;

		while self.matches_any(&[
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		]) {
			let typ = self.previous().0.typ.clone();
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
	fn term(&mut self) -> ParseResult<Expr> {
		let mut left = self.factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let typ = self.previous().0.typ.clone();
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
	fn factor(&mut self) -> ParseResult<Expr> {
		let mut left = self.unary()?;

		while self.matches_any(&[TokenType::Star, TokenType::Slash]) {
			let typ = self.previous().0.typ.clone();
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
	fn unary(&mut self) -> ParseResult<Expr> {
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
	fn call(&mut self) -> ParseResult<Expr> {
		let expr = self.primary()?;

		if self.matches(TokenType::LeftParen) {
			let args = if self.matches(TokenType::RightParen) {
				Vec::new()
			} else {
				let args = self.arguments()?;
				self.expect(
					TokenType::RightParen,
					"Expected closing `)` after arguments",
				)?;
				args
			};
			return Ok(Expr::Call(Box::new(expr), args));
		}

		Ok(expr)
	}

	/// expr ("," expr)*
	fn arguments(&mut self) -> ParseResult<Vec<Expr>> {
		let mut args = Vec::new();
		args.push(self.expr()?);
		while self.matches(TokenType::Comma) {
			let expr = self.expr()?;
			args.push(expr);
		}
		Ok(args)
	}

	/// primary => "(" expr ")", block, identifier, number | string | "true" | "false"
	fn primary(&mut self) -> ParseResult<Expr> {
		if self.peek_eq(0, TokenType::LeftBrace) {
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
			Some(Spanned(
				Token {
					typ,
					literal,
					ident,
					..
				},
				_,
			)) => Ok(match typ {
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
			None => ParseError::token_mismatch(self.advance(), "Expected Literal"),
		}
	}

	/// block => "{" stmt* "}"
	fn block(&mut self) -> ParseResult<Block> {
		self.expect(TokenType::LeftBrace, "Expected `{`")?;

		let mut stmts = Vec::new();
		let closed = loop {
			if self.matches(TokenType::RightBrace) {
				break true;
			}
			if self.is_at_end() {
				break false;
			}
			let stmt = self.stmt()?;
			stmts.push(stmt);
		};

		if closed {
			Ok(Block { stmts })
		} else {
			ParseError::token_mismatch(self.previous(), "Unclosed block, expected `}`")
		}
	}
}

impl Parser {
	fn expect(&mut self, typ: TokenType, error_msg: &str) -> ParseResult<&SpannedToken> {
		if self.matches(typ) {
			return Ok(self.previous());
		}

		ParseError::token_mismatch(self.advance(), error_msg)
	}

	fn matches_any(&mut self, types: &[TokenType]) -> bool {
		for typ in types {
			if self.matches(typ.clone()) {
				return true;
			}
		}

		false
	}

	fn matches_which(&mut self, types: &[TokenType]) -> Option<&SpannedToken> {
		for typ in types {
			if self.matches(typ.clone()) {
				return Some(self.previous());
			}
		}

		None
	}

	fn matches(&mut self, typ: TokenType) -> bool {
		if self.peek_eq(0, typ) {
			self.advance();
			return true;
		}
		false
	}

	fn advance(&mut self) -> &SpannedToken {
		self.pos += 1;
		self.previous()
	}

	fn previous(&self) -> &SpannedToken {
		&self.tokens[self.pos - 1]
	}

	fn is_at_end(&self) -> bool {
		match self.peek(0) {
			None => true,
			_ => false,
		}
	}

	fn peek_eq(&mut self, dist: usize, typ: TokenType) -> bool {
		match self.peek(dist) {
			Some(Spanned(
				Token {
					typ: peeked_typ, ..
				},
				_,
			)) => peeked_typ == &typ,
			None => false,
		}
	}

	fn peek(&self, dist: usize) -> Option<&SpannedToken> {
		self.tokens.get(self.pos + dist)
	}
}

impl Iterator for Parser {
	type Item = ParseResult<Stmt>;

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
