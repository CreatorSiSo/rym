use itertools::Itertools;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Constant {
    Unit = 0,
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<Constant>),
    String(String),
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Unit => write!(f, "()"),
            Constant::Bool(inner) => write!(f, "{inner}"),
            Constant::Int(inner) => write!(f, "{inner}"),
            Constant::Float(inner) => write!(f, "{inner}"),
            Constant::Array(inner) => write!(f, "[{}]", inner.iter().join(", ")),
            Constant::String(inner) => write!(f, "{inner}"),
        }
    }
}

impl Constant {
    /// Interprets Floats and Ints as an Int, if possible
    pub fn to_int(&self) -> Option<i64> {
        Some(match self {
            Constant::Int(inner) => *inner,
            Constant::Float(inner) if (*inner as i64) as f64 == *inner => *inner as i64,
            _ => None?,
        })
    }

    /// Interprets Floats and Ints as a Float
    pub fn to_float(&self) -> Option<f64> {
        Some(match self {
            Constant::Int(inner) => *inner as f64,
            Constant::Float(inner) => *inner,
            _ => None?,
        })
    }

    pub fn concat(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Constant::Array(lhs), Constant::Array(rhs)) => {
                Constant::Array(Vec::from_iter(lhs.iter().chain(rhs.iter()).cloned()))
            }
            (Constant::String(lhs), Constant::String(rhs)) => {
                Constant::String(lhs.to_owned() + rhs)
            }
            _ => return None,
        })
    }

    pub fn add(&self, rhs: &Self) -> Option<Self> {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Some(Self::Int(lhs.wrapping_add(*rhs))),
            (Self::Float(lhs), Self::Float(rhs)) => Some(Self::Float(lhs + rhs)),
            _ => None,
        }
    }

    pub fn subtract(&self, rhs: &Self) -> Option<Self> {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Some(Self::Int(lhs.wrapping_sub(*rhs))),
            (Self::Float(lhs), Self::Float(rhs)) => Some(Self::Float(lhs + rhs)),
            _ => None,
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Option<Self> {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Some(Self::Int(lhs.wrapping_mul(*rhs))),
            (Self::Float(lhs), Self::Float(rhs)) => Some(Self::Float(lhs * rhs)),
            _ => None,
        }
    }

    /// This operation always returns a Float, even when called with Integer arguments.
    pub fn float_divide(&self, rhs: &Self) -> Option<Self> {
        Some(Constant::Float(self.to_float()? / rhs.to_float()?))
    }

    pub fn int_divide(&self, rhs: &Self) -> Option<Self> {
        Some(Constant::Int(self.to_int()? / rhs.to_int()?))
    }

    pub fn remainder(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Constant::Int(lhs), Constant::Int(rhs)) => Constant::Int(lhs % rhs),
            (lhs, rhs) => Constant::Float(lhs.to_float()? % rhs.to_float()?),
        })
    }

    pub fn exponentiate(&self, rhs: &Self) -> Option<Self> {
        match (self, rhs) {
            (Constant::Int(_), Constant::Int(_)) => todo!(),
            (Constant::Int(_), Constant::Float(_)) => todo!(),
            (Constant::Float(_), Constant::Int(_)) => todo!(),
            (Constant::Float(_), Constant::Float(_)) => todo!(),
            _ => None,
        }
    }
}
