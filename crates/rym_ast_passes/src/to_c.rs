use rym_ast::*;
use visitor::*;

pub struct ToCPass {
	pub out: String,
}

impl ToCPass {
	pub fn new() -> Self {
		Self { out: String::new() }
	}
}

impl Visitor for ToCPass {
	type Result = ();

	fn visit_module(&mut self, module: &Spanned<Module>) -> Self::Result {
		walk_items(self, &module.0.items).for_each(drop);
	}

	fn visit_func(&mut self, func: &Spanned<Func>) -> Self::Result {
		todo!()
	}

	fn visit_var(&mut self, var: &Spanned<Var>) -> Self::Result {
		todo!()
	}

	fn visit_stmt(&mut self, stmt: &Spanned<Stmt>) -> Self::Result {
		walk_stmt(self, stmt);
		self.out += ";\n";
	}

	fn visit_ident(&mut self, Spanned(name, _): &Spanned<String>) -> Self::Result {
		self.out += name;
	}

	fn visit_literal(&mut self, Spanned(lit, _): &Spanned<Literal>) -> Self::Result {
		self.out += &lit.to_string();
	}

	fn visit_record(
		&mut self,
		name: &Option<Spanned<String>>,
		fields: &[(Spanned<String>, Spanned<Expr>)],
	) -> Self::Result {
		todo!()
	}

	fn visit_group(&mut self, expr: &Spanned<Expr>) -> Self::Result {
		self.out += "(";
		self.visit_expr(expr);
		self.out += ")";
	}

	fn visit_block(&mut self, block: &Spanned<Vec<Spanned<Stmt>>>) -> Self::Result {
		self.out += "{\n";
		walk_stmts(self, &block.0).for_each(drop);
		self.out += "}";
	}

	fn visit_if(
		&mut self,
		condition: &Spanned<Expr>,
		then_branch: &Spanned<Expr>,
		else_branch: &Option<Spanned<Expr>>,
	) -> Self::Result {
		todo!()
	}

	fn visit_loop(&mut self, expr: &Spanned<Expr>) -> Self::Result {
		self.out += "while (1) ";
		self.visit_expr(expr);
	}

	fn visit_continue(&mut self, _: Span) -> Self::Result {
		self.out += "continue";
	}

	fn visit_break(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> Self::Result {
		self.out += "break ";
		if let Some(expr) = maybe_expr {
			self.visit_expr(expr);
		}
	}

	fn visit_return(&mut self, maybe_expr: &Option<Spanned<Expr>>) -> Self::Result {
		self.out += "return ";
		if let Some(expr) = maybe_expr {
			self.visit_expr(expr);
		}
	}

	fn visit_unary(&mut self, op: UnaryOp, expr: &Spanned<Expr>) -> Self::Result {
		self.out += match op {
			UnaryOp::Not => " ! ",
			UnaryOp::Neg => "-",
		};
		self.visit_expr(expr);
	}

	fn visit_binary(
		&mut self,
		expr_l: &Spanned<Expr>,
		op: BinaryOp,
		expr_r: &Spanned<Expr>,
	) -> Self::Result {
		self.out += "(";
		self.visit_expr(expr_l);
		self.out += match op {
			BinaryOp::Add => " + ",
			BinaryOp::Sub => " - ",
			BinaryOp::Mul => " * ",
			BinaryOp::Div => " / ",
			BinaryOp::Rem => " % ",
			BinaryOp::Eq => " == ",
			BinaryOp::Ne => " != ",
			BinaryOp::Gt => " > ",
			BinaryOp::Ge => " >= ",
			BinaryOp::Lt => " < ",
			BinaryOp::Le => " <= ",
			BinaryOp::And => " && ",
			BinaryOp::Or => " || ",
		};
		self.visit_expr(expr_r);
		self.out += ")";
	}

	fn visit_assign(&mut self, expr_l: &Spanned<Expr>, expr_r: &Spanned<Expr>) -> Self::Result {
		self.visit_expr(expr_l);
		self.out += " = ";
		self.visit_expr(expr_r);
	}

	fn visit_call(
		&mut self,
		func: &Spanned<Expr>,
		args: &Spanned<Vec<Spanned<Expr>>>,
	) -> Self::Result {
		self.out += "(";
		self.visit_expr(func);
		self.out += ")(";
		for expr in &args.0 {
			self.visit_expr(expr);
			self.out += ", ";
		}
		self.out += ")";
	}

	fn visit_error(&mut self, span: Span) -> Self::Result {
		panic!("{span:?}");
	}
}
