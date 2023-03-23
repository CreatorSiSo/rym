use std::fmt::Display;

use colored::Colorize;
use stringx::Join;

use rym_ast::{Spanned, Stmt};
use rym_lexer::rich::Token;

// TODO Improve the print block api and add a properly colored `│` character infront of each line
pub(crate) fn block<F>(title: &str, mut f: F)
where
	F: FnMut() -> bool,
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

pub(crate) fn tokens(tokens: &[Token]) {
	println!(
		"{}",
		tokens
			.iter()
			.join_format(" ", |token| { token.to_string() })
	)
}

pub(crate) fn ast(ast: &[Spanned<Stmt>]) {
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
