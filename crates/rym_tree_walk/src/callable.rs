use crate::{error::RuntimeError, Interpreter, Value};

type CallableFn = fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>;

pub(crate) trait Callabe {
	fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, RuntimeError>;
}

#[derive(Clone)]
pub(crate) struct NativeFunction {
	callable: CallableFn,
}

impl NativeFunction {
	pub(crate) fn new(callable: CallableFn) -> Self {
		Self { callable }
	}
}

impl Callabe for NativeFunction {
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
pub(crate) struct RymFunction {
	callable: CallableFn,
}

impl Callabe for RymFunction {
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
