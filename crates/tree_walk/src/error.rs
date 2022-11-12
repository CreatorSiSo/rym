use std::error::Error;

use crate::Type;
use ast::{Span, UnaryOp};

#[derive(thiserror::Error, Debug)]
#[error("{0:?}: {0}")]
pub struct SpannedError(Box<dyn Error + 'static>, Span);

impl PartialEq for SpannedError {
	fn eq(&self, other: &Self) -> bool {
		self.0.to_string() == other.0.to_string() && self.1 == other.1
	}
}

pub fn spanned_err<T, E: Error + 'static>(err: E, span: Span) -> Result<T, SpannedError> {
	Err(SpannedError(Box::new(err), span))
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum LogicError {
	#[error("Expected `{expected}` argument(s) but got `{got}`")]
	NumArgsMismatch { expected: usize, got: usize },
	#[error("{0}")]
	ForbiddenInter(String),
	// TODO Use DivideByZero error
	#[error("Cannot divide by zero")]
	DivideByZero,
	#[error("Panic: {0}")]
	Panic(String),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
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
