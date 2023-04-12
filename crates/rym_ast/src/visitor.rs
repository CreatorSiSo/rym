use crate::{
	BinaryOp, Expr, Func, Item, Literal, LogicalOp, Module, Span, Spanned, Stmt, UnaryOp, Var,
};

pub trait Visitor: Sized {
	type Result;

	fn visit_item(&mut self, item: &Spanned<Item>) -> Self::Result {
		walk_item(self, item)
	}

	fn visit_module(&mut self, module: &Spanned<Module>) -> Self::Result;
	fn visit_func(&mut self, func: &Spanned<Func>) -> Self::Result;
	fn visit_var(&mut self, var: &Spanned<Var>) -> Self::Result;

	fn visit_stmt(&mut self, stmt: &Spanned<Stmt>) -> Self::Result {
		walk_stmt(self, stmt)
	}

	fn visit_expr(&mut self, expr: &Spanned<Expr>) -> Self::Result {
		walk_expr(self, expr)
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

pub fn walk_items<'a, V: Visitor>(
	visitor: &'a mut V,
	items: &'a [Spanned<Item>],
) -> impl Iterator<Item = V::Result> + 'a {
	items.iter().map(|item| walk_item(visitor, item))
}

pub fn walk_item<V: Visitor>(visitor: &mut V, Spanned(item, _): &Spanned<Item>) -> V::Result {
	match item {
		Item::Module(module) => visitor.visit_module(module),
		Item::Func(func) => visitor.visit_func(func),
		Item::Var(var) => visitor.visit_var(var),
	}
}

pub fn walk_stmts<'a, V: Visitor>(
	visitor: &'a mut V,
	stmts: &'a [Spanned<Stmt>],
) -> impl Iterator<Item = V::Result> + 'a {
	stmts.iter().map(|stmt| walk_stmt(visitor, stmt))
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &Spanned<Stmt>) -> V::Result {
	let Spanned(stmt, span) = stmt;
	match stmt {
		Stmt::Item(item) => visitor.visit_item(item),
		Stmt::Expr(expr) => visitor.visit_expr(expr),
		Stmt::Error => visitor.visit_error(span.clone()),
	}
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Spanned<Expr>) -> V::Result {
	let Spanned(expr, span) = expr;
	match expr {
		Expr::Ident(ident) => visitor.visit_ident(ident),
		Expr::Literal(literal) => visitor.visit_literal(literal),
		Expr::Record { name, fields } => visitor.visit_record(name, fields),

		Expr::Group(group) => visitor.visit_group(group),
		Expr::Block(block) => visitor.visit_block(block),

		Expr::If {
			condition,
			then_branch,
			else_branch,
		} => visitor.visit_if(condition, then_branch, else_branch),

		Expr::Loop(expr) => visitor.visit_loop(expr),
		Expr::Continue => visitor.visit_continue(span.clone()),
		Expr::Break(expr) => visitor.visit_break(expr),
		Expr::Return(expr) => visitor.visit_return(expr),

		Expr::Unary(op, expr) => visitor.visit_unary(*op, expr),
		Expr::Binary(expr_l, op, expr_r) => visitor.visit_binary(expr_l, *op, expr_r),
		Expr::Logical(expr_l, op, expr_r) => visitor.visit_logical(expr_l, *op, expr_r),
		Expr::Assign(expr_l, expr_r) => visitor.visit_assign(expr_l, expr_r),

		Expr::Call { func, args } => visitor.visit_call(func, args),

		Expr::Error => visitor.visit_error(span.clone()),
	}
}
