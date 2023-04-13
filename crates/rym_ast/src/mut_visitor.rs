use crate::{BinaryOp, Expr, Func, Item, Literal, LogicalOp, Module, Span, Spanned, Stmt, Var};

pub trait MutVisitor: Sized {
	type Result;

	fn visit_item(&mut self, item: &mut Spanned<Item>) {
		walk_item(self, item);
	}

	fn visit_module(&mut self, module: &mut Spanned<Module>) {
		walk_items(self, &mut module.0.items);
	}

	fn visit_func(&mut self, func: &mut Spanned<Func>) {
		func.0.rhs.as_mut().map(|expr| walk_expr(self, expr));
	}

	fn visit_var(&mut self, var: &mut Spanned<Var>) {
		walk_expr(self, &mut var.0.rhs);
	}

	fn visit_stmt(&mut self, stmt: &mut Spanned<Stmt>) {
		walk_stmt(self, stmt);
	}

	fn visit_expr(&mut self, expr: &mut Spanned<Expr>) {
		walk_expr(self, expr);
	}

	fn visit_ident(&mut self, _ident: &mut Spanned<String>) {}
	fn visit_literal(&mut self, _literal: &mut Spanned<Literal>) {}

	fn visit_record(
		&mut self,
		_name: &mut Option<Spanned<String>>,
		fields: &mut [(Spanned<String>, Spanned<Expr>)],
	) {
		for (_, expr) in fields {
			walk_expr(self, expr)
		}
	}

	fn visit_group(&mut self, expr: &mut Spanned<Expr>) {
		walk_expr(self, expr);
	}
	fn visit_block(&mut self, block: &mut Spanned<Vec<Spanned<Stmt>>>) {
		walk_stmts(self, &mut block.0);
	}

	fn visit_if(
		&mut self,
		condition: &mut Spanned<Expr>,
		then_branch: &mut Spanned<Expr>,
		else_branch: &mut Option<Spanned<Expr>>,
	) {
		walk_expr(self, condition);
		walk_expr(self, then_branch);
		else_branch.as_mut().map(|expr| walk_expr(self, expr));
	}

	fn visit_loop(&mut self, expr: &mut Spanned<Expr>) {
		walk_expr(self, expr);
	}

	fn visit_continue(&mut self, _span: Span) {}
	fn visit_break(&mut self, expr: &mut Option<Spanned<Expr>>) {
		expr.as_mut().map(|expr| walk_expr(self, expr));
	}
	fn visit_return(&mut self, expr: &mut Option<Spanned<Expr>>) {
		expr.as_mut().map(|expr| walk_expr(self, expr));
	}

	fn visit_unary(&mut self, expr: &mut Spanned<Expr>) {
		let Expr::Unary(_op, expr) = &mut expr.0 else {
			return;
		};
		walk_expr(self, &mut *expr);
	}
	fn visit_binary(
		&mut self,
		expr_l: &mut Spanned<Expr>,
		_op: BinaryOp,
		expr_r: &mut Spanned<Expr>,
	) {
		walk_expr(self, expr_l);
		walk_expr(self, expr_r);
	}
	fn visit_logical(
		&mut self,
		expr_l: &mut Spanned<Expr>,
		_op: LogicalOp,
		expr_r: &mut Spanned<Expr>,
	) {
		walk_expr(self, expr_l);
		walk_expr(self, expr_r);
	}
	fn visit_assign(&mut self, expr_l: &mut Spanned<Expr>, expr_r: &mut Spanned<Expr>) {
		walk_expr(self, expr_l);
		walk_expr(self, expr_r);
	}

	fn visit_call(&mut self, func: &mut Spanned<Expr>, args: &mut Spanned<Vec<Spanned<Expr>>>) {
		walk_expr(self, func);
		for arg in args.0.iter_mut() {
			walk_expr(self, arg);
		}
	}

	fn visit_error(&mut self, _span: Span) {}
}

pub fn walk_items<'a, V: MutVisitor>(visitor: &'a mut V, items: &'a mut [Spanned<Item>]) {
	for item in items.iter_mut() {
		walk_item(visitor, item)
	}
}

pub fn walk_item<V: MutVisitor>(visitor: &mut V, Spanned(item, _): &mut Spanned<Item>) {
	match item {
		Item::Module(module) => visitor.visit_module(module),
		Item::Func(func) => visitor.visit_func(func),
		Item::Var(var) => visitor.visit_var(var),
	}
}

pub fn walk_stmts<V: MutVisitor>(visitor: &mut V, stmts: &mut [Spanned<Stmt>]) {
	for stmt in stmts.iter_mut() {
		walk_stmt(visitor, stmt);
	}
}

pub fn walk_stmt<V: MutVisitor>(visitor: &mut V, stmt: &mut Spanned<Stmt>) {
	let Spanned(stmt, span) = stmt;
	match stmt {
		Stmt::Item(item) => visitor.visit_item(item),
		Stmt::Expr(expr) => visitor.visit_expr(expr),
		Stmt::Error => visitor.visit_error(span.clone()),
	}
}

pub fn walk_expr<V: MutVisitor>(visitor: &mut V, outer_expr: &mut Spanned<Expr>) {
	let Spanned(expr, span) = outer_expr;
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

		Expr::Unary(..) => visitor.visit_unary(outer_expr),
		Expr::Binary(expr_l, op, expr_r) => visitor.visit_binary(expr_l, *op, expr_r),
		Expr::Logical(expr_l, op, expr_r) => visitor.visit_logical(expr_l, *op, expr_r),
		Expr::Assign(expr_l, expr_r) => visitor.visit_assign(expr_l, expr_r),

		Expr::Call { func, args } => visitor.visit_call(func, args),

		Expr::Error => visitor.visit_error(span.clone()),
	}
}
