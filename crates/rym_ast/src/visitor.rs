use crate::{
	BinaryOp, Expr, Func, Item, Literal, LogicalOp, Module, Span, Spanned, Stmt, UnaryOp, Var,
};

macro_rules! make_visitor {
	($trait_name:ident $(, $mutable:ident)?) => {
		pub trait $trait_name {
			type Result;

			fn walk_item(&mut self, Spanned(item, _): & $($mutable)? Spanned<Item>) -> Self::Result {
				match item {
					Item::Module(module) => self.visit_module(module),
					Item::Func(func) => self.visit_func(func),
					Item::Var(var) => self.visit_var(var),
				}
			}

			fn visit_module(&mut self, module: & $($mutable)? Spanned<Module>) -> Self::Result;
			fn visit_func(&mut self, func: & $($mutable)? Spanned<Func>) -> Self::Result;
			fn visit_var(&mut self, var: & $($mutable)? Spanned<Var>) -> Self::Result;

			fn walk_stmt(&mut self, stmt: & $($mutable)? Spanned<Stmt>) -> Self::Result {
				let Spanned(stmt, span) = stmt;
				match stmt {
					Stmt::Item(item) => self.walk_item(item),
					Stmt::Expr(expr) => self.walk_expr(expr),
					Stmt::Error => self.visit_error(span.clone()),
				}
			}

			fn walk_expr(&mut self, expr: & $($mutable)? Spanned<Expr>) -> Self::Result {
				let Spanned(expr, span) = expr;
				match expr {
					Expr::Ident(ident) => self.visit_ident(ident),
					Expr::Literal(literal) => self.visit_literal(literal),
					Expr::Record { name, fields } => self.visit_record(name, fields),

					Expr::Group(group) => self.visit_group(group),
					Expr::Block(block) => self.visit_block(block),

					Expr::If {
						condition,
						then_branch,
						else_branch,
					} => self.visit_if(condition, then_branch, else_branch),

					Expr::Loop(expr) => self.visit_loop(expr),
					Expr::Continue => self.visit_continue(span.clone()),
					Expr::Break(expr) => self.visit_break(expr),
					Expr::Return(expr) => self.visit_return(expr),

					Expr::Unary(op, expr) => self.visit_unary(*op, expr),
					Expr::Binary(expr_l, op, expr_r) => self.visit_binary(expr_l, *op, expr_r),
					Expr::Logical(expr_l, op, expr_r) => self.visit_logical(expr_l, *op, expr_r),
					Expr::Assign(expr_l, expr_r) => self.visit_assign(expr_l, expr_r),

					Expr::Call { func, args } => self.visit_call(func, args),

					Expr::Error => self.visit_error(span.clone()),
				}
			}

			fn visit_ident(&mut self, ident: & $($mutable)? Spanned<String>) -> Self::Result;
			fn visit_literal(&mut self, literal: & $($mutable)? Spanned<Literal>) -> Self::Result;

			fn visit_record(
				&mut self,
				name: & $($mutable)? Option<Spanned<String>>,
				fields: &[(Spanned<String>, Spanned<Expr>)],
			) -> Self::Result;

			fn visit_group(&mut self, group: & $($mutable)? Spanned<Expr>) -> Self::Result;
			fn visit_block(&mut self, block: & $($mutable)? Spanned<Vec<Spanned<Stmt>>>) -> Self::Result;

			fn visit_if(
				&mut self,
				condition: & $($mutable)? Spanned<Expr>,
				then_branch: & $($mutable)? Spanned<Expr>,
				else_branch: & $($mutable)? Option<Spanned<Expr>>,
			) -> Self::Result;

			fn visit_loop(&mut self, expr: & $($mutable)? Spanned<Expr>) -> Self::Result;
			fn visit_continue(&mut self, span: Span) -> Self::Result;
			fn visit_break(&mut self, expr: & $($mutable)? Option<Spanned<Expr>>) -> Self::Result;
			fn visit_return(&mut self, expr: & $($mutable)? Option<Spanned<Expr>>) -> Self::Result;

			fn visit_unary(&mut self, op: UnaryOp, expr: & $($mutable)? Spanned<Expr>) -> Self::Result;
			fn visit_binary(
				&mut self,
				expr_l: & $($mutable)? Spanned<Expr>,
				op: BinaryOp,
				expr_r: & $($mutable)? Spanned<Expr>,
			) -> Self::Result;
			fn visit_logical(
				&mut self,
				expr_l: & $($mutable)? Spanned<Expr>,
				op: LogicalOp,
				expr_r: & $($mutable)? Spanned<Expr>,
			) -> Self::Result;
			fn visit_assign(&mut self, expr_l: & $($mutable)? Spanned<Expr>, expr_r: & $($mutable)? Spanned<Expr>) -> Self::Result;

			fn visit_call(
				&mut self,
				func: & $($mutable)? Spanned<Expr>,
				args: & $($mutable)? Spanned<Vec<Spanned<Expr>>>,
			) -> Self::Result;

			fn visit_error(&mut self, span: Span) -> Self::Result;
		}
	};
}

make_visitor!(MutVisitor, mut);
make_visitor!(Visitor);
