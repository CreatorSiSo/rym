use crate::debug::*;
use rym_ast::{Lexer, Parser};
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
	for result in Parser::new(tokens) {
		match result {
			Ok(stmt) => print_ast(&[stmt]),
			Err(err) => print_errors(&[err]),
		}
	}
	if !errors.is_empty() {
		had_syntax_error = true;
	}
	// print_ast(&ast);
	// print_errors(&errors);
	println!("\n");

	if had_syntax_error {
		exit(65 /* data format error */)
	}

	Ok(())
}
