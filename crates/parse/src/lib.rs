mod error;

use ast::*;

pub use error::ParseError;
// TODO: `ParseResult<T> = Result<T, ParseError>` should become `ParseResult<T> = Result<Spanned<T>, ParseError>` once this is possible
pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
	tokens: Vec<Spanned<Token>>,
	pos: usize,
}

impl Parser {
	pub fn parse(tokens: Vec<Spanned<Token>>) -> (Vec<Spanned<Stmt>>, Vec<ParseError>) {
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

	pub const fn new(tokens: Vec<Spanned<Token>>) -> Self {
		Self { tokens, pos: 0 }
	}

	// TODO: Refactor entire stmt function (maybe split it up)
	pub fn stmt(&mut self) -> ParseResult<Spanned<Stmt>> {
		if let Some(Spanned(_, newline_span)) =
			self.matches_which(&[TokenType::Semicolon, TokenType::Newline])
		{
			return Ok(Spanned(Stmt::Empty, newline_span));
		}
		let stmt_start_pos = self.pos;

		if let Some(Spanned(token, var_start_span)) =
			self.matches_which(&[TokenType::Const, TokenType::Mut])
		{
			let mutable = token.typ == TokenType::Mut;

			let Spanned(name_token, _) = self.expect(TokenType::Identifier, "Expected identifier")?;
			let name = name_token.data.ident(TokenType::Identifier);

			self.expect(TokenType::Equal, "Expected `=`")?;

			let expr = self.parse_expr()?;
			let Spanned(_, var_end_span) = self.expect_any(
				&[TokenType::Semicolon, TokenType::Newline],
				"Expected Semicolon or Newline",
			)?;

			return Ok(Spanned(
				if mutable {
					Stmt::Decl(Decl::Mut(name, expr.0))
				} else {
					Stmt::Decl(Decl::Const(name, expr.0))
				},
				var_start_span.start..var_end_span.end,
			));
		}

		// fn => "fn" identifier()
		if self.matches(TokenType::Fn) {
			let Spanned(fn_token, fn_start_span) =
				self.expect(TokenType::Identifier, "Expected function name")?;
			let name = match fn_token.data {
				TokenData::Identifier(ident) => ident,
				_ => unreachable!("Internal Error: Identifier Token has no value!"),
			};

			if self.matches(TokenType::LeftParen) {
				let params = if self.matches(TokenType::RightParen) {
					Vec::new()
				} else {
					let mut params = Vec::new();

					let Spanned(first_param, _) =
						self.expect(TokenType::Identifier, "Expected function parameter")?;
					params.push(first_param.data.ident(TokenType::Identifier));

					while self.matches(TokenType::Comma) {
						let Spanned(other_param, _) =
							self.expect(TokenType::Identifier, "Expected function parameter")?;
						params.push(other_param.data.ident(TokenType::Identifier));
					}

					self.expect(
						TokenType::RightParen,
						"Expected closing `)` after parameters",
					)?;

					params
				};
				return Ok(Spanned(
					Stmt::Decl(Decl::Fn {
						name,
						params,
						body: self.parse_expr()?,
					}),
					fn_start_span.start..self.previous_span().end,
				));
			}
		}

		let expr = self.parse_expr()?;
		self.matches_any(&[TokenType::Semicolon, TokenType::Newline]);
		Ok(Spanned(
			Stmt::Expr(expr.0),
			stmt_start_pos..self.previous_span().end,
		))
	}

	fn parse_expr(&mut self) -> ParseResult<Spanned<Expr>> {
		// return => "return" expr?;
		// break => "break" expr?;
		if let Some(Spanned(Token { typ, .. }, span)) =
			self.matches_which(&[TokenType::Break, TokenType::Return])
		{
			let is_return = typ == TokenType::Return;
			if self.matches_any(&[TokenType::Semicolon, TokenType::Newline])
				|| self.peek_eq(0, TokenType::RightBrace)
			{
				return if is_return {
					ParseError::token_mismatch(self.previous(), "Expected expression after `return`")
				} else {
					Ok(Spanned(Expr::Break(None), span))
				};
			}
			let expr = Box::new(self.parse_expr()?);
			let full_span = span.start..expr.1.end;
			return Ok(Spanned(
				if is_return {
					Expr::Return(expr)
				} else {
					Expr::Break(Some(expr))
				},
				full_span,
			));
		}

		// continue => "continue"
		if self.matches(TokenType::Continue) {
			return Ok(self.previous().map(|_| Expr::Continue));
		}

		self.parse_assignment()
	}

	/// assignment => identifier "=" expr
	fn parse_assignment(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.peek_eq(1, TokenType::Equal) {
			let start = self.tokens[self.pos].1.start;
			let expr_l = Box::new(self.parse_primary()?);
			self.advance();
			let expr_r = Box::new(self.parse_expr()?.0);

			return Ok(Spanned(
				Expr::Assign(expr_l, expr_r),
				start..self.previous_span().end,
			));
		}

		self.parse_if()
	}

	/// if => "if" expression block ("else" (if | block))?
	fn parse_if(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::If) {
			let start = self.previous_span().start;
			let expr = Box::new(self.parse_expr()?.0);
			let then_block = self.parse_block()?;
			let else_block = if self.matches(TokenType::Else) {
				if self.peek_eq(0, TokenType::If) {
					return self.parse_if();
				}
				Some(self.parse_block()?)
			} else {
				None
			};

			return Ok(Spanned(
				Expr::If(expr, then_block, else_block),
				start..self.previous_span().end,
			));
		}

		self.parse_loop()
	}

