use super::*;
mod function;

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
