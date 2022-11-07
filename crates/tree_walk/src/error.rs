use crate::{Type, Value};
use ast::UnaryOp;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
	ForbiddenType(String),
	ForbiddenInter(String),
	TypeMismatch(String),
	UndeclaredVar(String),
	Assignment(String),
	NumArgsMismatch(String),
	DivideByZero,
	Panic,
}

// TODO: Print line number as well
impl RuntimeError {
	pub(crate) fn num_args_mismatch<T>(expected: usize, got: usize) -> Result<T, Self> {
		Err(Self::NumArgsMismatch(format!(
			"Expected `{expected}` argument(s) but got `{got}`"
		)))
	}

	pub(crate) fn const_assign<T>(name: &str, value: Value) -> Result<T, Self> {
		Err(Self::Assignment(format!(
			"Assignment of `{value}` to constant `{name}` is forbidden"
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

	pub(crate) fn call<T>(typ: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot call `{typ}`, expected `{}` or `{}`",
			Type::RymFunction,
			Type::NativeFunction
		)))
	}

	pub fn expected<T>(expected: Type, got: Type) -> Result<T, Self> {
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

	pub(crate) fn addition<T>(left: Type, right: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot add `{}` to `{}`",
			left, right,
		)))
	}

	// TODO: Use this error
	pub(crate) fn _substraction<T>(left: Type, right: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot substract `{}` from `{}`",
			right, left,
		)))
	}

	// TODO: Use this error
	pub(crate) fn _multiplication<T>(left: Type, right: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot multiply `{}` by `{}`",
			left, right,
		)))
	}

	// TODO: Use this error
	pub(crate) fn _division<T>(left: Type, right: Type) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot divide `{}` by `{}`",
			left, right,
		)))
	}
}
