use std::{fmt::Debug, rc::Rc};

use crate::{error::RuntimeError, Inter, Interpreter, Value};
use ast::{AstVisitor, Expr, Spanned};

type Arity = Option<usize>;
type CallableFn = dyn Fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>;

pub trait Callable {
	/// None => infinite arguments
	///
	/// Some(num) => num arguments
	fn arity(&self) -> Option<usize>;

	fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, RuntimeError>;
}

#[derive(Clone)]
pub struct NativeFunction {
	arity: Arity,
	callable: Rc<CallableFn>,
}

impl NativeFunction {
	pub fn new(arity: Arity, callable: Rc<CallableFn>) -> Self {
		Self { arity, callable }
	}
}

impl Debug for NativeFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NativeFunction")
			.field("arity", &self.arity)
			.field("callable", &"Rc<dyn FnMut>")
			.finish()
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
	fn eq(&self, other: &Self) -> bool {
		self.arity == other.arity && Rc::ptr_eq(&self.callable, &other.callable)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct RymFunction {
	arity: Arity,
	params: Vec<String>,
	body: Expr,
}

impl RymFunction {
	pub fn new(arity: Arity, params: Vec<String>, body: &Expr) -> Self {
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
		debug_assert_eq!(
			self.params.len(),
			args.len(),
			"Internal Error: Number of `rym_fn` parameters does not match number of arguments."
		);

		interpreter.env.push_env();
		let return_val = {
			for (idx, param) in self.params.iter().enumerate() {
				interpreter.env.declare(param, args[idx].clone(), true)
			}
			interpreter.walk_expr(&Spanned(&self.body, /* TODO: Use proper span */ 0..0))
		};
		interpreter.env.pop_env();

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
