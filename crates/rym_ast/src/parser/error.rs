use crate::token::*;
use std::fmt::Display;

pub enum ParseError<'src> {
	TokenMismatch { token: Token<'src>, msg: String },
	InvalidAssignmentTarget { token: Token<'src> },
}

impl<'src> ParseError<'src> {
	pub(super) fn token_mismatch<T>(token: Token<'src>, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch {
			token,
			msg: msg.into(),
		})
	}
}

impl Display for ParseError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::TokenMismatch { token, msg } => {
				writeln!(f, "Error: {msg} got `{:?}`", token.value)?;
				writeln!(f, "Position: {}", token.start)
			}
			ParseError::InvalidAssignmentTarget { token } => {
				writeln!(f, "Error: Invalid assignment target `{:?}`", token.value)?;
				writeln!(f, "Position: {}", token.start)
			}
		}
	}
}
