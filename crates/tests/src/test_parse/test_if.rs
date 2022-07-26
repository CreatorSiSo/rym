use super::*;

#[test]
fn parse_if_int() {
	let (tokens, errors) = tokens_from_src("if true { 2 }");
	assert!(errors.is_empty());
	let (_ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	// FIXME
	// assert_eq!(
	// 	ast,
	// 	vec![stmt!(expr!(If(
	// 		boxed!(expr!(lit!(true);0..0)),
	// 		block![stmt!(expr!(lit!(2.0);0..0);0..0)],
	// 		None
	// 	);0..0);0..0)]
	// )
}

#[test]
fn parse_if_else_true() {
	let (tokens, errors) = tokens_from_src("if true { 2 } else { 1 }");
	assert!(errors.is_empty());
	let (_ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	// FIXME
	// assert_eq!(
	// 	ast,
	// 	vec![stmt!(expr!(If(
	// 		boxed!(expr!(lit!(true);0..0)),
	// 		block![stmt!(expr!(lit!(2.0);0..0);0..0)],
	// 		Some(block![stmt!(expr!(lit!(1.0);0..0);0..0)]),
	// 	);0..0);0..0)]
	// )
}

#[test]
fn parse_if_else_int() {
	let (tokens, errors) = tokens_from_src("if 0 == 0 { 2 } else { 1 }");
	assert!(errors.is_empty());
	let (_ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	// FIXME
	// assert_eq!(
	// 	ast,
	// 	vec![stmt!(expr!(If(
	// 		boxed!(expr!(Binary(
	// 			boxed!(expr!(lit!(0.0);0..0)),
	// 			BinaryOp::Eq,
	// 			boxed!(expr!(lit!(0.0);0..0)),
	// 		);0..0)),
	// 		block![stmt!(expr!(lit!(2.0);0..0);0..0)],
	// 		Some(block![stmt!(expr!(lit!(1.0);0..0);0..0)]),
	// 	);0..0);0..0)]
	// )
}
