use crate::{
	BinaryOp, Expr, Func, Item, Literal, LogicalOp, Module, Span, Spanned, Stmt, UnaryOp, Var,
};

pub trait Visitor {
	type Result;

	fn walk_item(&mut self, Spanned(item, _): &Spanned<Item>) -> Self::Result {
		match item {
			Item::Module(module) => self.visit_module(module),
			Item::Func(func) => self.visit_func(func),
			Item::Var(var) => self.visit_var(var),
		}
	}

	fn visit_module(&mut self, module: &Spanned<Module>) -> Self::Result;
	fn visit_func(&mut self, func: &Spanned<Func>) -> Self::Result;
	fn visit_var(&mut self, var: &Spanned<Var>) -> Self::Result;

	fn walk_stmt(&mut self, stmt: &Spanned<Stmt>) -> Self::Result {
		let Spanned(stmt, span) = stmt;
		match stmt {
			Stmt::Item(item) => self.walk_item(item),
			Stmt::Expr(expr) => self.walk_expr(expr),
			Stmt::Error => self.visit_error(span.clone()),
		}
	}

	fn walk_expr(&mut self, expr: &Spanned<Expr>) -> Self::Result {
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

	fn visit_ident(&mut self, ident: &Spanned<String>) -> Self::Result;
	fn visit_literal(&mut self, literal: &Spanned<Literal>) -> Self::Result;

	fn visit_record(
		&mut self,
		name: &Option<Spanned<String>>,
		fields: &[(Spanned<String>, Spanned<Expr>)],
	) -> Self::Result;

	fn visit_group(&mut self, group: &Spanned<Expr>) -> Self::Result;
	fn visit_block(&mut self, block: &Spanned<Vec<Spanned<Stmt>>>) -> Self::Result;

	fn visit_if(
		&mut self,
		condition: &Spanned<Expr>,
		then_branch: &Spanned<Expr>,
		else_branch: &Option<Spanned<Expr>>,
	) -> Self::Result;

	fn visit_loop(&mut self, expr: &Spanned<Expr>) -> Self::Result;
	fn visit_continue(&mut self, span: Span) -> Self::Result;
	fn visit_break(&mut self, expr: &Option<Spanned<Expr>>) -> Self::Result;
	fn visit_return(&mut self, expr: &Option<Spanned<Expr>>) -> Self::Result;

	fn visit_unary(&mut self, op: UnaryOp, expr: &Spanned<Expr>) -> Self::Result;
	fn visit_binary(
		&mut self,
		expr_l: &Spanned<Expr>,
		op: BinaryOp,
		expr_r: &Spanned<Expr>,
	) -> Self::Result;
	fn visit_logical(
		&mut self,
		expr_l: &Spanned<Expr>,
		op: LogicalOp,
		expr_r: &Spanned<Expr>,
	) -> Self::Result;
	fn visit_assign(&mut self, expr_l: &Spanned<Expr>, expr_r: &Spanned<Expr>) -> Self::Result;

	fn visit_call(
		&mut self,
		func: &Spanned<Expr>,
		args: &Spanned<Vec<Spanned<Expr>>>,
	) -> Self::Result;

	fn visit_error(&mut self, span: Span) -> Self::Result;
}
