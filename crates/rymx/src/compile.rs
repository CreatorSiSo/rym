use crate::ast::{Expr, Literal, Module, UnaryOp};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Write};

pub mod bytecode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Symbol(Vec<String>);

impl Symbol {
	pub const GLOBAL: Self = Symbol(vec![]);

	pub fn with_child(&self, child: impl Into<String>) -> Self {
		let mut clone = self.clone();
		clone.0.push(child.into());
		clone
	}
}

impl Ord for Symbol {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.len().cmp(&other.0.len()).then(self.0.cmp(&other.0))
	}
}

impl<I: IntoIterator<Item = T>, T: Into<String>> From<I> for Symbol {
	fn from(iter: I) -> Self {
		Symbol(iter.into_iter().map(Into::into).collect())
	}
}

pub struct Compiler {
	constants: HashMap<Symbol, (Type, Value)>,
}

impl Compiler {
	pub fn new() -> Self {
		Self {
			constants: HashMap::new(),
		}
	}

	/// Compiles the provided and all nested modules.
	///
	/// Not using recusion here as there is no depth limit for nested modules
	/// and a stack overflow really should not happen here!
	pub fn compile_module(&mut self, parent: Symbol, module: Module) {
		let mut modules = vec![(parent.with_child(&module.name), &module)];

		while !modules.is_empty() {
			modules = modules
				.into_iter()
				.flat_map(|(parent, module): (Symbol, &Module)| {
					for (name, expr) in &module.constants {
						let symbol = parent.with_child(name);
						let value = self.compile_expr(&parent, expr);
						let Some(prev) = self.constants.insert(symbol, value) else {
							continue;
						};
						todo!("Overriding: {prev:?}");
					}

					module
						.children
						.iter()
						.map(move |module| (parent.with_child(&module.name), module))
				})
				.collect();
		}
	}

	pub fn compile_expr(&mut self, parent: &Symbol, expr: &Expr) -> (Type, Value) {
		match expr {
			Expr::Unit => (Type::Unit, Value::Unit),
			Expr::Literal(lit) => Self::compile_literal(lit),
			Expr::Ident(_) => (Type::Unknown, Value::Unit),
			Expr::Chain(_, _) => (Type::Unknown, Value::Unit),
			Expr::ChainEnd(_) => (Type::Unknown, Value::Unit),
			Expr::Function(_) => (Type::Unknown, Value::Unit),
			Expr::Unary(op, inner) => self.compile_unary(parent, *op, inner),
			Expr::Binary(_, _, _) => (Type::Unknown, Value::Unit),
			Expr::Call(_, _) => (Type::Unknown, Value::Unit),
			Expr::IfElse(_, _, _) => (Type::Unknown, Value::Unit),
			Expr::Block(_) => (Type::Unknown, Value::Unit),
			Expr::Break(_) => (Type::Unknown, Value::Unit),
			Expr::Return(_) => (Type::Unknown, Value::Unit),
			Expr::Var(_, _, _) => (Type::Unit, Value::Unit),
		}
	}

	pub fn compile_unary(&mut self, parent: &Symbol, op: UnaryOp, expr: &Expr) -> (Type, Value) {
		let (typ, value) = self.compile_expr(parent, expr);
		match typ {
			Type::Never => (Type::Never, Value::Unit),
			Type::Bool => (Type::Bool, value),
			_ => panic!(),
		}
	}

	pub fn compile_literal(lit: &Literal) -> (Type, Value) {
		match lit {
			Literal::Bool(inner) => (Type::Bool, Value::Int(if *inner { 1 } else { 0 })),
			Literal::Int(inner) => (Type::IntLiteral, Value::Int(*inner)),
			Literal::Float(inner) => (Type::FloatLiteral, Value::Float(*inner)),
			// Literal::String(inner) => (
			// 	Type::Array(None, Type::UInt(8).into()).into(),
			// 	Value::String(inner.clone()),
			// ),
			_ => todo!(),
		}
	}
}

#[derive(PartialEq)]
pub enum Type {
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
	Array(Option<usize>, Box<Type>),
	Union(Vec<Type>),
}

