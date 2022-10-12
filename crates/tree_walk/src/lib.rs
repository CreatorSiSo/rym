#![allow(clippy::new_without_default)]

mod callable;
mod env;
mod error;
mod value;

use ast::{BinaryOp, Block, Expr, Identifier, Local, LogicalOp, Stmt, UnaryOp};
use callable::{Callable, NativeFunction};
use env::Env;
use error::RuntimeError;
use value::{Type, Value};

pub(crate) enum Inter {
	Break(Value),
	Continue,
	None(Value),
}

pub struct Interpreter {
	env: Env,
}

impl Default for Interpreter {
	fn default() -> Self {
		let print_fn = NativeFunction::new(None, |_: _, args: &[Value]| {
			let mut string = String::new();
			for arg in args {
				string.push_str(&arg.to_string())
			}
			print!("{string}");
			// TODO make this work properly in repl
			Ok(Value::Unit)
		});
		let println_fn = NativeFunction::new(None, |_: _, args: &[Value]| {
			let mut string = String::new();
			for arg in args {
				string.push_str(&arg.to_string())
			}
			println!("{string}");
			Ok(Value::Unit)
		});

		Self::with_globals(vec![
			("print", print_fn.into()),
			("println", println_fn.into()),
			("PI", std::f64::consts::PI.into()),
			("TAU", std::f64::consts::TAU.into()),
			("E", std::f64::consts::E.into()),
			("SQRT_2", std::f64::consts::SQRT_2.into()),
		])
	}
}

impl Interpreter {
	pub fn with_globals(globals: Vec<(&str, Value)>) -> Self {
		Self {
			env: globals
				.into_iter()
				.fold(Env::new(), |mut env, (name, val)| {
					env.declare(name, val.into(), true);
					env
				}),
		}
	}

	pub fn eval(&mut self, ast: &[Stmt]) -> Result<(), RuntimeError> {
		for stmt in ast {
			self.stmt(stmt)?;
		}
		Ok(())
	}

	fn stmt(&mut self, stmt: &Stmt) -> Result<Inter, RuntimeError> {
		match stmt {
			Stmt::Local(local) => {
				self.local(local)?;
			}
			Stmt::Print(expr) => {
				match self.expr(expr)?.into() {
					val @ (Value::Number(_) | Value::String(_) | Value::Bool(_)) => println!("{val}"),
					val => {
						return RuntimeError::expected(
							// TODO: Update error so that bool and number are valid as well
							Type::String,
							val.into(),
						);
					}
				}
			}
			Stmt::Expr(expr) => return self.expr(expr),
			Stmt::Empty => {}
		}
		Ok(Inter::None(Value::Unit))
	}

	fn local(&mut self, local: &Local) -> Result<(), RuntimeError> {
		match local {
			Local::Const(name, init) => {
				let val: Value = self.expr(init)?.into();
				self.env.declare(name, val, true);
			}
			Local::Mut(name, init) => {
				let val: Value = self.expr(init)?.into();
				self.env.declare(name, val, false);
			}
		}
		Ok(())
	}

	fn expr(&mut self, expr: &Expr) -> Result<Inter, RuntimeError> {
		match expr {
			Expr::Identifier(Identifier { name, .. }) => Ok(Inter::None(self.env.get(name)?.clone())),
			Expr::Literal(lit) => Ok(Inter::None(lit.clone().into())),
			Expr::Assign(left, right) => self.assign(left, right),
			Expr::Call(callee, args) => self.call(callee, args),

			Expr::Unary(op, expr) => self.unary(op, expr),
			Expr::Logical(left, op, right) => self.logical(left, op, right),
			Expr::Binary(left, op, right) => self.binary(left, op, right),

			Expr::Group(expr) => self.expr(expr),
			Expr::Block(block) => self.block(block),
			Expr::If(expr, then_block, else_block) => self.if_(expr, then_block, else_block),
			Expr::Loop(block) => self.loop_(block),

			Expr::Break(_) => Ok(Inter::Break(Value::Unit)),
			Expr::Continue => Ok(Inter::Continue),

			_ => panic!("Not yet implemented: {:?}", expr),
		}
	}

	fn if_(
		&mut self,
		expr: &Expr,
		then_block: &Block,
		else_block: &Option<Block>,
	) -> Result<Inter, RuntimeError> {
		let bool = match self.expr(expr)?.into() {
			Value::Bool(bool) => bool,
			val => return RuntimeError::expected(Type::Bool, val.into()),
		};

		if bool {
			self.block(then_block)
		} else if let Some(block) = else_block {
			self.block(block)
		} else {
			Ok(Inter::None(Value::Unit))
		}
	}

