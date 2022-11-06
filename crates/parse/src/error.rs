use std::fmt::Display;

use ast::{Spanned, Token};
use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum ParseError {
	TokenMismatch(Spanned<Token>, String),
	InvalidAssignmentTarget(Spanned<Token>),
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: &Spanned<Token>, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch(token.clone(), msg.into()))
	}
}

// TODO: Improve error messages by using Spanned properly
impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let err = "Error:".red().bold();

		match self {
			ParseError::TokenMismatch(Spanned(token, span), msg) => {
				writeln!(f, "{err}	{msg} got `{:?}`", token.typ)?;
				writeln!(f, "Span:	{:?}", span)
			}
			ParseError::InvalidAssignmentTarget(Spanned(token, span)) => {
				writeln!(f, "{err}	Invalid assignment target `{:?}`", token.typ)?;
				writeln!(f, "Span:	{:?}", span)
			}
		}
	}
}
