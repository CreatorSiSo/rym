use std::path::Path;
use std::process::exit;

use crate::log;
use rym_ast::Parser;
use rym_tree_walk::Interpreter;
use tokenize::Lexer;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let source = std::fs::read_to_string(path)?;

	let (tokens, errors) = Lexer::lex(&source);
	let syntax_correct = errors.is_empty();
	log::title("Lexer", syntax_correct);
	log::tokens(&tokens);
	log::errors(&errors);

	let (ast, errors) = Parser::parse(tokens);
	let syntax_correct = errors.is_empty();
	log::title("Parser", syntax_correct);
	log::ast(&ast);
	log::errors(&errors);

	if !syntax_correct {
		exit(65 /* data format error */)
	}

	log::title("Interpreter", true);
	if let Err(error) = Interpreter::new().eval(&ast) {
		println!("{error:?}");
		exit(1)
	}

	Ok(())
}
