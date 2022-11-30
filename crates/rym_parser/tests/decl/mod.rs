use super::*;

#[test]
fn variable_decl() {
	assert_ast_errs(
		"const	test1 = \n mut test2 = ;\n",
		&[const_decl("test1", 6..11), mut_decl("test2", 20..25)],
		&[],
	);
	assert_ast_errs(
		indoc!(
			r#"const
			test1	=
			mut	test2=;
			"#
		),
		&[const_decl("test1", 6..11), mut_decl("test2", 18..23)],
		&[],
	);
}

#[test]
fn function_decl() {
	assert_ast_errs(
		"fn empty() {}",
		&[fn_decl(("empty", 3..8), vec![], None, block(vec![], 11..13))],
		&[],
	);
	assert_ast_errs(
		"fn empty() -> Type {\n}",
		&[fn_decl(("empty", 3..8), vec![], Some(("Type", 14..18)), block(vec![], 19..22))],
		&[],
	);
	assert_ast_errs(
		"fn self_param(self) {\n}",
		&[fn_decl(("self_param", 3..13), vec![("self", 14..18)], None, block(vec![], 20..23))],
		&[],
	);
	assert_ast_errs(
		"fn self_param(self) -> Type_123 {\n}",
		&[fn_decl(
			("self_param", 3..13),
			vec![("self", 14..18)],
			Some(("Type_123", 23..31)),
			block(vec![], 32..35),
		)],
		&[],
	);
}
