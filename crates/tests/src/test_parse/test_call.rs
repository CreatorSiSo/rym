use super::*;

#[test]
fn empty_call() {
	let (tokens, errors) = tokens_from_src("call_me()");
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![stmt!(expr!(Call(boxed!(expr!(ident!("call_me"))), vec![])))]
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
			vec![expr!(lit!("Hello World!"))]
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
				expr!(lit!("Coords: ")),
				expr!(lit!(1.324)),
				expr!(lit!(0.43)),
				expr!(lit!(5.02))
			]
		)))]
	)
}

#[test]
fn chained_calls() {
	let (tokens, errors) = tokens_from_src(r#"returns_fn(println)("Hello World!")"#);
	assert!(errors.is_empty());
	let (ast, errors) = ast_from_src(tokens);
	assert!(errors.is_empty());
	assert_eq!(
		ast,
		vec![stmt!(expr!(Call(
			boxed!(expr!(Call(
				boxed!(expr!(ident!("returns_fn"))),
				vec![expr!(ident!("println")),]
			))),
			vec![expr!(lit!("Hello World!"))]
		)))]
	)
}
