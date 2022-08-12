use crate::debug::*;
use rym_ast::{Lexer, Parser};
use rym_tree_walk::Interpreter;
use std::path::Path;
use std::process::exit;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let source = std::fs::read_to_string(path)?;
	let mut had_syntax_error = false;

	println!("--- Lexer ---");
	let (tokens, errors) = Lexer::lex(&source);
	if !errors.is_empty() {
		had_syntax_error = true;
	}
	print_tokens(&tokens);
	print_errors(&errors);
	println!("\n");

	println!("--- Parser ---");
	let (ast, errors) = Parser::parse(tokens);
	if !errors.is_empty() {
		had_syntax_error = true;
	}
	print_ast(&ast);
	print_errors(&errors);
	println!("\n");

	if had_syntax_error {
		exit(65 /* data format error */)
	}

	println!("--- Interpreter ---");
	let error = Interpreter::new().eval(&ast);
	println!("{error:?}");
	println!("\n");

	Ok(())
}
