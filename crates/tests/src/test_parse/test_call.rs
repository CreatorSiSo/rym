use super::*;

#[test]
fn empty_call() {
	let (tokens, errors) = tokens_from_src("call_me()");
	assert!(errors.is_empty());
	let (ast, errors) = spanned_ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![Spanned(
			stmt!(expr!(Call(boxed!(expr!(ident!("call_me"))), vec![]))),
			0..8
		)]
	)
}

#[test]
fn one_arg_call() {
	let (tokens, errors) = tokens_from_src(r#"println("Hello World!")"#);
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![stmt!(expr!(Call(
			boxed!(expr!(ident!("println"))),
			vec![Spanned(expr!(lit!("Hello World!")), 0..0)]
		)))]
	)
}

#[test]
fn many_args_call() {
	let (tokens, errors) = tokens_from_src(r#"println("Coords: ", 1.324, 0.43, 5.02)"#);
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![stmt!(expr!(Call(
			boxed!(expr!(ident!("println"))),
			vec![
				Spanned(expr!(lit!("Coords: ")), 0..0),
				Spanned(expr!(lit!(1.324)), 0..0),
				Spanned(expr!(lit!(0.43)), 0..0),
				Spanned(expr!(lit!(5.02)), 0..0)
			]
		)))]
	)
}

#[test]
fn chained_calls() {
	let (tokens, errors) = tokens_from_src(r#"returns_fn(println)("Hello World!")()()"#);
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![stmt!(expr!(Call(
			boxed!(expr!(Call(
				boxed!(expr!(Call(
					boxed!(expr!(Call(
						boxed!(expr!(ident!("returns_fn"))),
						vec![Spanned(expr!(ident!("println")), 0..0)]
					))),
					vec![Spanned(expr!(lit!("Hello World!")), 0..0)]
				))),
				vec![]
			))),
			vec![]
		)))]
	)
}
