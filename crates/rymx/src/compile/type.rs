use super::Value;
use bumpalo::{collections::Vec, Bump};
use itertools::Itertools;

#[derive(PartialEq, Eq)]
pub enum Type<'a> {
	Unit,
	Never,
	Unknown,
	Bool,
	IntLiteral,
	Int(u8),
	ISize,
	UInt(u8),
	USize,
	FloatLiteral,
	Float(u8),
	Array(Option<usize>, &'a mut Type<'a>),
	Union(Vec<'a, Type<'a>>),
	Function {
		params: Vec<'a, (Type<'a>, Option<(&'a str, Value)>)>,
		return_type: &'a mut Type<'a>,
	},
}

impl Type<'_> {
	fn alloc<'a>(self, bump: &'a Bump) -> &'a mut Self {
		bump.alloc(self)
	}

	fn array_to_string(
		element_type: &Type,
		length: Option<usize>,
	) -> Result<String, std::fmt::Error> {
		use std::fmt::Write;

		let mut result = String::new();
		if let Some(length) = length {
			write!(result, "[{length}]")?;
		} else {
			write!(result, "[]")?;
		}
		match element_type {
			union @ Type::Union { .. } => write!(result, "({union})")?,
			_ => write!(result, "{element_type}")?,
		}
		Ok(result)
	}
}

impl std::fmt::Debug for Type<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.to_string().as_str())
	}
}

impl std::fmt::Display for Type<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Unit => write!(f, "()"),
			Type::Never => write!(f, "never"),
			Type::Unknown => write!(f, "<unkown>"),
			Type::Bool => write!(f, "bool"),
			Type::IntLiteral => write!(f, "<int_lit>"),
			Type::Int(size) => write!(f, "i{size}"),
			Type::ISize => write!(f, "isize"),
			Type::UInt(size) => write!(f, "u{size}"),
			Type::USize => write!(f, "usize"),
			Type::FloatLiteral => write!(f, "<float_lit>"),
			Type::Float(size) => write!(f, "f{size}"),
			Type::Array(length, element_type) => {
				write!(f, "{}", Self::array_to_string(&**element_type, *length)?)
			}
			Type::Union(types) => write!(f, "{}", types.iter().join(" | ")),
			Type::Function {
				params,
				return_type,
			} => {
				write!(
					f,
					"fn({}) {return_type}",
					params
						.iter()
						.map(|(typ, named)| {
							if let Some((name, value)) = named {
								format!("{name}: {typ} = {value}")
							} else {
								typ.to_string()
							}
						})
						.join(", ")
				)
			}
		}
	}
}

#[test]
fn display_types() {
	use bumpalo::vec;
	use num_bigint::BigInt;

	let bump = Bump::new();

	// TODO Make sure thses types never get constructed
	assert_eq!(&Type::Int(0).to_string(), "i0");
	assert_eq!(&Type::UInt(0).to_string(), "u0");
	assert_eq!(&Type::Float(0).to_string(), "f0");
	assert_eq!(&Type::Float(1).to_string(), "f1");
	assert_eq!(&Type::Float(2).to_string(), "f2");
	assert_eq!(&Type::Float(11).to_string(), "f11");
	// ... and so on

	assert_eq!(&Type::Int(1).to_string(), "i1");
	assert_eq!(&Type::Int(2).to_string(), "i2");
	assert_eq!(&Type::Int(8).to_string(), "i8");
	assert_eq!(&Type::Int(16).to_string(), "i16");
	assert_eq!(&Type::Int(32).to_string(), "i32");
	assert_eq!(&Type::Int(64).to_string(), "i64");
	assert_eq!(&Type::Int(100).to_string(), "i100");
	assert_eq!(&Type::Int(128).to_string(), "i128");

	assert_eq!(&Type::UInt(1).to_string(), "u1");
	assert_eq!(&Type::UInt(2).to_string(), "u2");
	assert_eq!(&Type::UInt(8).to_string(), "u8");
	assert_eq!(&Type::UInt(16).to_string(), "u16");
	assert_eq!(&Type::UInt(32).to_string(), "u32");
	assert_eq!(&Type::UInt(64).to_string(), "u64");
	assert_eq!(&Type::UInt(100).to_string(), "u100");
	assert_eq!(&Type::UInt(128).to_string(), "u128");

	assert_eq!(&Type::Float(16).to_string(), "f16");
	assert_eq!(&Type::Float(32).to_string(), "f32");
	assert_eq!(&Type::Float(64).to_string(), "f64");

	assert_eq!(
		&Type::Array(None, Type::Unit.alloc(&bump)).to_string(),
		"[]()"
	);
	assert_eq!(
		&Type::Array(
			None,
			Type::Array(None, Type::Unit.alloc(&bump)).alloc(&bump)
		)
		.to_string(),
		"[][]()"
	);
	assert_eq!(
		&Type::Array(
			Some(4),
			Type::Array(Some(32), Type::Float(64).alloc(&bump)).alloc(&bump),
		)
		.to_string(),
		"[4][32]f64"
	);
	assert_eq!(
		&Type::Array(
			None,
			Type::Union(vec![
				in &bump;
				Type::UInt(1),
				Type::UInt(8),
				Type::UInt(16)
			])
			.alloc(&bump),
		)
		.to_string(),
		"[](u1 | u8 | u16)"
	);
	assert_eq!(
		&Type::Array(
			None,
			Type::Array(
				Some(256),
				Type::Union(vec![
					in &bump;
					Type::UInt(1),
					Type::UInt(8),
					Type::UInt(16),
				])
				.alloc(&bump)
			)
			.alloc(&bump),
		)
		.to_string(),
		"[][256](u1 | u8 | u16)"
	);

	assert_eq!(
		&Type::Function {
			params: vec![
				in &bump;
				(Type::UInt(32), None),
				(Type::UInt(32), None),
				(
					Type::UInt(64),
					Some((&*bump.alloc_str("named"), Value::Int(BigInt::from(0))))
				)
			],
			return_type: Type::UInt(64).alloc(&bump),
		}
		.to_string(),
		"fn(u32, u32, named: u64 = 0) u64"
	);
}
