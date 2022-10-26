use std::path::Path;
use std::process::exit;

use crate::log;
use lex::Lexer;
use parse::Parser;
use tree_walk::Interpreter;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let source = std::fs::read_to_string(path)?;

	let (tokens, errors) = Lexer::lex(&source);
	let lex_success = errors.is_empty();
	log::block("Lexer", || {
		log::tokens(&tokens);
		log::errors(&errors);
		lex_success
	});

	let (ast, errors) = Parser::parse(tokens);
	let parse_success = errors.is_empty();
	log::block("Parser", || {
		log::ast(&ast);
		log::errors(&errors);
		parse_success
	});

	if !lex_success | !parse_success {
		exit(65 /* Data format error */)
	}

	log::block("Interpreter", || {
		if let Err(error) = Interpreter::default().eval(&ast) {
			println!("{error:?}");
			exit(1 /* Failure */)
		} else {
			true
		}
	});

	Ok(())
}
