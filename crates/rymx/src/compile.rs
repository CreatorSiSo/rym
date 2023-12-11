use crate::ast::{Expr, Literal, Module};
use std::collections::HashMap;

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
	constants: HashMap<Symbol, Value>,
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

	pub fn compile_expr(&mut self, parent: &Symbol, expr: &Expr) -> Value {
		match expr {
			Expr::Unit => Value::Unit,
			Expr::Literal(lit) => Self::compile_literal(lit),
			Expr::Ident(_) => todo!(),
			Expr::Chain(_, _) => todo!(),
			Expr::ChainEnd(_) => todo!(),
			Expr::Function(_) => todo!(),
			Expr::Unary(_, _) => todo!(),
			Expr::Binary(_, _, _) => todo!(),
			Expr::Call(_, _) => todo!(),
			Expr::IfElse(_, _, _) => todo!(),
			Expr::Block(_) => todo!(),
			Expr::Break(_) => todo!(),
			Expr::Return(_) => todo!(),
			Expr::Var(_, _, _) => todo!(),
		}
	}

	pub fn compile_literal(lit: &Literal) -> Value {
		match lit {
			Literal::Bool(inner) => Value::Bool(*inner),
			Literal::Int(inner) => Value::Int(*inner),
			Literal::Float(inner) => Value::Float(*inner),
			Literal::String(inner) => Value::String(inner.clone()),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Value {
	Unit,
	Bool(bool),
	// TODO Use integers with arbiträry size
	Int(i64),
	// TODO Use floats with arbiträry size
	Float(f64),
	// TODO Use string interning
	String(String),
}

enum Instruction {}

#[cfg(test)]
mod test {
	use super::{Compiler, Symbol};
	use crate::{
		ast::{Expr, Literal, Module},
		compile::Value,
		interpret::Function,
	};

	fn compile_modules<const N: usize>(modules: [Module; N]) -> Vec<(Symbol, Value)> {
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
			vec![(Symbol::from(["hello_world", "main"]), Value::Unit),]
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
				(Symbol::from(["std", "false"]), Value::Bool(false)),
				(Symbol::from(["std", "true"]), Value::Bool(true)),
				(Symbol::from(["std", "floats", "one"]), Value::Float(1.0)),
				(Symbol::from(["std", "floats", "two"]), Value::Float(2.0)),
				(Symbol::from(["std", "ints", "one"]), Value::Int(1)),
				(Symbol::from(["std", "ints", "two"]), Value::Int(2)),
			]
		);
	}
}
