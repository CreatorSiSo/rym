#![allow(clippy::new_without_default)]

use std::io::Write;

mod callable;
mod env;
mod error;
mod value;

pub use callable::{CallableFn, NativeFunction};
pub use error::{spanned_err, LogicError, SpannedError, TypeError};
pub use value::{Type, Value};

use ast::{AstVisitor, BinaryOp, Block, Expr, Item, LogicalOp, Span, Spanned, Stmt, UnaryOp};
use callable::{Callable, RymFunction};
use env::GlobalEnv;

#[derive(Debug)]
pub enum Inter {
	Return(Value),
	Break(Value),
	Continue,
	None(Value),
}

pub fn global_values<'a>() -> Vec<(&'a str, Value)> {
	let print_fn = NativeFunction::new(None, &|_: _, args: &[Value]| {
		let mut string = String::new();
		for arg in args {
			string.push_str(&arg.to_string())
		}
		// TODO fix print() for repl
		print!("{string}");
		std::io::stdout().flush().expect("Internal Error: Could not flush stout");
		Ok(Value::Unit)
	});

	let println_fn = NativeFunction::new(None, &|_: _, args: &[Value]| {
		let mut string = String::new();
		for arg in args {
			string.push_str(&arg.to_string())
		}
		println!("{string}");
		Ok(Value::Unit)
	});

	let panic_fn = NativeFunction::new(None, &|_: _, args: &[Value]| {
		spanned_err(
			LogicError::Panic(args.iter().fold(String::new(), |accum, arg| accum + &arg.to_string())),
			0..0,
		)
	});

	// TODO: Gracefully shut down interpreter on a failed assert
	let assert_fn = NativeFunction::new(Some(1), &|_: _, args: &[Value]| match args[0].clone() {
		Value::Bool(val) => {
			assert!(val);
			Ok(Value::Unit)
		}
		val => spanned_err(TypeError::Expected(Type::Bool, val.typ()), 0..0),
	});

	// TODO: Gracefully shut down interpreter on a failed assert_eq
	let assert_eq_fn = NativeFunction::new(Some(2), &|_: _, args: &[Value]| {
		assert_eq!(args[0], args[1]);
		Ok(Value::Unit)
	});

	let floor_fn = NativeFunction::new(Some(1), &|_: _, args: &[/* Spanned< */ Value /* > */]| {
			let val = &args[0];
			if let Value::Number(num) = val {
				Ok(Value::Number(num.floor()))
			} else {
				spanned_err(TypeError::Expected(Type::Number, val.typ()), 0..0)
			}
		});

	let sqrt_fn = NativeFunction::new(Some(1), &|_: _, args: &[Value]| {
		let val = &args[0];
		if let Value::Number(num) = val {
			Ok(Value::Number(num.sqrt()))
		} else {
			spanned_err(TypeError::Expected(Type::Number, val.typ()), 0..0)
		}
	});

	vec![
		("print", print_fn.into()),
		("println", println_fn.into()),
		("panic", panic_fn.into()),
		("assert", assert_fn.into()),
		("assert_eq", assert_eq_fn.into()),
		("floor", floor_fn.into()),
		("sqrt", sqrt_fn.into()),
		("PI", std::f64::consts::PI.into()),
		("TAU", std::f64::consts::TAU.into()),
		("E", std::f64::consts::E.into()),
		("SQRT_2", std::f64::consts::SQRT_2.into()),
	]
}

pub struct Interpreter {
	env: GlobalEnv,
}

impl Default for Interpreter {
	fn default() -> Self {
		Self::with_globals(global_values())
	}
}

impl Interpreter {
	pub fn with_globals(globals: Vec<(&str, Value)>) -> Self {
		Self {
			env: globals.into_iter().fold(GlobalEnv::new(), |mut global_env, (name, val)| {
				global_env.env.declare(name, val, true);
				global_env
			}),
		}
	}

	pub fn eval(&mut self, ast: &[Spanned<Stmt>]) -> Result<(), SpannedError> {
		for stmt in ast {
			self.walk_stmt(stmt.as_ref())?;
		}
		Ok(())
	}

	fn cmp_bool<F>(
		&mut self,
		Spanned(val_l, span_l): Spanned<Value>,
		expr_r: Spanned<&Expr>,
		f: F,
		short_circuit_if: bool,
	) -> Result<Inter, SpannedError>
	where
		F: Fn(bool, bool) -> bool,
	{
		let bool_l = match val_l {
			Value::Bool(bool_l) => bool_l,
			_ => return spanned_err(TypeError::Expected(Type::Bool, val_l.typ()), span_l),
		};
		if short_circuit_if == bool_l {
			return Ok(Inter::None(Value::Bool(short_circuit_if)));
		}
		let span_r = expr_r.1.clone();
		let bool_r = match self.walk_expr(expr_r)? {
			Inter::None(val) => match val {
				Value::Bool(bool_r) => bool_r,
				val_r => return spanned_err(TypeError::Expected(Type::Bool, val_r.typ()), span_r),
			},
			inter => return Ok(inter),
		};
		Ok(Inter::None(Value::Bool(f(bool_l, bool_r))))
	}
}

