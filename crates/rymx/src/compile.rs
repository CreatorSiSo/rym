pub mod bytecode;
mod r#type;
pub use r#type::Type;
mod value;
pub use value::Value;

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

#[cfg(target_os)]
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
			compiler.compile_module(Symbol::GLOBAL, &module);
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
