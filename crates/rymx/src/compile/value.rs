use itertools::Itertools;
use num_bigint::{BigInt, BigUint};
use num_rational::{BigRational, Ratio};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Value {
	Unit = 0,
	Int(BigInt),
	// Not using BigRational here to keep the size of Value reasonable
	Float(Ratio<u128>),
	Array(Box<[u8]>),
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Unit => write!(f, "()"),
			Value::Int(inner) => write!(f, "{inner}"),
			Value::Float(inner) => write!(f, "{inner}"),
			Value::Array(inner) => write!(f, "[{}]", inner.iter().join(", ")),
		}
	}
}
