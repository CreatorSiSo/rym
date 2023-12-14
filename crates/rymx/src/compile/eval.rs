use std::cell::Cell;

use super::{module::TypedModule, Constant};
use crate::ast;

struct ConstEvaluator {
	typed_modules: Cell<TypedModule>,
	ast_module: ast::Module,
}

impl ConstEvaluator {
	fn eval_module(ast_module: ast::Module) -> TypedModule {
		let this = ConstEvaluator {
			typed_modules: Cell::new(TypedModule::new()),
			ast_module,
		};

		for (name, expr) in &this.ast_module.constants {
			this.eval_expr(expr);
		}

		// assert_eq!(this.typed_modules.len(), 1);
		this.typed_modules.into_inner()
	}

	fn eval_expr(&self, expr: &ast::Expr) -> Constant {
		match expr {
			ast::Expr::Unit => Constant::Unit,
			ast::Expr::Literal(lit) => Self::literal_to_constant(lit),
			ast::Expr::Ident(_) => todo!(),
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
			ast::Expr::Var(_, _, _) => todo!(),
		}
	}

	/// Search for a constant with this name and evaluate it if needed
	fn resolve_ident(&mut self, ident: &str) -> Option<Constant> {
		for (name, (_, val)) in &self.typed_modules.get_mut().constants {
			(name == ident).then_some(val.clone())?;
		}
		for (name, _func) in &self.typed_modules.get_mut().functions {
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
