use super::*;

#[test]
fn lex_empty_file() {
	let tokens: Vec<LexResult<SpannedToken>> = Lexer::new("").collect();
	assert_eq!(tokens, vec![])
}

#[test]
fn parse_empty_file() {
	let ast: Vec<ParseResult<Spanned<Stmt>>> = parse::Parser::new(vec![]).collect();
	assert_eq!(ast, vec![])
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
	Parser::new(tokens).for_each(|result| ast.push(result.unwrap()));
	let result = Interpreter::default().eval(&ast);
	assert_eq!(result, Ok(()))
}
