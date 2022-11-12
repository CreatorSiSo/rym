use std::fmt::Debug;

use crate::error::{spanned_err, LogicError, SpannedError};
use crate::{Inter, Interpreter, Value};
use ast::{AstVisitor, Expr, Spanned};

type Arity = Option<usize>;
pub type CallableFn = dyn Fn(&mut Interpreter, &[Value]) -> Result<Value, SpannedError>;

pub trait Callable {
	/// None => infinite arguments
	///
	/// Some(num) => num arguments
	fn arity(&self) -> Option<usize>;

	fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, SpannedError>;
}

#[derive(Clone)]
pub struct NativeFunction {
	arity: Arity,
	callable: &'static CallableFn,
}

impl NativeFunction {
	pub fn new(arity: Arity, callable: &'static CallableFn) -> Self {
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
	) -> Result<Value, SpannedError> {
		(self.callable)(interpreter, args)
	}
}

impl PartialEq for NativeFunction {
	fn eq(&self, other: &Self) -> bool {
		self.arity == other.arity && std::ptr::eq(&self.callable, &other.callable)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct RymFunction {
	arity: Arity,
	params: Vec<String>,
	body: Spanned<Expr>,
}

impl RymFunction {
	pub const fn new(arity: Arity, params: Vec<String>, body: Spanned<Expr>) -> Self {
		Self {
			arity,
			params,
			body,
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
	) -> Result<Value, SpannedError> {
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
			interpreter.walk_expr(&self.body.as_ref())
		};
		interpreter.env.pop_env();

		match return_val {
			Ok(inter) => match inter {
				Inter::Return(val) | Inter::None(val) => Ok(val),
				Inter::Break(_) => spanned_err(
					LogicError::ForbiddenInter("Using `break` outside of a loop is not allowed".into()),
					self.body.1.clone(),
				),
				Inter::Continue => spanned_err(
					LogicError::ForbiddenInter("Using `continue` outside of a loop is not allowed".into()),
					self.body.1.clone(),
				),
			},
			Err(err) => Err(err.into()),
		}
	}
}
