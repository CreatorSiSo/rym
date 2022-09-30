#![allow(clippy::new_without_default)]

mod env;
mod error;
mod interrupt;
use env::Env;
use error::RuntimeError;
use interrupt::Inter;
use rym_ast::{BinaryOp, Block, Expr, Literal, Local, LogicalOp, Stmt, UnaryOp};

pub struct Interpreter<'src> {
	env: Env<'src>,
}

impl<'src> Interpreter<'src> {
	pub fn new() -> Self {
		Self { env: Env::new() }
	}

	pub fn eval(&mut self, ast: &'src [Stmt]) -> Result<(), RuntimeError> {
		for stmt in ast {
			self.stmt(stmt)?;
		}
		Ok(())
	}

	fn stmt(&mut self, stmt: &Stmt<'src>) -> Result<Inter<'src>, RuntimeError> {
		match stmt {
			Stmt::Local(local) => {
				self.local(local)?;
			}
			Stmt::Print(expr) => {
				let lit = self.expr(expr)?.into();
				if let Literal::Identifier(name) = lit {
					println!("{}", self.env.get(name)?);
				} else {
					println!("{lit}")
				}
			}
			Stmt::Expr(expr) => return self.expr(expr),
			Stmt::Empty => {}
		}
		Ok(Inter::None(Literal::Unit))
	}

	fn local(&mut self, local: &Local<'src>) -> Result<(), RuntimeError> {
		match local {
			Local::Const(name, init) => {
				let val: Literal = self.expr(init)?.into();
				self.env.declare(name, val.clone(), true);
				println!("const {name} = {val:?}")
			}
			Local::Mut(name, init) => {
				let val: Literal = self.expr(init)?.into();
				self.env.declare(name, val.clone(), false);
				println!("mut {name} = {val:?}")
			}
		}
		Ok(())
	}

	fn expr(&mut self, expr: &Expr<'src>) -> Result<Inter<'src>, RuntimeError> {
		match expr {
			Expr::Literal(literal) => Ok(Inter::None(match literal {
				Literal::Identifier(identifier) => self.env.get(identifier).cloned()?,
				_ => literal.clone(),
			})),
			Expr::Assign(left, right) => self.assign(left, right),

			Expr::Unary(op, expr) => self.unary(op, expr),
			Expr::Logical(left, op, right) => self.logical(left, op, right),
			Expr::Binary(left, op, right) => self.binary(left, op, right),

			Expr::Group(expr) => self.expr(expr),
			Expr::Block(block) => self.block(block),
			Expr::If(expr, then_block, else_block) => self.if_(expr, then_block, else_block),
			Expr::Loop(block) => self.loop_(block),

			Expr::Break(_) => Ok(Inter::Break(Literal::Unit)),
			Expr::Continue => Ok(Inter::Continue),

			_ => panic!("Not yet implemented: {:?}", expr),
		}
	}

	fn if_(
		&mut self,
		expr: &Expr<'src>,
		then_block: &Block<'src>,
		else_block: &Option<Block<'src>>,
	) -> Result<Inter<'src>, RuntimeError> {
		let bool = match self.expr(expr)?.into() {
			Literal::Bool(b) => b,
			// Identifier has already been resolved in self.expr()
			Literal::Identifier(_) => unreachable!(),
			result => return RuntimeError::expected("bool", result),
		};

		return if bool {
			self.block(then_block)
		} else if let Some(block) = else_block {
			self.block(block)
		} else {
			Ok(Inter::None(Literal::Unit))
		};
	}

	// TODO: Implement break, continue and return
	fn loop_(&mut self, block: &Block<'src>) -> Result<Inter<'src>, RuntimeError> {
		loop {
			match self.block(block)? {
				Inter::Break(lit) => break Ok(Inter::None(lit)),
				_ => continue,
			}
		}
	}

	fn block(&mut self, block: &Block<'src>) -> Result<Inter<'src>, RuntimeError> {
		self.env.push_scope();

		let mut stmts = block.stmts.iter();
		let return_value = loop {
			let stmt = match stmts.next() {
				Some(stmt) => stmt,
				None => break Inter::None(Literal::Unit),
			};

			let inter = self.stmt(stmt)?;
			match inter {
				Inter::Break(lit) => break Inter::Break(lit),
				_ => continue,
			}

			// TODO: Handle last stmt as result
		};

		self.env.pop_scope();

		Ok(return_value)
	}

	fn assign(
		&mut self,
		expr_l: &Expr<'src>,
		expr_r: &Expr<'src>,
	) -> Result<Inter<'src>, RuntimeError> {
		let name = match expr_l {
			Expr::Literal(Literal::Identifier(name)) => name,
			_ => return RuntimeError::expected("identfier", self.expr(expr_l)?.into()),
		};
		let value = self.expr(expr_r)?.into();
		self.env.set(name, value)?;

		Ok(Inter::None(Literal::Unit))
	}

	fn unary(&mut self, op: &UnaryOp, expr: &Expr<'src>) -> Result<Inter<'src>, RuntimeError> {
		let lit = self.expr(expr)?.into();

		Ok(Inter::None(match (op, lit) {
			(UnaryOp::Not, Literal::Bool(val)) => Literal::Bool(!val),
			(UnaryOp::Neg, Literal::Number(val)) => Literal::Number(-val),
			(op, lit) => return RuntimeError::unary(op, lit),
		}))
	}

	// TODO: Make this easily understandable
	fn logical(
		&mut self,
		expr_l: &Expr<'src>,
		op: &LogicalOp,
		expr_r: &Expr<'src>,
	) -> Result<Inter<'src>, RuntimeError> {
		let lit_l = self.expr(expr_l)?.into();

		Ok(Inter::None(if op == &LogicalOp::And {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l && val_r, false)?
		} else {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l || val_r, true)?
		}))
	}