impl AstVisitor for Interpreter {
	type Result = Result<Inter, SpannedError>;

	fn visit_item(&mut self, Spanned(item, _): Spanned<&ast::Item>) -> Self::Result {
		match item {
			Item::Module { .. } => {
				todo!()
			}
			Item::Func { name, params, rhs } => {
				let Some(rhs) = rhs else {
					todo!()
				};
				let val = RymFunction::new(Some(params.len()), params.clone(), rhs.clone());
				self.env.declare(&name.0, val.into(), true);
			}
			Item::Binding { name, rhs } => {
				let val: Value = match self.walk_expr(rhs.as_ref())? {
					Inter::None(val) => val,
					inter => return Ok(inter),
				};
				self.env.declare(&name.0, val, true);
			}
		}
		Ok(Inter::None(Value::Unit))
	}

	fn visit_ident(&mut self, ident: &str, span: Span) -> Self::Result {
		match self.env.get(ident) {
			Ok(val) => Ok(Inter::None(val.clone())),
			Err(err) => spanned_err(err, span),
		}
	}

	fn visit_literal(&mut self, lit: &ast::Literal) -> Self::Result {
		Ok(Inter::None(lit.clone().into()))
	}

	fn visit_assign(&mut self, expr_l: Spanned<&Expr>, expr_r: Spanned<&Expr>) -> Self::Result {
		let span_l = expr_l.1.clone();
		let span_r = expr_r.1.clone();

		let name = match expr_l {
			Spanned(Expr::Ident(name), _) => name,
			_ => {
				return spanned_err(
					TypeError::Expected(
						Type::Identifier,
						match self.walk_expr(expr_l)? {
							Inter::None(val) => val.typ(),
							inter => return Ok(inter),
						},
					),
					span_l,
				);
			}
		};

		let value = match self.walk_expr(expr_r)? {
			Inter::None(val) => val,
			inter => return Ok(inter),
		};
		if let Err(err) = self.env.set(name, value) {
			return spanned_err(err, span_r);
		}

		Ok(Inter::None(Value::Unit))
	}

	fn visit_call(&mut self, callee_expr: Spanned<&Expr>, args: &[Spanned<Expr>]) -> Self::Result {
		let callee_span = callee_expr.1.clone();
		let callee: Value = match self.walk_expr(callee_expr)? {
			Inter::None(val) => val,
			inter => return Ok(inter),
		};
		let args: Vec<Value> = {
			let mut vec = Vec::new();
			for arg in args {
				vec.push(match self.walk_expr(arg.as_ref())? {
					Inter::None(val) => val,
					inter => return Ok(inter),
				})
			}
			vec
		};

		let f: Box<dyn Callable> = match callee {
			Value::NativeFunction(f) => Box::new(f),
			Value::RymFunction(f) => Box::new(f),
			val => spanned_err(TypeError::Call(val.typ()), callee_span)?,
		};

		if let Some(arity) = f.arity() {
			if arity != args.len() {
				return spanned_err(
					LogicError::NumArgsMismatch { expected: arity, got: args.len() },
					0..0, /* TODO span args.first().1.start  */
				);
			}
		}

		Ok(Inter::None(f.call(self, &args)?))
	}

	fn visit_unary(&mut self, op: &UnaryOp, expr: Spanned<&Expr>) -> Self::Result {
		let val = match self.walk_expr(expr)? {
			Inter::None(val) => val,
			inter => return Ok(inter),
		};

		match (op, val) {
			(UnaryOp::Not, Value::Bool(val)) => Ok(Inter::None(Value::Bool(!val))),
			(UnaryOp::Neg, Value::Number(val)) => Ok(Inter::None(Value::Number(-val))),
			(op, val) => spanned_err(TypeError::Unary(*op, val.typ()), 0..0),
		}
	}

	fn visit_logical(
		&mut self,
		expr_l: Spanned<&Expr>,
		op: &LogicalOp,
		expr_r: Spanned<&Expr>,
	) -> Self::Result {
		let val_l = match self.walk_expr(expr_l.clone()) {
			Ok(Inter::None(val_l)) => val_l,
			other => return other,
		};

		if op == &LogicalOp::And {
			self.cmp_bool(expr_l.map(|_| val_l), expr_r, |val_l, val_r| val_l && val_r, false)
		} else {
			self.cmp_bool(expr_l.map(|_| val_l), expr_r, |val_l, val_r| val_l || val_r, true)
		}
	}