	// TODO: Implement break, continue and return
	fn loop_(&mut self, block: &Block) -> Result<Inter, RuntimeError> {
		loop {
			match self.block(block)? {
				Inter::Break(val) => break Ok(Inter::None(val)),
				_ => continue,
			}
		}
	}

	fn block(&mut self, block: &Block) -> Result<Inter, RuntimeError> {
		self.env.push_scope();

		let mut stmts = block.stmts.iter();
		let return_value = loop {
			let stmt = match stmts.next() {
				Some(stmt) => stmt,
				None => break Inter::None(Value::Unit),
			};

			let inter = self.stmt(stmt)?;
			match inter {
				Inter::Break(val) => break Inter::Break(val),
				_ => continue,
			}

			// TODO: Handle last stmt as result
		};

		self.env.pop_scope();

		Ok(return_value)
	}

	fn assign(&mut self, expr_l: &Expr, expr_r: &Expr) -> Result<Inter, RuntimeError> {
		let name = match expr_l {
			Expr::Identifier(Identifier { name, .. }) => name,
			_ => return RuntimeError::expected(Type::Identifier, self.expr(expr_l)?.into()),
		};
		let value = self.expr(expr_r)?.into();
		self.env.set(name, value)?;

		Ok(Inter::None(Value::Unit))
	}

	fn unary(&mut self, op: &UnaryOp, expr: &Expr) -> Result<Inter, RuntimeError> {
		let val = self.expr(expr)?.into();

		Ok(Inter::None(match (op, val) {
			(UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),
			(UnaryOp::Neg, Value::Number(val)) => Value::Number(-val),
			(op, val) => return RuntimeError::unary(op, val.into()),
		}))
	}

	fn call(&mut self, callee_expr: &Expr, args_expr: &Vec<Expr>) -> Result<Inter, RuntimeError> {
		let callee: Value = self.expr(callee_expr)?.into();
		let args: Vec<Value> = {
			let mut vec = Vec::new();
			for arg_expr in args_expr {
				vec.push(self.expr(arg_expr)?.into())
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

	// TODO: Make this easily understandable
	fn logical(
		&mut self,
		expr_l: &Expr,
		op: &LogicalOp,
		expr_r: &Expr,
	) -> Result<Inter, RuntimeError> {
		let val_l = self.expr(expr_l)?.into();

		Ok(Inter::None(if op == &LogicalOp::And {
			self.cmp_bool(val_l, expr_r, |val_l, val_r| val_l && val_r, false)?
		} else {
			self.cmp_bool(val_l, expr_r, |val_l, val_r| val_l || val_r, true)?
		}))
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
				let val_r = self.expr(expr_r)?.into();
				if let Value::Bool(bool_r) = val_r {
					return Ok(Value::Bool(f(bool_l, bool_r)));
				}
				RuntimeError::expected(Type::Bool, val_r.into())
			}
			_ => RuntimeError::expected(Type::Bool, val_l.into()),
		}
	}

	// TODO: Assignment expression

	fn binary(&mut self, expr_l: &Expr, op: &BinaryOp, expr_r: &Expr) -> Result<Inter, RuntimeError> {
		let val_l = self.expr(expr_l)?.into();
		let val_r = self.expr(expr_r)?.into();

		Ok(Inter::None(match op {
			BinaryOp::Eq => Value::from(val_l == val_r),
			BinaryOp::Ne => Value::from(val_l != val_r),
			BinaryOp::Gt => Self::number(val_l, val_r, |val_l, val_r| val_l > val_r)?,
			BinaryOp::Ge => Self::number(val_l, val_r, |val_l, val_r| val_l >= val_r)?,
			BinaryOp::Lt => Self::number(val_l, val_r, |val_l, val_r| val_l < val_r)?,
			BinaryOp::Le => Self::number(val_l, val_r, |val_l, val_r| val_l <= val_r)?,
			BinaryOp::Mul => Self::number(val_l, val_r, |val_l, val_r| val_l * val_r)?,
			BinaryOp::Div => Self::number(val_l, val_r, |val_l, val_r| val_l / val_r)?,
			BinaryOp::Mod => Self::number(val_l, val_r, |val_l, val_r| val_l % val_r)?,
			BinaryOp::Sub => Self::number(val_l, val_r, |val_l, val_r| val_l - val_r)?,
			BinaryOp::Add => match (val_l, val_r) {
				(val_l @ Value::Number(_), val_r @ Value::Number(_)) => {
					Self::number(val_l, val_r, |val_l, val_r| val_l + val_r)?
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

	fn number<F, R>(val_l: Value, val_r: Value, f: F) -> Result<Value, RuntimeError>
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
}
