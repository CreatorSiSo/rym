use std::fmt::Display;

use ast::{SpannedToken, Stmt, TokenType};

use colored::Colorize;

// TODO Improve the print block api and add a properly colored `│` character infront of each line
pub(crate) fn block<F>(title: &str, f: F)
where
	F: Fn() -> bool,
{
	println!("\n{} {}", "╭──".bright_blue(), title.bright_blue().bold());
	println!(
		"{}",
		if f() {
			"╰── ✓".green()
		} else {
			"╰── ✗".red()
		}
	);
}

pub(crate) fn tokens(tokens: &[SpannedToken]) {
	for token in tokens {
		match &token.0.typ {
			TokenType::Semicolon => println!("Semicolon"),
			value => print!("{value:?} "),
		}
	}
	println!()
}

pub(crate) fn ast(ast: &[Stmt]) {
	for stmt in ast {
		#[cfg(feature = "expand")]
		println!("{stmt:#?}");

		#[cfg(not(feature = "expand"))]
		println!("{stmt:?}");
	}
}

pub(crate) fn errors<E>(errors: &[E])
where
	E: Display,
{
	for error in errors {
		print!("\n{error}");
	}
}
