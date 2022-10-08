use rym_ast::{Literal, UnaryOp};

use crate::{Type, Value};

#[derive(Debug)]
pub enum RuntimeError {
	ForbiddenType(String),
	TypeMismatch(String),
	UndeclaredVar(String),
	Assignment(String),
	DivideByZero,
}

// TODO: Print line number as well
impl RuntimeError {
	pub(crate) fn const_assign<T>(name: &str, value: Value) -> Result<T, Self> {
		Err(Self::Assignment(format!(
			"Cannot assign `{value:?}` to constant {name}"
		)))
	}

	pub(crate) fn undeclared_var<T>(name: &str) -> Result<T, Self> {
		Err(Self::UndeclaredVar(format!(
			"Variable `{name}` has not been declared"
		)))
	}

	pub(crate) fn unary<T>(op: &UnaryOp, right: Type) -> Result<T, Self> {
		Err(Self::ForbiddenType(format!(
			"Cannot apply unary operator `{}` to `{}`",
			match op {
				UnaryOp::Neg => "-",
				UnaryOp::Not => "!",
			},
			right,
		)))
	}

	pub(crate) fn expected<T>(expected: Type, got: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Expected `{expected}` got {got}",
		)))
	}

	pub(crate) fn comparison<T>(left: Type, right: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot compare `{}` with `{}`",
			left, right,
		)))
	}

	pub(crate) fn addition<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot add `{}` to `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub(crate) fn substraction<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot substract `{}` from `{}`",
			right.to_type_string(),
			left.to_type_string(),
		)))
	}

	pub(crate) fn multiplication<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot multiply `{}` by `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub(crate) fn division<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot divide `{}` by `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}
}
