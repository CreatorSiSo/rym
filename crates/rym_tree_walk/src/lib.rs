#![allow(clippy::new_without_default)]

mod callable;
mod env;
mod error;

use callable::{Callabe, NativeFunction, RymFunction};
use env::Env;
use error::RuntimeError;
use rym_ast::{BinaryOp, Block, Expr, Literal, Local, LogicalOp, Stmt, UnaryOp};

enum Type {
	Unit,
	Bool,
	Number,
	String,
	NativeFunction,
	RymFunction,
}

impl From<Value> for Type {
	fn from(val: Value) -> Self {
		match val {
			Value::Unit => Type::Unit,
			Value::Bool(_) => Type::Bool,
			Value::Number(_) => Type::Number,
			Value::String(_) => Type::String,
			Value::NativeFunction(_) => Type::NativeFunction,
			Value::RymFunction(_) => Type::RymFunction,
		}
	}
}

impl From<Inter> for Type {
	fn from(inter: Inter) -> Self {
		Value::from(inter).into()
	}
}

impl From<bool> for Type {
	fn from(_: bool) -> Self {
		Self::Bool
	}
}

impl core::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Unit => f.write_str("()"),
			Type::Bool => f.write_str("bool"),
			Type::Number => f.write_str("number"),
			Type::String => f.write_str("string"),
			Type::NativeFunction => f.write_str("native_fn"),
			Type::RymFunction => f.write_str("rym_fn"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
	Unit,
	Number(f64),
	String(String),
	Bool(bool),
	NativeFunction(NativeFunction),
	RymFunction(RymFunction),
}

impl From<Literal<'_>> for Value {
	fn from(lit: Literal<'_>) -> Self {
		match lit {
			Literal::Unit => Self::Unit,
			Literal::Bool(bool) => Self::Bool(bool),
			Literal::Number(num) => Self::Number(num),
			Literal::String(str) => Self::String(str),
			Literal::Identifier(_) => todo!(),
		}
	}
}

