use crate::{env::EnvError, Type};
use ast::UnaryOp;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
	#[error("Expected `{expected}` argument(s) but got `{got}`")]
	NumArgsMismatch { expected: usize, got: usize },
	#[error("{0}")]
	ForbiddenInter(String),
	#[error("Cannot divide by zero")]
	DivideByZero,

	#[error("Panic: {0}")]
	Panic(String),

	#[error("{0}")]
	TypeError(#[source] TypeError),
	#[error("{0}")]
	EnvError(#[source] EnvError),
}

impl From<TypeError> for RuntimeError {
	fn from(err: TypeError) -> Self {
		RuntimeError::TypeError(err)
	}
}

impl From<EnvError> for RuntimeError {
	fn from(err: EnvError) -> Self {
		RuntimeError::EnvError(err)
	}
}

// TODO: Print line number as well
impl RuntimeError {
	pub(crate) fn num_args_mismatch<T>(expected: usize, got: usize) -> Result<T, Self> {
		Err(Self::NumArgsMismatch { expected, got })
	}
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TypeError {
	#[error("Expected `{0}` got `{1}`")]
	Expected(Type, Type),

	#[error("Cannot apply unary operator `{0}` to `{1}`")]
	Unary(UnaryOp, Type),

	#[error("Cannot compare `{0}` with `{1}`")]
	Compare(Type, Type),

	#[error("Cannot add `{0}` to `{1}`")]
	Add(Type, Type),

	#[error("Cannot substract `{1}` from `{0}`")]
	Substract(Type, Type),

	#[error("Cannot multiply `{0}` with `{1}`")]
	Multiply(Type, Type),

	#[error("Cannot divide `{0}` by `{1}`")]
	Divide(Type, Type),

	#[error("Cannot modulate `{0}` by `{1}`")]
	Modulate(Type, Type),

	#[error(
		"Cannot call `{0}` expected `{}` or `{}`",
		Type::RymFunction,
		Type::NativeFunction
	)]
	Call(Type),
}
