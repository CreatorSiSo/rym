#![cfg(test)]

use ast::*;
use lex::*;
use parse::*;
use tree_walk::*;

fn tokens_from_src(src: &str) -> (Vec<SpannedToken>, Vec<LexError>) {
	let mut results = (Vec::new(), Vec::new());
	for result in Lexer::new(src) {
		match result {
			Ok(tok) => results.0.push(tok),
			Err(err) => results.1.push(err),
		}
	}
	results
}

fn ast_from_src(tokens: Vec<SpannedToken>) -> (Vec<Stmt>, Vec<ParseError>) {
	let mut results = (Vec::new(), Vec::new());
	for result in Parser::new(tokens) {
		match result {
			Ok(tok) => results.0.push(tok),
			Err(err) => results.1.push(err),
		}
	}
	results
}

mod test_empty_file;
mod test_lex;
mod test_parse;
