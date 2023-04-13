use rym_ast::{mut_visitor::*, *};

pub struct ValidationPass {
	diagnostics: Vec<()>,
}

impl ValidationPass {
	pub fn new() -> Self {
		Self {
			diagnostics: vec![],
		}
	}
}

impl MutVisitor for ValidationPass {
	type Result = ();

	fn visit_unary(&mut self, outer_expr: &mut Spanned<Expr>) {
		let mut nested_levels = 1;
		let mut diagnostic_span = outer_expr.1.clone();

		while let Expr::Unary(outer_op, ref inner_expr) = outer_expr.0 {
			let Expr::Unary(inner_op, ref inner_inner_expr) = inner_expr.0 else {
				break;
			};
			if inner_op != outer_op {
				break;
			}
			diagnostic_span.end = inner_inner_expr.1.start + 1;
			nested_levels += 1;
			*outer_expr = (**inner_expr).clone();
		}

		if nested_levels > 1 {
			// dbg!(diagnostic_span);
			// TODO Create diagnostic
		}

		let Expr::Unary(_, inner_expr) = &mut outer_expr.0 else {
			unreachable!("Internal Error: Expected unary expression, got {outer_expr:?}");
		};

		if nested_levels % 2 == 0 {
			// even number of unary operations
			// => all of them cancel each other out
			*outer_expr = (**inner_expr).clone();
			walk_expr(self, outer_expr);
		} else {
			// odd number of unary opertions
			// => only one remains
			walk_expr(self, inner_expr);
		}
	}
}