	fn cmp_bool<F>(
		&mut self,
		lit_l: Literal<'src>,
		expr_r: &Expr<'src>,
		f: F,
		short_circuit_if: bool,
	) -> Result<Literal<'src>, RuntimeError>
	where
		F: Fn(bool, bool) -> bool,
	{
		if let Literal::Bool(val_l) = lit_l {
			if val_l == short_circuit_if {
				return Ok(Literal::Bool(short_circuit_if));
			}

			let lit_r = self.expr(expr_r)?.into();
			if let Literal::Bool(val_r) = lit_r {
				return Ok(Literal::Bool(f(val_l, val_r)));
			}
			return RuntimeError::comparison(lit_l, lit_r);
		}

		// TODO: How should errors and short circuiting work?
		// Should the second value still be calculated if the first does not result in a bool?
		RuntimeError::comparison(lit_l, Literal::Identifier("not evaluated"))
	}

	// TODO: Assignment expression

	fn binary(
		&mut self,
		expr_l: &Expr<'src>,
		op: &BinaryOp,
		expr_r: &Expr<'src>,
	) -> Result<Inter<'src>, RuntimeError> {
		let lit_l = self.expr(expr_l)?.into();
		let lit_r = self.expr(expr_r)?.into();

		Ok(Inter::None(match op {
			BinaryOp::Eq => Literal::from(lit_l == lit_r),
			BinaryOp::Ne => Literal::from(lit_l != lit_r),
			BinaryOp::Gt => Self::number(lit_l, lit_r, |val_l, val_r| val_l > val_r)?,
			BinaryOp::Ge => Self::number(lit_l, lit_r, |val_l, val_r| val_l >= val_r)?,
			BinaryOp::Lt => Self::number(lit_l, lit_r, |val_l, val_r| val_l < val_r)?,
			BinaryOp::Le => Self::number(lit_l, lit_r, |val_l, val_r| val_l <= val_r)?,
			BinaryOp::Mul => Self::number(lit_l, lit_r, |val_l, val_r| val_l * val_r)?,
			BinaryOp::Div => Self::number(lit_l, lit_r, |val_l, val_r| val_l / val_r)?,
			BinaryOp::Mod => Self::number(lit_l, lit_r, |val_l, val_r| val_l % val_r)?,
			BinaryOp::Sub => Self::number(lit_l, lit_r, |val_l, val_r| val_l - val_r)?,
			BinaryOp::Add => Self::number(lit_l, lit_r, |val_l, val_r| val_l + val_r)?,
		}))
	}

	fn number<F, R>(
		lit_l: Literal<'src>,
		lit_r: Literal<'src>,
		f: F,
	) -> Result<Literal<'src>, RuntimeError>
	where
		F: Fn(f64, f64) -> R,
		Literal<'src>: From<R>,
	{
		if let Literal::Number(val_l) = lit_l {
			if let Literal::Number(val_r) = lit_r {
				return Ok(Literal::from(f(val_l, val_r)));
			}
		}
		RuntimeError::comparison(lit_l, lit_r)
	}
}
