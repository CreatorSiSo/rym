#![allow(clippy::new_without_default)]

mod env;
mod error;
use env::Env;
use error::RuntimeError;
use rym_ast::{BinaryOp, Expr, Literal, Local, Stmt, UnaryOp};

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

	fn stmt(&mut self, stmt: &Stmt<'src>) -> Result<(), RuntimeError> {
		match stmt {
			Stmt::Local(local) => {
				self.local(local)?;
			}
			Stmt::Expr(expr) => {
				let lit = self.expr(expr)?;
				println!("{lit}");
			}
			Stmt::Empty => {}
		}
		Ok(())
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

	fn expr<'expr>(&self, expr: &'expr Expr<'src>) -> Result<Literal<'src>, RuntimeError> {
		match expr {
			Expr::Group(expr) => self.expr(expr),
			Expr::Literal(literal) => Ok(literal.clone()),
			Expr::Unary(op, expr) => self.unary(op, expr),
			Expr::Binary(left, BinaryOp::And, right) => self.logical(left, BinaryOp::And, right),
			Expr::Binary(left, BinaryOp::Or, right) => self.logical(left, BinaryOp::Or, right),
			Expr::Binary(left, op, right) => self.binary(left, op, right),
			_ => todo!(),
		}
	}

	fn unary(&self, op: &UnaryOp, expr: &Expr) -> Result<Literal<'src>, RuntimeError> {
		let lit = self.expr(expr)?;
		match (op, lit) {
			(UnaryOp::Not, Literal::Bool(val)) => Ok(Literal::Bool(!val)),
			(UnaryOp::Neg, Literal::Number(val)) => Ok(Literal::Number(-val)),
			(op, lit) => RuntimeError::unary(op, lit),
		}
	}

	fn logical(
		&self,
		expr_l: &Expr<'src>,
		op: BinaryOp,
		expr_r: &Expr<'src>,
	) -> Result<Literal<'src>, RuntimeError> {
		let lit_l = self.expr(expr_l)?;

		if op == BinaryOp::And {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l && val_r, false)
		} else {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l || val_r, true)
		}
	}

	fn cmp_bool<F>(
		&self,
		lit_l: Literal<'src>,
		expr_r: &Expr<'src>,
		f: F,
		short_if: bool,
	) -> Result<Literal<'src>, RuntimeError>
	where
		F: Fn(bool, bool) -> bool,
	{
		if let Literal::Bool(val_l) = lit_l {
			if val_l == short_if {
				return Ok(Literal::Bool(short_if));
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

	fn binary(
		&self,
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
			_ => panic!("Internal Error: Should never be reached!"),
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
