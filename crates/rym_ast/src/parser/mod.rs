use crate::Lexer;

mod error;
use error::ParserError;

struct Parser<'src> {
	tokens: Lexer<'src>,
}

impl<'src> Parser<'src> {
	pub fn new(tokens: Lexer<'src>) -> Self {
		Self { tokens }
	}

	pub fn parse() -> Result<(), ParserError<'src>> {
		todo!()
	}
}
