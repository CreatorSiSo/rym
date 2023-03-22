use crate::{BinaryOp, Block, Expr, Item, Literal, LogicalOp, Span, Spanned, Stmt, UnaryOp};

pub trait AstVisitor {
	type Result;

	fn walk_stmt(&mut self, Spanned(stmt, span): Spanned<&Stmt>) -> Self::Result {
		match stmt {
			Stmt::Error => todo!("Stmt::Error at {span:?}"),
			Stmt::Item(item) => self.visit_item(item.as_ref()),
			Stmt::Expr(expr) => self.walk_expr(expr.as_ref()),
		}
	}

	fn visit_item(&mut self, decl: Spanned<&Item>) -> Self::Result;

	fn walk_expr(&mut self, Spanned(expr, span): Spanned<&Expr>) -> Self::Result {
		match expr {
			Expr::Ident(ident) => self.visit_ident(ident, span),
			Expr::Literal(lit) => self.visit_literal(lit),
			Expr::Assign(expr_l, expr_r) => self.visit_assign((**expr_l).as_ref(), (**expr_r).as_ref()),
			Expr::Call { callee, args } => self.visit_call((**callee).as_ref(), args),

			Expr::Unary(op, expr) => self.visit_unary(op, (**expr).as_ref()),
			Expr::Logical(expr_l, op, expr_r) => {
				self.visit_logical((**expr_l).as_ref(), op, (**expr_r).as_ref())
			}
			Expr::Binary(expr_l, op, expr_r) => {
				self.visit_binary((**expr_l).as_ref(), op, (**expr_r).as_ref())
			}

			Expr::Group(expr) => self.walk_expr((**expr).as_ref()),
			Expr::Block(block) => self.visit_block(block.as_ref()),
			Expr::Loop(block) => self.visit_loop(block.as_ref()),
			Expr::If(expr, then_block, else_block) => {
				self.visit_if((**expr).as_ref(), then_block.as_ref(), else_block)
			}

			Expr::Return(expr) => self.visit_return((**expr).as_ref()),
			Expr::Break(expr) => self.visit_break(expr.as_deref().map(|expr| expr.as_ref())),
			Expr::Continue => self.visit_continue(),

			_ => todo!("{:?}", expr),
		}
	}

	fn visit_ident(&mut self, ident: &str, span: Span) -> Self::Result;
	fn visit_literal(&mut self, lit: &Literal) -> Self::Result;

	fn visit_assign(&mut self, expr_l: Spanned<&Expr>, expr_r: Spanned<&Expr>) -> Self::Result;
	fn visit_call(&mut self, callee: Spanned<&Expr>, args: &[Spanned<Expr>]) -> Self::Result;
	fn visit_unary(&mut self, op: &UnaryOp, expr: Spanned<&Expr>) -> Self::Result;
	fn visit_logical(
		&mut self,
		expr_l: Spanned<&Expr>,
		op: &LogicalOp,
		expr_r: Spanned<&Expr>,
	) -> Self::Result;
	fn visit_binary(
		&mut self,
		expr_l: Spanned<&Expr>,
		op: &BinaryOp,
		expr_r: Spanned<&Expr>,
	) -> Self::Result;

	fn visit_block(&mut self, block: Spanned<&Block>) -> Self::Result;
	fn visit_loop(&mut self, block: Spanned<&Block>) -> Self::Result;
	fn visit_if(
		&mut self,
		expr: Spanned<&Expr>,
		then_block: Spanned<&Block>,
		else_block: &Option<Spanned<Block>>,
	) -> Self::Result;

	fn visit_return(&mut self, expr: Spanned<&Expr>) -> Self::Result;
	fn visit_break(&mut self, expr: Option<Spanned<&Expr>>) -> Self::Result;
	fn visit_continue(&mut self) -> Self::Result;
}
