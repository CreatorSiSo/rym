#![allow(clippy::new_without_default)]

use std::{io::Write, rc::Rc};

mod callable;
mod env;
mod error;
mod value;

pub use callable::NativeFunction;
pub use error::RuntimeError;
pub use value::{Type, Value};

use ast::{AstVisitor, BinaryOp, Block, Decl, Expr, LogicalOp, Spanned, Stmt, UnaryOp};
use callable::{Callable, RymFunction};
use env::Env;

pub enum Inter {
	Return(Value),
	Break(Value),
	Continue,
	None(Value),
}

pub struct NativeValues {
	print: NativeFunction,
	println: NativeFunction,
	floor: NativeFunction,
	panic: NativeFunction,
}

impl Default for NativeValues {
	fn default() -> Self {
		Self {
			print: NativeFunction::new(
				None,
				Rc::new(|_: _, args: &[Value]| {
					let mut string = String::new();
					for arg in args {
						string.push_str(&arg.to_string())
					}
					// TODO fix print() for repl
					print!("{string}");
					std::io::stdout()
						.flush()
						.expect("Internal Error: Could not flush stout");
					Ok(Value::Unit)
				}),
			),
			println: NativeFunction::new(
				None,
				Rc::new(|_: _, args: &[Value]| {
					let mut string = String::new();
					for arg in args {
						string.push_str(&arg.to_string())
					}
					println!("{string}");
					Ok(Value::Unit)
				}),
			),
			floor: NativeFunction::new(
				Some(1),
				Rc::new(|_: _, args: &[Value]| {
					let val = &args[0];
					if let Value::Number(num) = val {
						Ok(Value::Number(num.floor()))
					} else {
						RuntimeError::expected(Type::Number, val.clone().into())
					}
				}),
			),
			panic: NativeFunction::new(None, Rc::new(|_: _, _: _| Err(RuntimeError::Panic))),
		}
	}
}

impl NativeValues {
	pub fn as_vec<'a>(self) -> Vec<(&'a str, Value)> {
		vec![
			("print", self.print.into()),
			("println", self.println.into()),
			("panic", self.panic.into()),
			("floor", self.floor.into()),
			("PI", std::f64::consts::PI.into()),
			("TAU", std::f64::consts::TAU.into()),
			("E", std::f64::consts::E.into()),
			("SQRT_2", std::f64::consts::SQRT_2.into()),
		]
	}
}

pub struct Interpreter {
	env: Env,
}

impl Default for Interpreter {
	fn default() -> Self {
		Self::with_globals(NativeValues::default().as_vec())
	}
}

impl Interpreter {
	pub fn with_globals(globals: Vec<(&str, Value)>) -> Self {
		Self {
			env: globals
				.into_iter()
				.fold(Env::new(), |mut env, (name, val)| {
					env.declare(name, val, true);
					env
				}),
		}
	}

	pub fn eval(&mut self, ast: &[Spanned<Stmt>]) -> Result<(), RuntimeError> {
		for Spanned(stmt, span) in ast {
			// TODO: Figure out who should own the spans and if they should be cloned here
			self.walk_stmt(&Spanned(stmt, span.clone()))?;
		}
		Ok(())
	}

	fn cmp_bool<F>(
		&mut self,
		val_l: Value,
		expr_r: &Expr,
		f: F,
		short_circuit_if: bool,
	) -> Result<Value, RuntimeError>
	where
		F: Fn(bool, bool) -> bool,
	{
		match val_l {
			Value::Bool(bool_l) => {
				if bool_l == short_circuit_if {
					return Ok(Value::Bool(short_circuit_if));
				}
				let val_r = self
					.walk_expr(&Spanned(expr_r, /* TODO: Use proper span */ 0..0))?
					.into();
				if let Value::Bool(bool_r) = val_r {
					return Ok(Value::Bool(f(bool_l, bool_r)));
				}
				RuntimeError::expected(Type::Bool, val_r.into())
			}
			_ => RuntimeError::expected(Type::Bool, val_l.into()),
		}
	}
}

impl AstVisitor for Interpreter {
	type Result = Result<Inter, RuntimeError>;

	fn visit_empty(&mut self) -> Self::Result {
		Ok(Inter::None(Value::Unit))
	}

