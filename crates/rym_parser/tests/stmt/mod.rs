// #[test]
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
