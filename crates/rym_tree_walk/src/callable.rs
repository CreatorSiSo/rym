use crate::{error::RuntimeError, Interpreter, Value};

pub(crate) trait Callabe {
	fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, RuntimeError>;
}

#[derive(Clone)]
pub(crate) struct NativeFunction {
	id: u16,
	callable: fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>,
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
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl core::fmt::Debug for NativeFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NativeFunction")
			.field("id", &self.id)
			.finish()
	}
}

#[derive(Clone)]
pub(crate) struct RymFunction {
	id: u16,
	callable: fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>,
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
		f.debug_struct("RymFunction").field("id", &self.id).finish()
	}
}

impl PartialEq for RymFunction {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
