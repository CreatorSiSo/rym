use super::*;

#[test]
fn module() {
	assert_ast_errs(
		"module test {}",
		&[item_module(("test", 7..11), vec![], delim_span(12..13, 13..14, 12..14))],
		&[],
	);

	assert_ast_errs(
		"module test {}",
		&[item_module(("test", 7..11), vec![], delim_span(12..13, 13..14, 12..14))],
		&[],
	);
}

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
				fn_param(("testing", 16..23), Some(("Some", 25..29))),
				fn_param(("other_stuff", 31..42), Some(("Other", 44..49))),
			],
			None,
			block(vec![], delim_span(51..52, 52..53, 51..53)),
		)],
		&[],
	);
}

#[test]
fn function_rest_param() {
	assert_ast_errs(
		"fn print(..values) {}",
		&[fn_item(
			("print", 3..8),
			vec![fn_rest_param(true, ("values", 11..17), None)],
			None,
			block(vec![], delim_span(19..20, 20..21, 19..21)),
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
			Some(("Type", 14..18)),
			block(vec![], delim_span(19..20, 21..22, 19..22)),
		)],
		&[],
	);

	assert_ast_errs(
		"fn self_param(self) -> Type_123 {\n}",
		&[fn_item(
			("self_param", 3..13),
			vec![fn_param(("self", 14..18), None)],
			Some(("Type_123", 23..31)),
			block(vec![], delim_span(32..33, 34..35, 32..35)),
		)],
		&[],
	);
}
