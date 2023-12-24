use super::{
	module::{FunctionPrototype, ModulePrototype},
	Constant,
};
use crate::ast;
use std::cell::RefCell;

struct ConstEvaluator {
	prototype: RefCell<ModulePrototype>,
	ast_module: ast::Module,
}

impl ConstEvaluator {
	fn eval_module(ast_module: ast::Module) -> ModulePrototype {
		let this = ConstEvaluator {
			prototype: RefCell::new(ModulePrototype::new()),
			ast_module,
		};

		for (name, _, expr) in &this.ast_module.constants {
			this.eval_expr(expr);
		}

		// assert_eq!(this.typed_modules.len(), 1);
		this.prototype.into_inner()
	}

	fn eval_expr(&self, expr: &ast::Expr) -> Constant {
		match expr {
			ast::Expr::Unit => Constant::Unit,
			ast::Expr::Literal(lit) => Self::literal_to_constant(lit),
			ast::Expr::Ident(ident) => self
				.resolve_ident(ident)
				.expect(&format!("Could not find {ident}")),
			ast::Expr::Chain(_, _) => todo!(),
			ast::Expr::ChainEnd(_) => todo!(),
			ast::Expr::Function(_) => todo!(),
			ast::Expr::Unary(_, _) => todo!(),
			ast::Expr::Binary(_, _, _) => todo!(),
			ast::Expr::Call(_, _) => todo!(),
			ast::Expr::IfElse(_, _, _) => todo!(),
			ast::Expr::Block(_) => todo!(),
			ast::Expr::Break(_) => todo!(),
			ast::Expr::Return(_) => todo!(),
		}
	}

	fn eval_function(&self, func: FunctionPrototype) {}

	/// Search for a constant with this name and evaluate it if needed
	fn resolve_ident(&self, ident: &str) -> Option<Constant> {
		for (name, (_, val)) in &self.prototype.borrow().constants {
			(name == ident).then_some(val.clone())?;
		}
		for (name, _func) in &self.prototype.borrow().functions {
			if name == ident {
				todo!()
			}
		}

		None
	}

	fn literal_to_constant(lit: &ast::Literal) -> Constant {
		match lit {
			ast::Literal::Bool(inner) => Constant::Bool(*inner),
			ast::Literal::Int(inner) => Constant::Int(*inner),
			ast::Literal::Float(inner) => Constant::Float(*inner),
			ast::Literal::String(inner) => Constant::String(inner.clone()),
		}
	}
}