	/// loop => "loop" block
	fn parse_loop(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::Loop) {
			let start = self.previous_span().start;
			let block = self.parse_block()?;
			return Ok(Spanned(Expr::Loop(block), start..self.previous_span().end));
		}

		self.parse_logic_or()
	}

	/// logic_or => logic_and ("||" logic_and)*
	fn parse_logic_or(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_logic_and()?;

		while self.matches(TokenType::DoublePipe) {
			let right = Box::new(self.parse_logic_and()?);
			left.0 = Expr::Logical(Box::new(left.clone()), LogicalOp::Or, right);
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// logic_and => equality ("&&" equality)*
	fn parse_logic_and(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_equality()?;

		while self.matches(TokenType::DoubleAmpersand) {
			let right = Box::new(self.parse_equality()?);
			left.0 = Expr::Logical(Box::new(left.clone()), LogicalOp::And, right);
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// equality => comparison (("==" | "!=") comparison)*
	fn parse_equality(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_comparison()?;

		while self.matches_any(&[TokenType::EqualEqual, TokenType::BangEqual]) {
			let typ = self.previous().0.typ.clone();
			let right = Box::new(self.parse_comparison()?);

			left.0 = if typ == TokenType::EqualEqual {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Eq, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Ne, right)
			};
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// comparison => term ((">" | ">=" | "<" | "<=") term)*
	fn parse_comparison(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_term()?;

		while self.matches_any(&[
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		]) {
			let typ = self.previous().0.typ.clone();
			let right = Box::new(self.parse_term()?);

			left.0 = Expr::Binary(
				Box::new(left.clone()),
				match typ {
					TokenType::Greater => BinaryOp::Gt,
					TokenType::GreaterEqual => BinaryOp::Ge,
					TokenType::Less => BinaryOp::Lt,
					_ => BinaryOp::Le,
				},
				right,
			);
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// term => factor (("+" | "-") factor)*
	fn parse_term(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let typ = self.previous().0.typ.clone();
			let right = Box::new(self.parse_factor()?);

			left.0 = if typ == TokenType::Plus {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Add, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Sub, right)
			};
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// factor => unary (("/" | "*") unary)*
	fn parse_factor(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_unary()?;

		while self.matches_any(&[TokenType::Star, TokenType::Slash]) {
			let typ = self.previous().0.typ.clone();
			let right = Box::new(self.parse_unary()?);

			left.0 = if typ == TokenType::Star {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Mul, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Div, right)
			};
			left.1.end = self.previous_span().end;
		}

		Ok(left)
	}

	/// unary => ("!" | "-") (unary | call)
	fn parse_unary(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::Bang) {
			let start = self.previous_span().start;
			let expr = Box::new(self.parse_unary()?);
			return Ok(Spanned(
				Expr::Unary(UnaryOp::Not, expr.clone()),
				start..expr.1.end,
			));
		}
		if self.matches(TokenType::Minus) {
			let start = self.previous_span().start;
			let expr = Box::new(self.parse_unary()?);
			return Ok(Spanned(
				Expr::Unary(UnaryOp::Neg, expr.clone()),
				start..expr.1.end,
			));
		}
		self.parse_call()
	}

	/// call => primary ("(" arguments? ")")*
	fn parse_call(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut expr = Spanned(self.parse_primary()?, 0..0);

		while self.matches(TokenType::LeftParen) {
			let args = if self.matches(TokenType::RightParen) {
				Vec::new()
			} else {
				let args = self.parse_arguments()?;
				self.expect(
					TokenType::RightParen,
					"Expected closing `)` after arguments",
				)?;
				args
			};
			expr = Spanned(
				Expr::Call(Box::new(expr.clone()), args),
				expr.1.start..self.previous_span().end,
			);
		}

		Ok(expr)
	}

	/// expr ("," expr)*
	fn parse_arguments(&mut self) -> ParseResult<Vec<Spanned<Expr>>> {
		let mut args = Vec::new();
		args.push(self.parse_expr()?);
		while self.matches(TokenType::Comma) {
			let expr = self.parse_expr()?;
			args.push(expr);
		}
		Ok(args)
	}

	/// primary => "(" expr? ")", block, identifier, number | string | "true" | "false"
	fn parse_primary(&mut self) -> ParseResult<Expr> {
		if self.peek_eq(0, TokenType::LeftBrace) {
			return self.parse_block().map(Expr::Block);
		}

		if self.matches(TokenType::LeftParen) {
			let expr = if self.peek_eq(0, TokenType::RightParen) {
				Expr::Literal(Literal::Unit)
			} else {
				self.parse_expr()?.0
			};
			self.expect(TokenType::RightParen, "Expected closing `)`")?;
			return Ok(Expr::Group(Box::new(Spanned(expr, 0..0))));
		}

		match self.matches_which(&[
			TokenType::False,
			TokenType::True,
			TokenType::Number,
			TokenType::String,
			TokenType::Identifier,
		]) {
			Some(Spanned(Token { typ, data }, _)) => Ok(match typ {
				TokenType::False => Expr::Literal(Literal::Bool(false)),
				TokenType::True => Expr::Literal(Literal::Bool(true)),
				TokenType::Number | TokenType::String => Expr::Literal(data.lit(typ)),
				TokenType::Identifier => Expr::Identifier(data.ident(typ)),
				got => unreachable!("{got}"),
			}),
			None => ParseError::token_mismatch(self.advance(), "Expected Literal"),
		}
	}

	/// block => "{" stmt* "}"
	fn parse_block(&mut self) -> ParseResult<Block> {
		self.expect(TokenType::LeftBrace, "Expected `{`")?;

		let mut stmts = Vec::new();
		let closed = loop {
			if self.matches(TokenType::RightBrace) {
				break true;
			}
			if self.is_at_end() {
				break false;
			}
			stmts.push(self.stmt()?);
		};

		if closed {
			Ok(Block { stmts })
		} else {
			ParseError::token_mismatch(self.previous(), "Unclosed block, expected `}`")
		}
	}
}

impl Parser {
	fn expect_any(&mut self, types: &[TokenType], error_msg: &str) -> ParseResult<Spanned<Token>> {
		for typ in types {
			if self.matches(typ.clone()) {
				return Ok(self.previous());
			}
		}
		ParseError::token_mismatch(self.advance(), error_msg)
	}

	fn expect(&mut self, typ: TokenType, error_msg: &str) -> ParseResult<Spanned<Token>> {
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

	fn matches_which(&mut self, types: &[TokenType]) -> Option<Spanned<Token>> {
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

	fn advance(&mut self) -> Spanned<Token> {
		if !self.is_at_end() {
			self.pos += 1;
		}
		self.previous()
	}

	fn previous_span(&self) -> &Span {
		&self.tokens[self.pos - 1].1
	}

	fn previous(&self) -> Spanned<Token> {
		// TODO: Is it possible to avoid cloning here and tell rust that this cant be a mutable borrow
		// (but is currently without this clone seen as one when you have nested mutable calls to eg. advance or matches)
		self.tokens[self.pos - 1].clone()
	}

	fn is_at_end(&self) -> bool {
		matches!(self.peek(0), None)
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

	fn peek(&self, dist: usize) -> Option<&Spanned<Token>> {
		self.tokens.get(self.pos + dist)
	}
}

pub struct AstIter {
	parser: Parser,
}

impl Iterator for AstIter {
	type Item = ParseResult<Spanned<Stmt>>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.parser.tokens.is_empty() || self.parser.is_at_end() {
			return None;
		}
		match self.parser.stmt() {
			Ok(stmt) => Some(Ok(stmt)),
			Err(err) => Some(Err(err)),
		}
	}
}

impl IntoIterator for Parser {
	type Item = ParseResult<Spanned<Stmt>>;
	type IntoIter = AstIter;

	fn into_iter(self) -> Self::IntoIter {
		AstIter { parser: self }
	}
}
