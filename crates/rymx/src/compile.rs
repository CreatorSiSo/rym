pub mod bytecode;
mod r#type;
pub use r#type::Type;
mod constant;
pub use constant::Constant;
mod eval;
mod module;
mod symbol;

#[cfg(target_os)]
mod test {
	use super::{Compiler, Symbol, Type};
	use crate::{
		ast::{Expr, Literal, Module},
		compile::Constant,
		interpret::Function,
	};

	fn compile_modules<const N: usize>(modules: [Module; N]) -> Vec<(Symbol, (Type, Constant))> {
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
				(Type::Unknown, Constant::Unit)
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
				(
					Symbol::from(["std", "false"]),
					(Type::Bool, Constant::Int(0))
				),
				(
					Symbol::from(["std", "true"]),
					(Type::Bool, Constant::Int(1))
				),
				(
					Symbol::from(["std", "floats", "one"]),
					(Type::FloatLiteral, Constant::Float(1.0))
				),
				(
					Symbol::from(["std", "floats", "two"]),
					(Type::FloatLiteral, Constant::Float(2.0))
				),
				(
					Symbol::from(["std", "ints", "one"]),
					(Type::IntLiteral, Constant::Int(1))
				),
				(
					Symbol::from(["std", "ints", "two"]),
					(Type::IntLiteral, Constant::Int(2))
				),
			]
		);
	}
}
