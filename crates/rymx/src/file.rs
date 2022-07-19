use rym_ast::Lexer;
use std::path::Path;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let source = std::fs::read_to_string(path)?;
	let lexer = Lexer::new(&source);

	println!("--- Lexer ---");
	for maybe_token in lexer {
		match maybe_token {
			Ok(token) => print!("{:?} ", token),
			Err(err) => print!("{:?} ", err),
		}
	}
	println!("\n");

	Ok(())
}
