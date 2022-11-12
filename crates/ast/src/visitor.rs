use crate::{BinaryOp, Block, Decl, Expr, Literal, LogicalOp, Span, Spanned, Stmt, UnaryOp};

pub trait AstVisitor {
	type Result;

	fn walk_stmt(&mut self, Spanned(stmt, span): &Spanned<&Stmt>) -> Self::Result {
		match stmt {
			// TODO: Give Span to visit_empty
			Stmt::Empty => self.visit_empty(),
			Stmt::Decl(decl) => self.visit_decl(&Spanned(decl, span.clone())),
			Stmt::Expr(expr) => self.walk_expr(&Spanned(expr, span.clone())),
		}
	}

	fn visit_empty(&mut self) -> Self::Result;

	fn visit_decl(&mut self, decl: &Spanned<&Decl>) -> Self::Result;

	fn walk_expr(&mut self, Spanned(expr, span): &Spanned<&Expr>) -> Self::Result {
		match expr {
			Expr::Identifier(ident) => self.visit_ident(ident, span),
			Expr::Literal(lit) => self.visit_lit(lit),
			Expr::Assign(expr_l, expr_r) => self.visit_assign(expr_l, expr_r),
			Expr::Call(callee, args) => self.visit_call(callee, args),

			Expr::Unary(op, expr) => self.visit_unary(op, expr),
			Expr::Logical(expr_l, op, expr_r) => self.visit_logical(expr_l, op, expr_r),
			Expr::Binary(expr_l, op, expr_r) => self.visit_binary(expr_l, op, expr_r),

			Expr::Group(expr) => self.walk_expr(&Spanned(expr, span.clone())),
			Expr::Block(block) => self.visit_block(block),
			Expr::Loop(block) => self.visit_loop(block),
			Expr::If(expr, then_block, else_block) => self.visit_if(expr, then_block, else_block),

			Expr::Return(expr) => self.visit_return(&**expr),
			Expr::Break(expr) => self.visit_break(&**expr),
			Expr::Continue => self.visit_continue(),

			_ => panic!("Not yet implemented: {:?}", expr),
		}
	}

	fn visit_ident(&mut self, ident: &str, span: &Span) -> Self::Result;
	fn visit_lit(&mut self, lit: &Literal) -> Self::Result;

	fn visit_assign(&mut self, expr_l: &Expr, expr_r: &Expr) -> Self::Result;
	fn visit_call(&mut self, callee: &Expr, args: &[Spanned<Expr>]) -> Self::Result;
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

	fn visit_return(&mut self, expr: &Expr) -> Self::Result;
	fn visit_break(&mut self, expr: &Option<Expr>) -> Self::Result;
	fn visit_continue(&mut self) -> Self::Result;
}
