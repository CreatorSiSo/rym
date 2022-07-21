use rym_ast::{Lexer, TokenValue};
use std::path::Path;
use std::process::exit;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let source = std::fs::read_to_string(path)?;
	let lexer = Lexer::new(&source);
	let mut had_lexer_error = false;

	println!("--- Lexer ---");
	for maybe_token in lexer {
		match maybe_token {
			Err(err) => {
				println!("\n{err}");
				had_lexer_error = true;
			}
			Ok(token) => {
				print!("{:?} ", token.value);
				if token.value == TokenValue::Semicolon {
					print!("\n")
				}
			}
		}
	}
	println!("\n");

	if had_lexer_error {
		exit(65 /* data format error */)
	}

	Ok(())
}
