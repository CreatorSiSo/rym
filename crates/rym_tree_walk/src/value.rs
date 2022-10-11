use ast::Literal;

use crate::callable::{NativeFunction, RymFunction};
use crate::Inter;

pub(crate) enum Type {
	Unit,
	Bool,
	Number,
	String,
	Identifier,
	NativeFunction,
	RymFunction,
}

impl From<Value> for Type {
	fn from(val: Value) -> Self {
		match val {
			Value::Unit => Type::Unit,
			Value::Bool(_) => Type::Bool,
			Value::Number(_) => Type::Number,
			Value::String(_) => Type::String,
			Value::NativeFunction(_) => Type::NativeFunction,
			Value::RymFunction(_) => Type::RymFunction,
		}
	}
}

impl From<Inter> for Type {
	fn from(inter: Inter) -> Self {
		Value::from(inter).into()
	}
}

impl From<bool> for Type {
	fn from(_: bool) -> Self {
		Self::Bool
	}
}

impl core::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Unit => f.write_str("()"),
			Type::Bool => f.write_str("bool"),
			Type::Number => f.write_str("number"),
			Type::String => f.write_str("string"),
			Type::Identifier => f.write_str("identifier"),
			Type::NativeFunction => f.write_str("native_fn"),
			Type::RymFunction => f.write_str("rym_fn"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
	Unit,
	Number(f64),
	String(String),
	Bool(bool),
	NativeFunction(NativeFunction),
	RymFunction(RymFunction),
}

impl core::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Unit => f.write_str("()"),
			Value::Number(num) => f.write_str(&num.to_string()),
			Value::String(str) => f.write_str(str),
			Value::Bool(bool) => f.write_str(&bool.to_string()),
			Value::NativeFunction(_) => f.write_str("native_fn"),
			Value::RymFunction(_) => f.write_str("rym_fn"),
		}
	}
}

impl From<Literal> for Value {
	fn from(lit: Literal) -> Self {
		match lit {
			Literal::Unit => Self::Unit,
			Literal::Bool(bool) => Self::Bool(bool),
			Literal::Number(num) => Self::Number(num),
			Literal::String(str) => Self::String(str),
		}
	}
}

impl From<Inter> for Value {
	fn from(inter: Inter) -> Self {
		match inter {
			Inter::Break(val) | Inter::None(val) => val,
			_ => Value::Unit,
		}
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Self::Bool(value)
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Self::Number(value)
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl From<NativeFunction> for Value {
	fn from(value: NativeFunction) -> Self {
		Self::NativeFunction(value)
	}
}
