use crate::{error::RuntimeError, Interpreter, Value};

pub(crate) type Arity = Option<usize>;
pub(crate) type CallableFn = fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>;

pub(crate) trait Callable {
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
	callable: CallableFn,
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
		(self.callable)(interpreter, args)
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