impl From<Inter> for Value {
	fn from(inter: Inter) -> Self {
		match inter {
			Inter::Break(val) | Inter::None(val) => val,
			_ => Value::Unit,
		}
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Self::Bool(value)
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Self::Number(value)
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

pub(crate) enum Inter {
	Break(Value),
	Continue,
	None(Value),
}

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

	fn stmt(&mut self, stmt: &Stmt<'src>) -> Result<Inter, RuntimeError> {
		match stmt {
			Stmt::Local(local) => {
				self.local(local)?;
			}
			Stmt::Print(expr) => {
				let val: Value = self.expr(expr)?.into();
				if let Value::String(string) = val {
					println!("{string}")
				} else {
					return RuntimeError::expected(Type::String, val.into());
				}
			}
			Stmt::Expr(expr) => return self.expr(expr),
			Stmt::Empty => {}
		}
		Ok(Inter::None(Value::Unit))
	}

	fn local(&mut self, local: &Local<'src>) -> Result<(), RuntimeError> {
		match local {
			Local::Const(name, init) => {
				let val: Value = self.expr(init)?.into();
				self.env.declare(name, val.clone(), true);
			}
			Local::Mut(name, init) => {
				let val: Value = self.expr(init)?.into();
				self.env.declare(name, val.clone(), false);
			}
		}
		Ok(())
	}

	fn expr(&mut self, expr: &Expr<'src>) -> Result<Inter, RuntimeError> {
		match expr {
			Expr::Literal(literal) => Ok(Inter::None(match literal {
				Literal::Identifier(identifier) => self.env.get(identifier)?.clone(),
				_ => literal.clone().into(),
			})),
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
		expr: &Expr<'src>,
		then_block: &Block<'src>,
		else_block: &Option<Block<'src>>,
	) -> Result<Inter, RuntimeError> {
		let bool = match self.expr(expr)?.into() {
			Value::Bool(bool) => bool,
			val => return RuntimeError::expected(Type::Bool, val.into()),
		};

		return if bool {
			self.block(then_block)
		} else if let Some(block) = else_block {
			self.block(block)
		} else {
			Ok(Inter::None(Value::Unit))
		};
	}

	// TODO: Implement break, continue and return
	fn loop_(&mut self, block: &Block<'src>) -> Result<Inter, RuntimeError> {
		loop {
			match self.block(block)? {
				Inter::Break(val) => break Ok(Inter::None(val)),
				_ => continue,
			}
		}
	}

	fn block(&mut self, block: &Block<'src>) -> Result<Inter, RuntimeError> {
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

	fn assign(&mut self, expr_l: &Expr<'src>, expr_r: &Expr<'src>) -> Result<Inter, RuntimeError> {
		let name = match expr_l {
			Expr::Literal(Literal::Identifier(name)) => name,
			_ => return RuntimeError::expected(todo!("symbol"), self.expr(expr_l)?.into()),
		};
		let value = self.expr(expr_r)?.into();
		self.env.set(name, value)?;

		Ok(Inter::None(Value::Unit))
	}

	fn call(
		&mut self,
		callee_expr: &Box<Expr<'src>>,
		args_expr: &Vec<Expr<'src>>,
	) -> Result<Inter, RuntimeError> {
		let callee: Value = self.expr(&callee_expr)?.into();
		let args: Vec<Value> = {
			let mut vec = Vec::new();
			for arg_expr in args_expr {
				vec.push(self.expr(arg_expr)?.into())
			}
			vec
		};

		Ok(Inter::None(match callee {
			Value::NativeFunction(f) => f.call(self, &args)?,
			Value::RymFunction(f) => f.call(self, &args)?,
			val => return RuntimeError::expected(Type::RymFunction, val.into()),
		}))
	}

	fn unary(&mut self, op: &UnaryOp, expr: &Expr<'src>) -> Result<Inter, RuntimeError> {
		let val = self.expr(expr)?.into();

		Ok(Inter::None(match (op, val) {
			(UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),
			(UnaryOp::Neg, Value::Number(val)) => Value::Number(-val),
			(op, val) => return RuntimeError::unary(op, val.into()),
		}))
	}

	// TODO: Make this easily understandable
	fn logical(
		&mut self,
		expr_l: &Expr<'src>,
		op: &LogicalOp,
		expr_r: &Expr<'src>,
	) -> Result<Inter, RuntimeError> {
		let lit_l = self.expr(expr_l)?.into();

		Ok(Inter::None(if op == &LogicalOp::And {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l && val_r, false)?
		} else {
			self.cmp_bool(lit_l, expr_r, |val_l, val_r| val_l || val_r, true)?
		}))
	}

	fn cmp_bool<F>(
		&mut self,
		val_l: Value,
		expr_r: &Expr<'src>,
		f: F,
		short_circuit_if: bool,
	) -> Result<Value, RuntimeError>
	where
		F: Fn(bool, bool) -> bool,
	{
		if let Value::Bool(val_l) = val_l {
			if val_l == short_circuit_if {
				return Ok(Value::Bool(short_circuit_if));
			}

			let val_r = self.expr(expr_r)?.into();
			if let Value::Bool(val_r) = val_r {
				return Ok(Value::Bool(f(val_l, val_r)));
			}
			return RuntimeError::comparison(val_l.into(), val_r.into());
		}

		// TODO: How should errors and short circuiting work?
		// Should the second value still be calculated if the first does not result in a bool?
		RuntimeError::comparison(val_l.into(), todo!())
	}

	// TODO: Assignment expression

	fn binary(
		&mut self,
		expr_l: &Expr<'src>,
		op: &BinaryOp,
		expr_r: &Expr<'src>,
	) -> Result<Inter, RuntimeError> {
		let lit_l = self.expr(expr_l)?.into();
		let lit_r = self.expr(expr_r)?.into();

		Ok(Inter::None(match op {
			BinaryOp::Eq => Value::from(lit_l == lit_r),
			BinaryOp::Ne => Value::from(lit_l != lit_r),
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

	fn number<F, R>(lit_l: Value, lit_r: Value, f: F) -> Result<Value, RuntimeError>
	where
		F: Fn(f64, f64) -> R,
		Value: From<R>,
	{
		if let Value::Number(val_l) = lit_l {
			if let Value::Number(val_r) = lit_r {
				return Ok(Value::from(f(val_l, val_r)));
			}
		}
		RuntimeError::comparison(lit_l.into(), lit_r.into())
	}
}
