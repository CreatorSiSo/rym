use rym_ast::visitor::*;
use rym_ast::*;

pub struct NodeCounter;

impl Visitor for NodeCounter {
	type Result = usize;

	fn visit_module(&mut self, Spanned(module, _): &Spanned<Module>) -> usize {
		1 + walk_items(self, &module.items).sum::<usize>()
	}

	fn visit_func(&mut self, Spanned(func, _): &Spanned<Func>) -> usize {
		1 + func.rhs.as_ref().map_or(0, |rhs| walk_expr(self, &rhs))
	}

	fn visit_var(&mut self, Spanned(var, _): &Spanned<Var>) -> usize {
		1 + walk_expr(self, &var.rhs)
	}

	fn visit_ident(&mut self, _ident: &Spanned<String>) -> usize {
		1
	}

	fn visit_literal(&mut self, _literal: &Spanned<Literal>) -> usize {
		1
	}

	fn visit_record(
		&mut self,
		_name: &Option<Spanned<String>>,
		fields: &[(Spanned<String>, Spanned<Expr>)],
	) -> usize {
		1 + fields
			.iter()
			.map(|(_, expr)| walk_expr(self, expr))
			.sum::<usize>()
	}

	fn visit_group(&mut self, group: &Spanned<Expr>) -> usize {
		1 + walk_expr(self, group)
	}

	fn visit_block(&mut self, Spanned(stmts, _): &Spanned<Vec<Spanned<Stmt>>>) -> usize {
		1 + walk_stmts(self, stmts).sum::<usize>()
	}

	fn visit_if(
		&mut self,
		condition: &Spanned<Expr>,
		then_branch: &Spanned<Expr>,
		else_branch: &Option<Spanned<Expr>>,
	) -> usize {
		1 + walk_expr(self, condition)
			+ walk_expr(self, then_branch)
			+ else_branch.as_ref().map_or(0, |expr| walk_expr(self, expr))
	}

	fn visit_loop(&mut self, expr: &Spanned<Expr>) -> usize {
		1 + walk_expr(self, expr)
	}

	fn visit_continue(&mut self, _span: Span) -> usize {
		1
	}

	fn visit_break(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> usize {
		1 + maybe_expr.as_ref().map_or(0, |expr| walk_expr(self, expr))
	}

	fn visit_return(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> usize {
		1 + maybe_expr.as_ref().map_or(0, |expr| walk_expr(self, expr))
	}

	fn visit_unary(&mut self, _op: UnaryOp, expr: &Spanned<Expr>) -> usize {
		1 + walk_expr(self, expr)
	}

	fn visit_binary(
		&mut self,
		expr_l: &Spanned<Expr>,
		_op: BinaryOp,
		expr_r: &Spanned<Expr>,
	) -> usize {
		1 + walk_expr(self, expr_l) + walk_expr(self, expr_r)
	}

	fn visit_logical(
		&mut self,
		expr_l: &Spanned<Expr>,
		_op: LogicalOp,
		expr_r: &Spanned<Expr>,
	) -> usize {
		1 + walk_expr(self, expr_l) + walk_expr(self, expr_r)
	}

	fn visit_assign(&mut self, expr_l: &Spanned<Expr>, expr_r: &Spanned<Expr>) -> usize {
		1 + walk_expr(self, expr_l) + walk_expr(self, expr_r)
	}

	fn visit_call(
		&mut self,
		func: &Spanned<Expr>,
		Spanned(args, _): &Spanned<Vec<Spanned<Expr>>>,
	) -> usize {
		1 + walk_expr(self, func) + args.iter().map(|arg| walk_expr(self, arg)).sum::<usize>()
	}

	fn visit_error(&mut self, _span: Span) -> usize {
		0
	}
}
