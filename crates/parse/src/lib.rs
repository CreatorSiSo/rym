mod error;

use ast::*;

pub use error::ParseError;
// TODO: `ParseResult<T> = Result<T, ParseError>` should become `ParseResult<T> = Result<Spanned<T>, ParseError>` once this is possible
pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
}

impl Parser {
	pub fn parse(tokens: Vec<Token>) -> (Vec<Spanned<Stmt>>, Vec<ParseError>) {
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

	pub const fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}

	// TODO: Refactor entire stmt function (maybe split it up)
	pub fn stmt(&mut self) -> ParseResult<Spanned<Stmt>> {
		if let Some(token) = self.matches_which(&[TokenType::Semicolon, TokenType::Newline]) {
			return Ok(Spanned(token.span, Stmt::Empty));
		}

		if let Some(token) = self.matches_which(&[TokenType::Const, TokenType::Mut]) {
			let mutable = token.typ == TokenType::Mut;

			let name_token = self.expect(TokenType::Identifier, "Expected identifier")?;
			let name = name_token.data.ident(TokenType::Identifier);

			self.expect(TokenType::Equal, "Expected `=`")?;

			let expr = self.parse_expr()?;
			let var_end = self.expect_any(
				&[TokenType::Semicolon, TokenType::Newline],
				"Expected Semicolon or Newline",
			)?;

			return Ok(Spanned(
				token.span.start..var_end.span.end,
				if mutable {
					Stmt::Decl(Decl::Mut(name, expr.1))
				} else {
					Stmt::Decl(Decl::Const(name, expr.1))
				},
			));
		}

		// fn => "fn" identifier()
		if self.matches(TokenType::Fn) {
			let fn_token = self.expect(TokenType::Identifier, "Expected function name")?;
			let name = match fn_token.data {
				TokenData::Identifier(ident) => ident,
				_ => unreachable!("Internal Error: Identifier Token has no value!"),
			};

			if self.matches(TokenType::LeftParen) {
				let params = if self.matches(TokenType::RightParen) {
					Vec::new()
				} else {
					let mut params = Vec::new();

					let first_param = self.expect(TokenType::Identifier, "Expected function parameter")?;
					params.push(first_param.data.ident(TokenType::Identifier));

					while self.matches(TokenType::Comma) {
						let other_param = self.expect(TokenType::Identifier, "Expected function parameter")?;
						params.push(other_param.data.ident(TokenType::Identifier));
					}

					self.expect(
						TokenType::RightParen,
						"Expected closing `)` after parameters",
					)?;

					params
				};
				return Ok(Spanned(
					fn_token.span.start..self.previous().span.end,
					Stmt::Decl(Decl::Fn {
						name,
						params,
						body: self.parse_expr()?,
					}),
				));
			}
		}

		let expr = self.parse_expr()?;
		let start = expr.0.start;
		self.matches_any(&[TokenType::Semicolon, TokenType::Newline]);
		Ok(Spanned(start..self.previous().span.end, Stmt::Expr(expr)))
	}

	fn parse_expr(&mut self) -> ParseResult<Spanned<Expr>> {
		// return => "return" expr?;
		// break => "break" expr?;
		if let Some(Token { typ, span, .. }) =
			self.matches_which(&[TokenType::Break, TokenType::Return])
		{
			let is_return = typ == TokenType::Return;
			if self.matches_any(&[TokenType::Semicolon, TokenType::Newline])
				|| self.peek_eq(0, TokenType::RightBrace)
			{
				return if is_return {
					ParseError::token_mismatch(self.previous(), "Expected expression after `return`")
				} else {
					Ok(Spanned(span, Expr::Break(None)))
				};
			}
			let expr = Box::new(self.parse_expr()?);
			let full_span = span.start..expr.0.end;
			return Ok(Spanned(
				full_span,
				if is_return {
					Expr::Return(expr)
				} else {
					Expr::Break(Some(expr))
				},
			));
		}

		// continue => "continue"
		if self.matches(TokenType::Continue) {
			return Ok(Spanned(self.previous().span, Expr::Continue));
		}

		self.parse_assignment()
	}

