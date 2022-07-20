#[cfg(test)]
use crate::{Lexer, TokenValue};

#[test]
fn test() {
	let source = include_str!("../../../../examples/lexer.rym");
	let mut lexer = Lexer::new(source);
	loop {
		match lexer.token() {
			Ok(token) => {
				println!("{token:?}");
				if token.value == TokenValue::Eof {
					break;
				}
			}
			Err(err) => println!("{err:?}"),
		}
	}
}
