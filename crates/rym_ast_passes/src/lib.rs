use rym_ast::*;

pub struct NodeCounter {
	pub count: usize,
}

impl NodeCounter {
	pub fn new() -> NodeCounter {
		NodeCounter { count: 0 }
	}
}

impl Visitor for NodeCounter {
	type Result = ();

	fn visit_module(&mut self, Spanned(module, _): &Spanned<Module>) -> Self::Result {
		self.count += 1;
		for item in &module.items {
			self.walk_item(item);
		}
	}

	fn visit_func(&mut self, Spanned(func, _): &Spanned<Func>) -> Self::Result {
		self.count += 1;
		if let Some(rhs) = &func.rhs {
			self.walk_expr(rhs);
		}
	}

	fn visit_var(&mut self, Spanned(var, _): &Spanned<Var>) -> Self::Result {
		self.count += 1;
		self.walk_expr(&var.rhs);
	}

	fn visit_ident(&mut self, _ident: &Spanned<String>) -> Self::Result {
		self.count += 1;
	}

	fn visit_literal(&mut self, _literal: &Spanned<Literal>) -> Self::Result {
		self.count += 1;
	}

	fn visit_record(
		&mut self,
		_name: &Option<Spanned<String>>,
		fields: &[(Spanned<String>, Spanned<Expr>)],
	) -> Self::Result {
		self.count += 1;
		for (_, expr) in fields {
			self.walk_expr(expr);
		}
	}

	fn visit_group(&mut self, group: &Spanned<Expr>) -> Self::Result {
		self.count += 1;
		self.walk_expr(group);
	}

	fn visit_block(&mut self, Spanned(stmts, _): &Spanned<Vec<Spanned<Stmt>>>) -> Self::Result {
		self.count += 1;
		for stmt in stmts {
			self.walk_stmt(stmt);
		}
	}

	fn visit_if(
		&mut self,
		condition: &Spanned<Expr>,
		then_branch: &Spanned<Expr>,
		else_branch: &Option<Spanned<Expr>>,
	) -> Self::Result {
		self.count += 1;
		self.walk_expr(condition);
		self.walk_expr(then_branch);
		if let Some(else_expr) = else_branch {
			self.walk_expr(else_expr);
		}
	}

	fn visit_loop(&mut self, expr: &Spanned<Expr>) -> Self::Result {
		self.count += 1;
		self.walk_expr(expr);
	}

	fn visit_continue(&mut self, _span: Span) -> Self::Result {
		self.count += 1;
	}

	fn visit_break(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> Self::Result {
		self.count += 1;
		if let Some(expr) = maybe_expr {
			self.walk_expr(expr);
		}
	}

	fn visit_return(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> Self::Result {
		self.count += 1;
		if let Some(expr) = maybe_expr {
			self.walk_expr(expr);
		}
	}

	fn visit_unary(&mut self, _op: UnaryOp, expr: &Spanned<Expr>) -> Self::Result {
		self.count += 1;
		self.walk_expr(expr);
	}

	fn visit_binary(
		&mut self,
		expr_l: &Spanned<Expr>,
		_op: BinaryOp,
		expr_r: &Spanned<Expr>,
	) -> Self::Result {
		self.count += 1;
		self.walk_expr(expr_l);
		self.walk_expr(expr_r);
	}

	fn visit_logical(
		&mut self,
		expr_l: &Spanned<Expr>,
		_op: LogicalOp,
		expr_r: &Spanned<Expr>,
	) -> Self::Result {
		self.count += 1;
		self.walk_expr(expr_l);
		self.walk_expr(expr_r);
	}

	fn visit_assign(&mut self, expr_l: &Spanned<Expr>, expr_r: &Spanned<Expr>) -> Self::Result {
		self.count += 1;
		self.walk_expr(expr_l);
		self.walk_expr(expr_r);
	}

	fn visit_call(
		&mut self,
		func: &Spanned<Expr>,
		Spanned(args, _): &Spanned<Vec<Spanned<Expr>>>,
	) -> Self::Result {
		self.count += 1;
		self.walk_expr(func);
		for expr in args {
			self.walk_expr(expr);
		}
	}

	fn visit_error(&mut self, _span: Span) -> Self::Result {}
}