	fn visit_binary(
		&mut self,
		expr_l: Spanned<&Expr>,
		op: &BinaryOp,
		expr_r: Spanned<&Expr>,
	) -> Self::Result {
		let span = expr_l.1.start..expr_r.1.end;

		let val_l = match self.walk_expr(expr_l)? {
			Inter::None(val) => val,
			inter => return Ok(inter),
		};
		let val_r = match self.walk_expr(expr_r)? {
			Inter::None(val) => val,
			inter => return Ok(inter),
		};

		fn apply_num_fn<F, R>(
			maybe_err: Option<TypeError>,
			vals: (Value, Value),
			span: Span,
			f: F,
		) -> Result<Value, SpannedError>
		where
			Value: From<R>,
			F: Fn(f64, f64) -> R,
		{
			if let (Value::Number(val_l), Value::Number(val_r)) = vals {
				return Ok(Value::from(f(val_l, val_r)));
			}
			spanned_err(
				match maybe_err {
					Some(err) => err,
					None => TypeError::Compare(vals.0.typ(), vals.1.typ()),
				},
				span,
			)
		}

		Ok(Inter::None(match op {
			BinaryOp::Eq => Value::from(val_l == val_r),
			BinaryOp::Ne => Value::from(val_l != val_r),
			BinaryOp::Gt => apply_num_fn(None, (val_l, val_r), span, |val_l, val_r| val_l > val_r)?,
			BinaryOp::Ge => apply_num_fn(None, (val_l, val_r), span, |val_l, val_r| val_l >= val_r)?,
			BinaryOp::Lt => apply_num_fn(None, (val_l, val_r), span, |val_l, val_r| val_l < val_r)?,
			BinaryOp::Le => apply_num_fn(None, (val_l, val_r), span, |val_l, val_r| val_l <= val_r)?,
			BinaryOp::Mul => apply_num_fn(
				Some(TypeError::Multiply(val_l.typ(), val_r.typ())),
				(val_l, val_r),
				span,
				|val_l, val_r| val_l * val_r,
			)?,
			BinaryOp::Div => apply_num_fn(
				Some(TypeError::Divide(val_l.typ(), val_r.typ())),
				(val_l, val_r),
				span,
				|val_l, val_r| val_l / val_r,
			)?,
			BinaryOp::Mod => apply_num_fn(
				Some(TypeError::Modulate(val_l.typ(), val_r.typ())),
				(val_l, val_r),
				span,
				|val_l, val_r| val_l % val_r,
			)?,
			BinaryOp::Sub => apply_num_fn(
				Some(TypeError::Substract(val_l.typ(), val_r.typ())),
				(val_l, val_r),
				span,
				|val_l, val_r| val_l - val_r,
			)?,
			BinaryOp::Add => match (val_l, val_r) {
				(val_l @ Value::Number(_), val_r @ Value::Number(_)) => {
					apply_num_fn(None, (val_l, val_r), span, |val_l, val_r| val_l + val_r)?
				}

				(Value::String(l), Value::String(r)) => (l + &r).into(),
				(Value::Number(l), Value::String(r)) => (l.to_string() + &r).into(),
				(Value::Bool(l), Value::String(r)) => (l.to_string() + &r).into(),
				(Value::String(l), Value::Number(r)) => (l + &r.to_string()).into(),
				(Value::String(l), Value::Bool(r)) => (l + &r.to_string()).into(),

				(val_l, val_r) => return spanned_err(TypeError::Add(val_l.typ(), val_r.typ()), span),
			},
		}))
	}

	fn visit_block(&mut self, Spanned(stmts, _): Spanned<&Block>) -> Self::Result {
		self.env.push_scope();

		let mut stmts = stmts.iter();
		let return_value = loop {
			let stmt = match stmts.next() {
				Some(stmt) => stmt,
				None => break Inter::None(Value::Unit),
			};

			let inter = self.walk_stmt(stmt.as_ref())?;
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
	fn visit_loop(&mut self, block: Spanned<&Block>) -> Self::Result {
		loop {
			match self.visit_block(block.clone())? {
				Inter::Continue | Inter::None(_) => continue,
				Inter::Break(val) => break Ok(Inter::None(val)),
				inter_return => return Ok(inter_return),
			}
		}
	}

	fn visit_if(
		&mut self,
		expr: Spanned<&Expr>,
		then_block: Spanned<&Block>,
		else_block: &Option<Spanned<Block>>,
	) -> Self::Result {
		let span = expr.1.clone();
		let bool = match self.walk_expr(expr)? {
			Inter::None(val) => match val {
				Value::Bool(bool) => bool,
				val => return spanned_err(TypeError::Expected(Type::Bool, val.typ()), span),
			},
			inter => return Ok(inter),
		};

		if bool {
			self.visit_block(then_block)
		} else if let Some(block) = else_block {
			self.visit_block(block.as_ref())
		} else {
			Ok(Inter::None(Value::Unit))
		}
	}

	fn visit_return(&mut self, expr: Spanned<&Expr>) -> Self::Result {
		match self.walk_expr(expr)? {
			Inter::None(val) => Ok(Inter::Return(val)),
			inter => Ok(inter),
		}
	}

	fn visit_break(&mut self, expr: Option<Spanned<&Expr>>) -> Self::Result {
		Ok(Inter::Break(match expr {
			// TODO: Do loops work inside of break expr?
			Some(expr) => match self.walk_expr(expr)? {
				Inter::None(val) => val,
				inter => return Ok(inter),
			},
			None => Value::Unit,
		}))
	}

	fn visit_continue(&mut self) -> Self::Result {
		Ok(Inter::Continue)
	}
}