	fn visit_decl(&mut self, Spanned(decl, span): &Spanned<&ast::Decl>) -> Self::Result {
		match decl {
			Decl::Fn(name, params, body) => {
				let val = RymFunction::new(Some(params.len()), params.clone(), body);
				self.env.declare(name, val.into(), true);
			}
			Decl::Const(name, init) => {
				let val: Value = self.walk_expr(&Spanned(init, span.clone()))?.into();
				self.env.declare(name, val, true);
			}
			Decl::Mut(name, init) => {
				let val: Value = self.walk_expr(&Spanned(init, span.clone()))?.into();
				self.env.declare(name, val, false);
			}
		}
		Ok(Inter::None(Value::Unit))
	}

	fn visit_ident(&mut self, ident: &str) -> Self::Result {
		Ok(Inter::None(self.env.get(&ident)?.clone()))
	}

	fn visit_lit(&mut self, lit: &ast::Literal) -> Self::Result {
		Ok(Inter::None(lit.clone().into()))
	}

	fn visit_assign(&mut self, expr_l: &Expr, expr_r: &Expr) -> Self::Result {
		let name = match expr_l {
			Expr::Identifier(name) => name,
			_ => {
				return RuntimeError::expected(
					Type::Identifier,
					self
						.walk_expr(&Spanned(expr_l, /* TODO: Use proper span */ 0..0))?
						.into(),
				);
			}
		};
		let value = self
			.walk_expr(&Spanned(expr_r, /* TODO: Use proper span */ 0..0))?
			.into();
		self.env.set(name, value)?;

		Ok(Inter::None(Value::Unit))
	}

	fn visit_call(&mut self, callee: &Expr, args: &[Expr]) -> Self::Result {
		let callee: Value = self
			.walk_expr(&Spanned(callee, /* TODO: Use proper span */ 0..0))?
			.into();
		let args: Vec<Value> = {
			let mut vec = Vec::new();
			for arg in args {
				vec.push(
					self
						.walk_expr(&Spanned(arg, /* TODO: Use proper span */ 0..0))?
						.into(),
				)
			}
			vec
		};

		let f: Box<dyn Callable> = match callee {
			Value::NativeFunction(f) => Box::new(f),
			Value::RymFunction(f) => Box::new(f),
			val => return RuntimeError::call(val.into()),
		};

		if let Some(arity) = f.arity() {
			if arity != args.len() {
				return RuntimeError::num_args_mismatch(arity, args.len());
			}
		}

		Ok(Inter::None(f.call(self, &args)?))
	}

	fn visit_unary(&mut self, op: &UnaryOp, expr: &Expr) -> Self::Result {
		let val = self
			.walk_expr(&Spanned(expr, /* TODO: Use proper span */ 0..0))?
			.into();

		Ok(Inter::None(match (op, val) {
			(UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),
			(UnaryOp::Neg, Value::Number(val)) => Value::Number(-val),
			(op, val) => return RuntimeError::unary(op, val.into()),
		}))
	}

	fn visit_logical(&mut self, expr_l: &Expr, op: &LogicalOp, expr_r: &Expr) -> Self::Result {
		let val_l = self
			.walk_expr(&Spanned(expr_l, /* TODO: Use proper span */ 0..0))?
			.into();

		Ok(Inter::None(if op == &LogicalOp::And {
			self.cmp_bool(val_l, expr_r, |val_l, val_r| val_l && val_r, false)?
		} else {
			self.cmp_bool(val_l, expr_r, |val_l, val_r| val_l || val_r, true)?
		}))
	}

