use super::*;

#[test]
fn function() {
	assert_ast_errs(
		"fn empty() {}",
		&[fn_item(("empty", 3..8), vec![], None, block(vec![], delim_span(11..12, 12..13, 11..13)))],
		&[],
	);

	assert_ast_errs(
		"fn self_param(self) {\n}",
		&[fn_item(
			("self_param", 3..13),
			vec![fn_param(("self", 14..18), None)],
			None,
			block(vec![], delim_span(20..21, 22..23, 20..23)),
		)],
		&[],
	);

	assert_ast_errs(
		"fn params(self, testing: Some, other_stuff: Other) {}",
		&[fn_item(
			("params", 3..9),
			vec![
				fn_param(("self", 10..14), None),
				fn_param(("testing", 16..23), Some(path!(Some, 25..29))),
				fn_param(("other_stuff", 31..42), Some(path!(Other, 44..49))),
			],
			None,
			block(vec![], delim_span(51..52, 52..53, 51..53)),
		)],
		&[],
	);
}

#[test]
fn function_param_default() {
	assert_ast_errs(
		"fn default(self, testing = 0, other_stuff: Other) {}",
		&[fn_item(
			("default", 3..10),
			vec![
				fn_param(("self", 11..15), None),
				fn_param_default(("testing", 17..24), None, Some(expr_lit(0, 27..28))),
				fn_param(("other_stuff", 30..41), Some(path!(Other, 43..48))),
			],
			None,
			block(vec![], delim_span(50..51, 51..52, 50..52)),
		)],
		&[],
	);
}

#[test]
fn function_rest_param() {
	assert_ast_errs(
		r#"fn funny(..values) {}"#,
		&[fn_item(
			("funny", 3..8),
			vec![fn_rest_param(true, ("values", 11..17), None)],
			None,
			block(vec![], delim_span(19..20, 20..21, 19..21)),
		)],
		&[],
	);

	assert_ast_errs(
		// TODO: Use "fn print(..values: [Display]) {}", once arrays types have been added
		"fn print(..values: Display) {}",
		&[fn_item(
			("print", 3..8),
			vec![fn_rest_param(true, ("values", 11..17), Some(path!(Display, 19..26)))],
			None,
			block(vec![], delim_span(28..29, 29..30, 28..30)),
		)],
		&[],
	);

	// TODO: Add more tests for error recovery when parsing function params
	assert_ast_errs(
		r#"fn funny(..values = &%) {}"#,
		&[fn_item(
			("funny", 3..8),
			vec![fn_rest_param(true, ("values", 11..17), None)],
			None,
			block(vec![], delim_span(24..25, 25..26, 24..26)),
		)],
		&[Diagnostic::new_spanned(Level::Error, "Expected `AnyLiteral` got `And`", Span::new(20, 21))],
	);

	assert_ast_errs(
		// TODO: Use r#"fn funny(..values = [12]) {}"#, once arrays have been added
		r#"fn funny(..values = 12) {}"#,
		&[fn_item(
			("funny", 3..8),
			vec![fn_rest_param_default(true, ("values", 11..17), None, Some(expr_lit(12, 20..22)))],
			None,
			block(vec![], delim_span(24..25, 25..26, 24..26)),
		)],
		&[],
	);
}

#[test]
fn function_return_type() {
	assert_ast_errs(
		"fn empty() -> Type {\n}",
		&[fn_item(
			("empty", 3..8),
			vec![],
			Some(path!(Type, 14..18)),
			block(vec![], delim_span(19..20, 21..22, 19..22)),
		)],
		&[],
	);

	assert_ast_errs(
		"fn self_param(self) -> Type_123 {\n}",
		&[fn_item(
			("self_param", 3..13),
			vec![fn_param(("self", 14..18), None)],
			Some(path!(Type_123, 23..31)),
			block(vec![], delim_span(32..33, 34..35, 32..35)),
		)],
		&[],
	);
}
