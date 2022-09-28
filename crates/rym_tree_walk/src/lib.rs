#![allow(clippy::new_without_default)]

mod env;
mod error;
use env::Env;
use error::RuntimeError;
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

	fn stmt(&mut self, stmt: &Stmt<'src>) -> Result<Literal<'src>, RuntimeError> {
		match stmt {
			Stmt::Local(local) => {
				self.local(local)?;
			}
			Stmt::Print(expr) => {
				let lit = self.expr(expr)?;
				if let Literal::Identifier(name) = lit {
					println!("{}", self.env.get(name)?);
				} else {
					println!("{lit}")
				}
			}
			Stmt::Expr(expr) => return self.expr(expr),
			Stmt::Empty => {}
		}
		Ok(Literal::Tuple)
	}

	fn local(&mut self, local: &Local<'src>) -> Result<(), RuntimeError> {
		match local {
			Local::Const(name, init) => {
				let val = self.expr(init)?;
				self.env.declare(name, val.clone(), false);
				println!("const {name} = {val:?}")
			}
			Local::Mut(name, init) => {
				let val = self.expr(init)?;
				self.env.declare(name, val.clone(), true);
				println!("mut {name} = {val:?}")
			}
		}
		Ok(())
	}

	fn expr(&mut self, expr: &Expr<'src>) -> Result<Literal<'src>, RuntimeError> {
		match expr {
			Expr::Literal(literal) => match literal {
				Literal::Identifier(identifier) => Ok(self.env.get(identifier).cloned()?),
				_ => Ok(literal.clone()),
			},
			Expr::Group(expr) => self.expr(expr),
			Expr::Unary(op, expr) => self.unary(op, expr),
			Expr::Logical(left, LogicalOp::And, right) => self.logical(left, LogicalOp::And, right),
			Expr::Logical(left, LogicalOp::Or, right) => self.logical(left, LogicalOp::Or, right),
			Expr::Binary(left, op, right) => self.binary(left, op, right),
			Expr::Block(block) => self.block(block),
			Expr::If(expr, then_block, else_block) => self.if_(expr, then_block, else_block),
			_ => panic!("Not yet implemented: {:?}", expr),
		}
	}

	fn if_(
		&mut self,
		expr: &Expr<'src>,
		then_block: &Block<'src>,
		else_block: &Option<Block<'src>>,
	) -> Result<Literal<'src>, RuntimeError> {
		let bool = match self.expr(expr)? {
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
			Ok(Literal::Tuple)
		};
	}

	fn block(&mut self, block: &Block<'src>) -> Result<Literal<'src>, RuntimeError> {
		if let Some((last, prev)) = block.stmts.split_last() {
			self.env.push_scope();
			for stmt in prev {
				self.stmt(stmt)?;
			}
			let return_value = self.stmt(last)?;

			self.env.pop_scope();
			Ok(return_value)
		} else {
			Ok(Literal::Tuple)
		}
	}

	fn unary(&mut self, op: &UnaryOp, expr: &Expr<'src>) -> Result<Literal<'src>, RuntimeError> {
		let lit = self.expr(expr)?;
		match (op, lit) {
			(UnaryOp::Not, Literal::Bool(val)) => Ok(Literal::Bool(!val)),
			(UnaryOp::Neg, Literal::Number(val)) => Ok(Literal::Number(-val)),
			(op, lit) => RuntimeError::unary(op, lit),
		}
	}

	// TODO: Make this easily understandable
	fn logical(
		&mut self,
		expr_l: &Expr<'src>,
		op: LogicalOp,
		expr_r: &Expr<'src>,
	) -> Result<Literal<'src>, RuntimeError> {
		let lit_l = self.expr(expr_l)?;

		if op == LogicalOp::And {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l && val_r, false)
		} else {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l || val_r, true)
		}
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
			let lit_r = self.expr(expr_r)?;
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
	) -> Result<Literal<'src>, RuntimeError> {
		let lit_l = self.expr(expr_l)?;
		let lit_r = self.expr(expr_r)?;

		match op {
			BinaryOp::Eq => Ok(Literal::from(lit_l == lit_r)),
			BinaryOp::Ne => Ok(Literal::from(lit_l != lit_r)),
			BinaryOp::Gt => Self::number(lit_l, lit_r, |val_l, val_r| val_l > val_r),
			BinaryOp::Ge => Self::number(lit_l, lit_r, |val_l, val_r| val_l >= val_r),
			BinaryOp::Lt => Self::number(lit_l, lit_r, |val_l, val_r| val_l < val_r),
			BinaryOp::Le => Self::number(lit_l, lit_r, |val_l, val_r| val_l <= val_r),
			BinaryOp::Mul => Self::number(lit_l, lit_r, |val_l, val_r| val_l * val_r),
			BinaryOp::Div => Self::number(lit_l, lit_r, |val_l, val_r| val_l / val_r),
			BinaryOp::Mod => Self::number(lit_l, lit_r, |val_l, val_r| val_l % val_r),
			BinaryOp::Sub => Self::number(lit_l, lit_r, |val_l, val_r| val_l - val_r),
			BinaryOp::Add => Self::number(lit_l, lit_r, |val_l, val_r| val_l + val_r),
		}
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
