use super::*;
use rym_parser::parse_single_expr;
use BinaryOp::*;

fn assert_expr_errs(src: &str, expect: RymResult<Expr>, errs: &[Diagnostic]) {
	let handler = Handler::default();
	let got_expr = parse_single_expr(src, &handler);

	let got_errs = handler.collect();
	println!("{:?}", got_errs);
	assert_eq!(&got_errs, errs);

	assert_eq!(got_expr, expect);
}

// #[test]
fn simple_addition() {
	assert_expr_errs(
		"src + 2",
		Ok(expr_binary(Add, expr_ident("src", 0..3), expr_lit(2, 6..7), 0..7)),
		&[],
	);
}

// #[test]
fn tests() {
	assert_expr_errs("1", Ok(expr_lit(1, 0..1)), &[]);

	assert_expr_errs(
		"1 + 2 * 3",
		Ok(expr_binary(
			Add,
			expr_lit(1, 0..1),
			expr_binary(Mul, expr_lit(2, 4..5), expr_lit(3, 8..9), 4..9),
			0..9,
		)),
		&[],
	);

	// let s = expr("a + b * c * d + e");
	// assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

	// let s = expr("f . g . h");
	// assert_eq!(s.to_string(), "(. f (. g h))");

	// let s = expr(" 1 + 2 + f . g . h * 3 * 4");
	// assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");

	// let s = expr("--1 * 2");
	// assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

	// let s = expr("--f . g");
	// assert_eq!(s.to_string(), "(- (- (. f g)))");

	// let s = expr("-9!");
	// assert_eq!(s.to_string(), "(- (! 9))");

	// let s = expr("f . g !");
	// assert_eq!(s.to_string(), "(! (. f g))");

	// let s = expr("(((0)))");
	// assert_eq!(s.to_string(), "0");

	// let s = expr("(1 + 2) * 3");
	// assert_eq!(s.to_string(), "(* (+ 1 2) 3)");

	// let s = expr("1 + (2 * 3)");
	// assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}
