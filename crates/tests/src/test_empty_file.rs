use super::*;

#[test]
fn lex_empty_file() {
	let tokens: Vec<LexResult<Spanned<Token>>> = Lexer::new("").collect();
	assert_eq!(tokens, vec![]);
}

#[test]
fn parse_empty_file() {
	let ast: Vec<ParseResult<Spanned<Stmt>>> = Parser::new(vec![]).into_iter().collect();
	assert_eq!(ast, vec![]);
}

#[test]
fn interpret_empty_ast() {
	let result = Interpreter::default().eval(&[]);
	assert_eq!(result, Ok(()));
}

#[test]
fn integration_empty_file() {
	let mut tokens = Vec::new();
	Lexer::new("").for_each(|result| tokens.push(result.unwrap()));
	let mut ast = Vec::new();
	for result in Parser::new(tokens) {
		ast.push(result.unwrap())
	}
	let result = Interpreter::default().eval(&ast);
	assert_eq!(result, Ok(()));

	let mut tokens = Vec::new();
	Lexer::new("\n").for_each(|result| tokens.push(result.unwrap()));
	let mut ast = Vec::new();
	for result in Parser::new(tokens) {
		ast.push(result.unwrap())
	}
	let result = Interpreter::default().eval(&ast);
	assert_eq!(result, Ok(()))
}