impl Type {
	fn array_to_string(
		element_type: &Type,
		length: Option<usize>,
	) -> Result<String, std::fmt::Error> {
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

impl std::fmt::Debug for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.to_string().as_str())
	}
}

impl std::fmt::Display for Type {
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
		}
	}
}

#[test]
fn display_types() {
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

	assert_eq!(&Type::Array(None, Type::Unit.into(),).to_string(), "[]()");
	assert_eq!(
		&Type::Array(None, Type::Array(None, Type::Unit.into()).into(),).to_string(),
		"[][]()"
	);
	assert_eq!(
		&Type::Array(
			Some(4),
			Type::Array(Some(32), Type::Float(64).into()).into(),
		)
		.to_string(),
		"[4][32]f64"
	);
	assert_eq!(
		&Type::Array(
			None,
			Type::Union(vec![Type::UInt(1), Type::UInt(8), Type::UInt(16)]).into(),
		)
		.to_string(),
		"[](u1 | u8 | u16)"
	);
	assert_eq!(
		&Type::Array(
			None,
			Type::Array(
				Some(256),
				Type::Union(vec![Type::UInt(1), Type::UInt(8), Type::UInt(16)]).into()
			)
			.into(),
		)
		.to_string(),
		"[][256](u1 | u8 | u16)"
	);
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Value {
	Unit = 0,
	// TODO Use integers with arbiträry size
	Int(i64),
	// TODO Use floats with arbiträry size
	Float(f64),
	// TODO Use string interning
	// String(String),
}

#[cfg(test)]
mod test {
	use super::{Compiler, Symbol, Type};
	use crate::{
		ast::{Expr, Literal, Module},
		compile::Value,
		interpret::Function,
	};

	fn compile_modules<const N: usize>(modules: [Module; N]) -> Vec<(Symbol, (Type, Value))> {
		let mut compiler = Compiler::new();
		for module in modules {
			compiler.compile_module(Symbol::GLOBAL, module);
		}
		let mut constants: Vec<_> = compiler.constants.into_iter().collect();
		constants.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
		constants
	}

	#[test]
	fn hello_world() {
		let constants = compile_modules([Module {
			name: "hello_world".into(),
			constants: vec![(
				"main".into(),
				Expr::Function(Function {
					name: Some("main".into()),
					params: vec![],
					return_type: "Unit".into(),
					body: Box::new(Expr::Unit),
				}),
			)],
			children: vec![],
		}]);

		assert_eq!(
			constants,
			vec![(
				Symbol::from(["hello_world", "main"]),
				(Type::Unknown, Value::Unit)
			),]
		);
	}

	#[test]
	fn nested_modules() {
		let constants = compile_modules([Module {
			name: "std".into(),
			constants: vec![
				("true".into(), Expr::Literal(Literal::Bool(true))),
				("false".into(), Expr::Literal(Literal::Bool(false))),
			],
			children: vec![
				Module {
					name: "ints".into(),
					constants: vec![
						("one".into(), Expr::Literal(Literal::Int(1))),
						("two".into(), Expr::Literal(Literal::Int(2))),
					],
					children: vec![],
				},
				Module {
					name: "floats".into(),
					constants: vec![
						("one".into(), Expr::Literal(Literal::Float(1.0))),
						("two".into(), Expr::Literal(Literal::Float(2.0))),
					],
					children: vec![],
				},
			],
		}]);

		assert_eq!(
			constants,
			vec![
				(Symbol::from(["std", "false"]), (Type::Bool, Value::Int(0))),
				(Symbol::from(["std", "true"]), (Type::Bool, Value::Int(1))),
				(
					Symbol::from(["std", "floats", "one"]),
					(Type::FloatLiteral, Value::Float(1.0))
				),
				(
					Symbol::from(["std", "floats", "two"]),
					(Type::FloatLiteral, Value::Float(2.0))
				),
				(
					Symbol::from(["std", "ints", "one"]),
					(Type::IntLiteral, Value::Int(1))
				),
				(
					Symbol::from(["std", "ints", "two"]),
					(Type::IntLiteral, Value::Int(2))
				),
			]
		);
	}
}
