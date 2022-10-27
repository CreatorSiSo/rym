use super::*;

#[test]
fn parse_if_else_true() {
	let (tokens, errors) = tokens_from_src("if true { 2 } else { 1 }");
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![Stmt::Expr(Expr::If(
			Box::new(Expr::Literal(Literal::Bool(true))),
			Block::new(vec![Stmt::Expr(Expr::Literal(Literal::Number(2.)))]),
			Some(Block::new(vec![Stmt::Expr(Expr::Literal(
				Literal::Number(1.)
			))])),
		))]
	)
}

#[test]
fn parse_if_else_int() {
	let (tokens, errors) = tokens_from_src("if 0 == 0 { 2 } else { 1 }");
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![Stmt::Expr(Expr::If(
			Box::new(Expr::Binary(
				Box::new(Expr::Literal(Literal::Number(0.))),
				BinaryOp::Eq,
				Box::new(Expr::Literal(Literal::Number(0.))),
			)),
			Block::new(vec![Stmt::Expr(Expr::Literal(Literal::Number(2.)))]),
			Some(Block::new(vec![Stmt::Expr(Expr::Literal(
				Literal::Number(1.)
			))])),
		))]
	)
}
