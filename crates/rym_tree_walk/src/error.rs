use rym_ast::{Literal, UnaryOp};

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
	pub fn assignment<T>(name: &str, value: Literal) -> Result<T, Self> {
		Err(Self::Assignment(format!(
			"Cannot assign `{value}` to constant {name}"
		)))
	}

	pub fn undeclared_var<T>(name: &str) -> Result<T, Self> {
		Err(Self::UndeclaredVar(format!(
			"Variable `{name}` has not been declared"
		)))
	}

	pub fn unary<T>(op: &UnaryOp, right: Literal) -> Result<T, Self> {
		Err(Self::ForbiddenType(format!(
			"Cannot apply unary operator `{}` to `{}`",
			match op {
				UnaryOp::Neg => "-",
				UnaryOp::Not => "!",
			},
			right.to_type_string(),
		)))
	}

	pub fn expected<T>(type_str: &str, value: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Expected `{type_str}` got {}",
			value.to_type_string()
		)))
	}

	pub fn comparison<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot compare `{}` with `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub fn addition<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot add `{}` to `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub fn substraction<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot substract `{}` from `{}`",
			right.to_type_string(),
			left.to_type_string(),
		)))
	}

	pub fn multiplication<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot multiply `{}` by `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub fn division<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot divide `{}` by `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}
}