	fn visit_binary(&mut self, expr_l: &Expr, op: &BinaryOp, expr_r: &Expr) -> Self::Result {
		let val_l = self
			.walk_expr(&Spanned(expr_l, /* TODO: Use proper span */ 0..0))?
			.into();
		let val_r = self
			.walk_expr(&Spanned(expr_r, /* TODO: Use proper span */ 0..0))?
			.into();

		fn apply_num_fn<F, R>(val_l: Value, val_r: Value, f: F) -> Result<Value, RuntimeError>
		where
			F: Fn(f64, f64) -> R,
			Value: From<R>,
		{
			if let Value::Number(val_l) = val_l {
				if let Value::Number(val_r) = val_r {
					return Ok(Value::from(f(val_l, val_r)));
				}
			}
			RuntimeError::comparison(val_l.into(), val_r.into())
		}

		Ok(Inter::None(match op {
			BinaryOp::Eq => Value::from(val_l == val_r),
			BinaryOp::Ne => Value::from(val_l != val_r),
			BinaryOp::Gt => apply_num_fn(val_l, val_r, |val_l, val_r| val_l > val_r)?,
			BinaryOp::Ge => apply_num_fn(val_l, val_r, |val_l, val_r| val_l >= val_r)?,
			BinaryOp::Lt => apply_num_fn(val_l, val_r, |val_l, val_r| val_l < val_r)?,
			BinaryOp::Le => apply_num_fn(val_l, val_r, |val_l, val_r| val_l <= val_r)?,
			BinaryOp::Mul => apply_num_fn(val_l, val_r, |val_l, val_r| val_l * val_r)?,
			BinaryOp::Div => apply_num_fn(val_l, val_r, |val_l, val_r| val_l / val_r)?,
			BinaryOp::Mod => apply_num_fn(val_l, val_r, |val_l, val_r| val_l % val_r)?,
			BinaryOp::Sub => apply_num_fn(val_l, val_r, |val_l, val_r| val_l - val_r)?,
			BinaryOp::Add => match (val_l, val_r) {
				(val_l @ Value::Number(_), val_r @ Value::Number(_)) => {
					apply_num_fn(val_l, val_r, |val_l, val_r| val_l + val_r)?
				}

				(Value::String(l), Value::String(r)) => (l + &r).into(),
				(Value::Number(l), Value::String(r)) => (l.to_string() + &r).into(),
				(Value::Bool(l), Value::String(r)) => (l.to_string() + &r).into(),
				(Value::String(l), Value::Number(r)) => (l + &r.to_string()).into(),
				(Value::String(l), Value::Bool(r)) => (l + &r.to_string()).into(),

				(val_l, val_r) => return RuntimeError::addition(val_l.into(), val_r.into()),
			},
		}))
	}

	fn visit_block(&mut self, block: &Block) -> Self::Result {
		self.env.push_scope();

		let mut stmts = block.stmts.iter();
		let return_value = loop {
			let stmt = match stmts.next() {
				Some(stmt) => stmt,
				None => break Inter::None(Value::Unit),
			};

			let inter = self.walk_stmt(&Spanned(stmt, /* TODO: Use proper span */ 0..0))?;
			match inter {
				Inter::Return(val) => break Inter::Return(val),
				Inter::Break(val) => break Inter::Break(val),
				Inter::Continue => break Inter::Continue,
				Inter::None(val) => {
					if stmts.len() == 0 {
						break Inter::None(val);
					};
					continue;
				}
			}
		};

		self.env.pop_scope();

		Ok(return_value)
	}

	// TODO: Implement break, continue and return
	fn visit_loop(&mut self, block: &Block) -> Self::Result {
		loop {
			match self.visit_block(block)? {
				Inter::Break(val) => break Ok(Inter::None(val)),
				_ => continue,
			}
		}
	}

	fn visit_if(
		&mut self,
		expr: &Expr,
		then_block: &Block,
		else_block: &Option<Block>,
	) -> Self::Result {
		let bool = match self
			.walk_expr(&Spanned(expr, /* TODO: Use proper span */ 0..0))?
			.into()
		{
			Value::Bool(bool) => bool,
			val => return RuntimeError::expected(Type::Bool, val.into()),
		};

		if bool {
			self.visit_block(then_block)
		} else if let Some(block) = else_block {
			self.visit_block(block)
		} else {
			Ok(Inter::None(Value::Unit))
		}
	}

	fn visit_return(&mut self, expr: &Expr) -> Self::Result {
		Ok(Inter::Return(
			self
				.walk_expr(&Spanned(expr, /* TODO: Use proper span */ 0..0))?
				.into(),
		))
	}

	fn visit_break(&mut self, expr: &Option<Expr>) -> Self::Result {
		Ok(Inter::Break(match expr {
			// TODO: Do loops work inside of break expr?
			Some(expr) => self
				.walk_expr(&Spanned(expr, /* TODO: Use proper span */ 0..0))?
				.into(),
			None => Value::Unit,
		}))
	}

	fn visit_continue(&mut self) -> Self::Result {
		Ok(Inter::Continue)
	}
}
