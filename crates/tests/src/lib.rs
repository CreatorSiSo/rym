#![cfg(test)]

use ast::*;
use lex::*;
use parse::*;
use tree_walk::*;

fn tokens_from_src(src: &str) -> (Vec<Spanned<Token>>, Vec<LexError>) {
	let mut results = (Vec::new(), Vec::new());
	for result in Lexer::new(src) {
		match result {
			Ok(tok) => results.0.push(tok),
			Err(err) => results.1.push(err),
		}
	}
	results
}

fn ast_from_src(tokens: Vec<Spanned<Token>>) -> (Vec<Stmt>, Vec<ParseError>) {
	let mut results = (Vec::new(), Vec::new());
	for result in Parser::new(tokens) {
		match result {
			Ok(Spanned(stmt, ..)) => results.0.push(stmt),
			Err(err) => results.1.push(err),
		}
	}
	results
}

fn spanned_ast_from_src(tokens: Vec<Spanned<Token>>) -> (Vec<Spanned<Stmt>>, Vec<ParseError>) {
	let mut results = (Vec::new(), Vec::new());
	for result in Parser::new(tokens) {
		match result {
			Ok(stmt) => results.0.push(stmt),
			Err(err) => results.1.push(err),
		}
	}
	results
}

macro_rules! boxed {
	($inner:expr) => {
		Box::new($inner)
	};
}

macro_rules! stmt {
	() => {
		Stmt::Empty
	};
	($inner:expr) => {
		Stmt::from($inner)
	};
}

macro_rules! expr {
	($variant:ident ($($inner:expr),+ $(,)?)) => {
		Expr::$variant($($inner),+)
	};
	($inner:expr) => {
		Expr::from($inner)
	};
}

macro_rules! lit {
	($inner:expr) => {
		Literal::from($inner)
	};
}

macro_rules! ident {
	($inner:expr) => {
		Identifier::from($inner)
	};
}

macro_rules! block {
	() => {
		Block::new(vec![])
	};
	($($stmts:expr),+ $(,)?) => {
		Block::new(vec![$($stmts),+])
	};
}

mod test_empty_file;
mod test_lex;
mod test_parse;
mod test_unit;
