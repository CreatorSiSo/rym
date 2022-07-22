#[cfg(test)]
use crate::{Lexer, TokenType};

#[test]
fn test() {
	let source = include_str!("../../../../examples/lexer.rym");
	let mut lexer = Lexer::new(source);
	loop {
		match lexer.next_token() {
			Ok(token) => {
				println!("{token:?}");
				if token.typ == TokenType::Eof {
					break;
				}
			}
			Err(err) => println!("{err:?}"),
		}
	}
}
