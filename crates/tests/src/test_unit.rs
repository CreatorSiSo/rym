use super::*;

#[test]
fn integration_unit() {
	let src = r#"
	const unit = if false {
		2
	}
	println(unit)
	// TODO: This should not even start to run (and wont once types are implemented) because it is always ()
	unit + 2
	"#;
	let mut tokens = Vec::new();
	Lexer::new(src).for_each(|result| tokens.push(result.unwrap()));
	let mut ast = Vec::new();
	Parser::new(tokens).for_each(|result| ast.push(result.unwrap()));
	let result = Interpreter::default().eval(&ast);
	assert_eq!(result, Err(TypeError::Add(Type::Unit, Type::Number).into()));
}
