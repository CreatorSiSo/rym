use crate::{BinaryOp, Block, Decl, Expr, Identifier, Literal, LogicalOp, Stmt, UnaryOp};

pub trait AstVisitor {
	type Result;

	fn walk_stmt(&mut self, stmt: &Stmt) -> Self::Result {
		match stmt {
			Stmt::Empty => self.visit_empty(),
			Stmt::Decl(decl) => self.visit_decl(decl),
			Stmt::Expr(expr) => self.walk_expr(expr),
		}
	}

	fn visit_empty(&mut self) -> Self::Result;

	fn visit_decl(&mut self, decl: &Decl) -> Self::Result;

	fn walk_expr(&mut self, expr: &Expr) -> Self::Result {
		match expr {
			Expr::Identifier(ident) => self.visit_ident(ident),
			Expr::Literal(lit) => self.visit_lit(lit),
			Expr::Assign(expr_l, expr_r) => self.visit_assign(expr_l, expr_r),
			Expr::Call(callee, args) => self.visit_call(callee, args),

			Expr::Unary(op, expr) => self.visit_unary(op, expr),
			Expr::Logical(expr_l, op, expr_r) => self.visit_logical(expr_l, op, expr_r),
			Expr::Binary(expr_l, op, expr_r) => self.visit_binary(expr_l, op, expr_r),

			Expr::Group(expr) => self.walk_expr(expr),
			Expr::Block(block) => self.visit_block(block),
			Expr::Loop(block) => self.visit_loop(block),
			Expr::If(expr, then_block, else_block) => self.visit_if(expr, then_block, else_block),

			Expr::Break(expr) => self.visit_break(&**expr),
			Expr::Continue => self.visit_continue(),

			_ => panic!("Not yet implemented: {:?}", expr),
		}
	}

	fn visit_ident(&mut self, ident: &Identifier) -> Self::Result;
	fn visit_lit(&mut self, lit: &Literal) -> Self::Result;

	fn visit_assign(&mut self, expr_l: &Expr, expr_r: &Expr) -> Self::Result;
	fn visit_call(&mut self, callee: &Expr, args: &[Expr]) -> Self::Result;
	fn visit_unary(&mut self, op: &UnaryOp, expr: &Expr) -> Self::Result;
	fn visit_logical(&mut self, expr_l: &Expr, op: &LogicalOp, expr_r: &Expr) -> Self::Result;
	fn visit_binary(&mut self, expr_l: &Expr, op: &BinaryOp, expr_r: &Expr) -> Self::Result;

	fn visit_block(&mut self, block: &Block) -> Self::Result;
	fn visit_loop(&mut self, block: &Block) -> Self::Result;
	fn visit_if(
		&mut self,
		expr: &Expr,
		then_block: &Block,
		else_block: &Option<Block>,
	) -> Self::Result;

	fn visit_break(&mut self, expr: &Option<Expr>) -> Self::Result;
	fn visit_continue(&mut self) -> Self::Result;
}
