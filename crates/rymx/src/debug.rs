use rym_ast::{Stmt, Token, TokenType};
use std::fmt::Display;

pub fn print_tokens(tokens: &[Token]) {
	for token in tokens {
		match &token.typ {
			TokenType::Semicolon => println!("Semicolon"),
			value => print!("{value:?} "),
		}
	}
}

pub fn print_ast(ast: &[Stmt]) {
	for stmt in ast {
		println!("{stmt:?}");
	}
}

pub fn print_errors<E>(errors: &[E])
where
	E: Display,
{
	for err in errors {
		println!("{err}");
	}
}