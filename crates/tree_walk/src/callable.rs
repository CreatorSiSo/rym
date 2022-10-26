use ast::{AstVisitor, Expr};

use crate::{error::RuntimeError, Inter, Interpreter, Value};

pub(crate) type Arity = Option<usize>;
pub(crate) type CallableFn = fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>;

pub trait Callable {
	fn arity(&self) -> Option<usize>;
	fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, RuntimeError>;
}

#[derive(Clone)]
pub struct NativeFunction {
	arity: Arity,
	callable: CallableFn,
}

impl NativeFunction {
	pub(crate) fn new(arity: Arity, callable: CallableFn) -> Self {
		Self { arity, callable }
	}
}

impl Callable for NativeFunction {
	fn arity(&self) -> Option<usize> {
		self.arity
	}

	fn call<'src>(
		&self,
		interpreter: &mut Interpreter,
		args: &[Value],
	) -> Result<Value, RuntimeError> {
		(self.callable)(interpreter, args)
	}
}

impl PartialEq for NativeFunction {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

impl core::fmt::Debug for NativeFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("native_fn")
		// TODO: Display types
	}
}

#[derive(Clone)]
pub struct RymFunction {
	arity: Arity,
	params: Vec<String>,
	body: Expr,
}

impl RymFunction {
	pub(crate) fn new(arity: Arity, params: Vec<String>, body: &Expr) -> Self {
		Self {
			arity,
			params,
			body: body.clone(),
		}
	}
}

impl Callable for RymFunction {
	fn arity(&self) -> Option<usize> {
		self.arity
	}

	fn call<'src>(
		&self,
		interpreter: &mut Interpreter,
		args: &[Value],
	) -> Result<Value, RuntimeError> {
		assert_eq!(
			self.params.len(),
			args.len(),
			"Internal Error: Number of `rym_fn` parameters does not match number of arguments."
		);

		interpreter.env.push_scope();
		let return_val = {
			for (idx, param) in self.params.iter().enumerate() {
				interpreter.env.declare(param, args[idx].clone(), true)
			}
			interpreter.walk_expr(&self.body)
		};
		interpreter.env.pop_scope();

		match return_val {
			Ok(inter) => match inter {
				Inter::Return(val) | Inter::None(val) => Ok(val),
				Inter::Break(_) => Err(RuntimeError::ForbiddenInter(
					"Using `break` outside of a loop is not allowed.".into(),
				)),
				Inter::Continue => Err(RuntimeError::ForbiddenInter(
					"Using `continue` outside of a loop is not allowed.".into(),
				)),
			},
			Err(err) => Err(err),
		}
	}
}

impl core::fmt::Debug for RymFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("rym_fn")
	}
}

impl PartialEq for RymFunction {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}
