use std::fmt::Display;

use ast::Token;
use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum ParseError {
	TokenMismatch(Token, String),
	InvalidAssignmentTarget(Token),
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: Token, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch(token, msg.into()))
	}
}

// TODO: Improve error messages by using Spanned properly
impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let err = "Error:".red().bold();

		match self {
			ParseError::TokenMismatch(token, msg) => {
				writeln!(f, "{err}	{msg} got `{:?}`", token.typ)?;
				writeln!(f, "Span:	{:?}", token.span)
			}
			ParseError::InvalidAssignmentTarget(token) => {
				writeln!(f, "{err}	Invalid assignment target `{:?}`", token.typ)?;
				writeln!(f, "Span:	{:?}", token.span)
			}
		}
	}
}