	/// assignment => identifier "=" expr
	fn parse_assignment(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.peek_eq(1, TokenType::Equal) {
			let start = self.tokens[self.pos].span.start;
			let expr_l = Box::new(self.parse_primary()?);
			self.advance();
			let expr_r = Box::new(self.parse_expr()?);

			return Ok(Spanned(
				start..self.previous().span.end,
				Expr::Assign(expr_l, expr_r),
			));
		}

		self.parse_if()
	}

	/// if => "if" expression block ("else" (if | block))?
	fn parse_if(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::If) {
			let start = self.previous().span.start;
			let expr = Box::new(self.parse_expr()?);
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
				start..self.previous().span.end,
				Expr::If(expr, then_block, else_block),
			));
		}

		self.parse_loop()
	}

	/// loop => "loop" block
	fn parse_loop(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::Loop) {
			let start = self.previous().span.start;
			let block = self.parse_block()?;
			return Ok(Spanned(start..self.previous().span.end, Expr::Loop(block)));
		}

		self.parse_logic_or()
	}

	/// logic_or => logic_and ("||" logic_and)*
	fn parse_logic_or(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_logic_and()?;

		while self.matches(TokenType::DoublePipe) {
			let right = Box::new(self.parse_logic_and()?);
			left.1 = Expr::Logical(Box::new(left.clone()), LogicalOp::Or, right);
			left.0.end = self.previous().span.end;
		}

		Ok(left)
	}

	/// logic_and => equality ("&&" equality)*
	fn parse_logic_and(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_equality()?;

		while self.matches(TokenType::DoubleAmpersand) {
			let right = Box::new(self.parse_equality()?);
			left.1 = Expr::Logical(Box::new(left.clone()), LogicalOp::And, right);
			left.0.end = self.previous().span.end;
		}

		Ok(left)
	}

	/// equality => comparison (("==" | "!=") comparison)*
	fn parse_equality(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_comparison()?;

		while self.matches_any(&[TokenType::EqualEqual, TokenType::BangEqual]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.parse_comparison()?);

			left.1 = if typ == TokenType::EqualEqual {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Eq, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Ne, right)
			};
			left.0.end = self.previous().span.end;
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
			let typ = self.previous().typ.clone();
			let right = Box::new(self.parse_term()?);

			left.1 = Expr::Binary(
				Box::new(left.clone()),
				match typ {
					TokenType::Greater => BinaryOp::Gt,
					TokenType::GreaterEqual => BinaryOp::Ge,
					TokenType::Less => BinaryOp::Lt,
					_ => BinaryOp::Le,
				},
				right,
			);
			left.0.end = self.previous().span.end;
		}

		Ok(left)
	}

	/// term => factor (("+" | "-") factor)*
	fn parse_term(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.parse_factor()?);

			left.1 = if typ == TokenType::Plus {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Add, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Sub, right)
			};
			left.0.end = self.previous().span.end;
		}

		Ok(left)
	}

	/// factor => unary (("/" | "*") unary)*
	fn parse_factor(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut left = self.parse_unary()?;

		while self.matches_any(&[TokenType::Star, TokenType::Slash]) {
			let typ = self.previous().typ.clone();
			let right = Box::new(self.parse_unary()?);

			left.1 = if typ == TokenType::Star {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Mul, right)
			} else {
				Expr::Binary(Box::new(left.clone()), BinaryOp::Div, right)
			};
			left.0.end = self.previous().span.end;
		}

		Ok(left)
	}

	/// unary => ("!" | "-") (unary | call)
	fn parse_unary(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.matches(TokenType::Bang) {
			let start = self.previous().span.start;
			let expr = Box::new(self.parse_unary()?);
			return Ok(Spanned(start..expr.0.end, Expr::Unary(UnaryOp::Not, expr)));
		}
		if self.matches(TokenType::Minus) {
			let start = self.previous().span.start;
			let expr = Box::new(self.parse_unary()?);
			return Ok(Spanned(start..expr.0.end, Expr::Unary(UnaryOp::Neg, expr)));
		}
		self.parse_call()
	}

	/// call => primary ("(" arguments? ")")*
	fn parse_call(&mut self) -> ParseResult<Spanned<Expr>> {
		let mut expr = self.parse_primary()?;

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
				expr.0.start..self.previous().span.end,
				Expr::Call {
					callee: Box::new(expr.clone()),
					args,
				},
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
	fn parse_primary(&mut self) -> ParseResult<Spanned<Expr>> {
		if self.peek_eq(0, TokenType::LeftBrace) {
			let start = self.previous().span.start;
			return Ok(Spanned(
				start..self.previous().span.end,
				Expr::Block(self.parse_block()?),
			));
		}

		if self.matches(TokenType::LeftParen) {
			let start = self.previous().span.start;
			let expr = if self.peek_eq(0, TokenType::RightParen) {
				Spanned(
					start..self.previous().span.end,
					Expr::Literal(Literal::Unit),
				)
			} else {
				self.parse_expr()?
			};
			self.expect(TokenType::RightParen, "Expected closing `)`")?;
			return Ok(Spanned(
				start..self.previous().span.end,
				Expr::Group(Box::new(expr)),
			));
		}

		if let Some(Token { typ, data, span }) = self.matches_which(&[
			TokenType::False,
			TokenType::True,
			TokenType::Number,
			TokenType::String,
			TokenType::Identifier,
		]) {
			Ok(Spanned(
				span,
				match typ {
					TokenType::False => Expr::Literal(Literal::Bool(false)),
					TokenType::True => Expr::Literal(Literal::Bool(true)),
					TokenType::Number | TokenType::String => Expr::Literal(data.lit(typ)),
					TokenType::Identifier => Expr::Identifier(data.ident(typ)),
					got => unreachable!("{got}"),
				},
			))
		} else {
			ParseError::token_mismatch(self.advance(), "Expected Literal")
		}
	}

	/// block => "{" stmt* "}"
	fn parse_block(&mut self) -> ParseResult<Spanned<Block>> {
		self.expect(TokenType::LeftBrace, "Expected `{`")?;
		let start = self.previous().span.start;

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
			Ok(Spanned(start..self.previous().span.end, stmts))
		} else {
			ParseError::token_mismatch(self.previous(), "Unclosed block, expected `}`")
		}
	}
}

impl Parser {
	fn expect_any(&mut self, types: &[TokenType], error_msg: &str) -> ParseResult<Token> {
		for typ in types {
			if self.matches(typ.clone()) {
				return Ok(self.previous());
			}
		}
		ParseError::token_mismatch(self.advance(), error_msg)
	}

	fn expect(&mut self, typ: TokenType, error_msg: &str) -> ParseResult<Token> {
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

	fn matches_which(&mut self, types: &[TokenType]) -> Option<Token> {
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

	fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.pos += 1;
		}
		self.previous()
	}

	fn previous(&self) -> Token {
		// TODO: Is it possible to avoid cloning here and tell rust that this cant be a mutable borrow
		// (but is currently without this clone seen as one when you have nested mutable calls to eg. advance or matches)
		self.tokens[self.pos - 1].clone()
	}

	fn is_at_end(&self) -> bool {
		matches!(self.peek(0), None)
	}

	fn peek_eq(&mut self, dist: usize, typ: TokenType) -> bool {
		match self.peek(dist) {
			Some(Token {
				typ: peeked_typ, ..
			}) => peeked_typ == &typ,
			None => false,
		}
	}

	fn peek(&self, dist: usize) -> Option<&Token> {
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
