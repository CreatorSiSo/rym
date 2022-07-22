mod env;
mod error;
use env::Env;
use error::RuntimeError;
use rym_ast::{Expr, Literal, Local, Stmt};

pub struct Interpreter<'src> {
	env: Env<'src>,
}

impl<'src> Interpreter<'src> {
	pub fn new() -> Self {
		Self { env: Env::new() }
	}

	pub fn eval(&mut self, ast: &'src [Stmt]) -> Result<(), RuntimeError> {
		for stmt in ast {
			match stmt {
				Stmt::Local(local) => {
					self.local(local)?;
				}
				Stmt::Expr(expr) => {
					self.expr(expr)?;
				}
				Stmt::Empty => {}
			}
		}
		Ok(())
	}

	pub fn local(&mut self, local: &'src Local) -> Result<(), RuntimeError> {
		match local {
			Local::Const(name, init) => {
				self.env.declare(name, self.expr(init)?, false);
			}
			Local::Mut(name, init) => {
				self.env.declare(name, self.expr(init)?, false);
			}
		}
		Ok(())
	}

	fn expr<'expr>(&self, expr: &'expr Expr<'src>) -> Result<&'expr Literal<'src>, RuntimeError> {
		match expr {
			Expr::Binary(left, op, right) => {
				let result_left = self.expr(left)?;
				let result_right = self.expr(right)?;
				todo!()
			}
			Expr::Unary(_, _) => todo!(),
			Expr::Group(_) => todo!(),
			Expr::Literal(literal) => Ok(literal),
			_ => todo!(),
		}
	}
}
