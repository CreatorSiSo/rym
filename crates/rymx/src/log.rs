use std::fmt::Display;

use rym_ast::{Stmt, Token, TokenType};

use colored::Colorize;

pub(crate) fn title(title: &str, success: bool) {
	if success {
		print!("\n{0} {1} {0}\n", "---".green(), title.green().bold())
	} else {
		print!("\n{0} {1} {0}\n", "---".red(), title.red().bold())
	};
}

pub(crate) fn tokens(tokens: &[Token]) {
	for token in tokens {
		match &token.typ {
			TokenType::Semicolon => println!("Semicolon"),
			value => print!("{value:?} "),
		}
	}
	print!("\n")
}

pub(crate) fn ast(ast: &[Stmt]) {
	for stmt in ast {
		println!("{stmt:?}");
	}
}

pub(crate) fn errors<E>(errors: &[E])
where
	E: Display,
{
	for err in errors {
		println!("{err}");
	}
}
