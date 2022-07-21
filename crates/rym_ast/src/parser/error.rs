use crate::token::*;
use std::fmt::Display;

pub enum ParserError<'src> {
	TokenMismatch { token: Token<'src>, msg: String },
	InvalidAssignmentTarget { token: Token<'src> },
}

impl<'src> ParserError<'src> {
	pub(super) fn token_mismatch<T>(token: Token<'src>, msg: &str) -> Result<T, Self> {
		Err(ParserError::TokenMismatch {
			token,
			msg: msg.into(),
		})
	}
}

impl Display for ParserError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParserError::TokenMismatch { token, msg } => {
				writeln!(f, "Error: {msg} got `{:?}`", token.value)?;
				writeln!(f, "Position: {}", token.start)
			}
			ParserError::InvalidAssignmentTarget { token } => {
				writeln!(f, "Error: Invalid assignment target `{:?}`", token.value)?;
				writeln!(f, "Position: {}", token.start)
			}
		}
	}
}
