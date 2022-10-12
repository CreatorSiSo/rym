use std::fmt::Display;

use ast::Token;

#[derive(Debug)]
pub enum ParseError {
	TokenMismatch { token: Token, msg: String },
	InvalidAssignmentTarget { token: Token },
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: &Token, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch {
			token: token.clone(),
			msg: msg.into(),
		})
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::TokenMismatch { token, msg } => {
				writeln!(f, "Error:	{msg} got `{:?}`", token.typ)?;
				write!(f, "Pos:	{}", token.start)
			}
			ParseError::InvalidAssignmentTarget { token } => {
				writeln!(f, "Error:	Invalid assignment target `{:?}`", token.typ)?;
				write!(f, "Pos:	{}", token.start)
			}
		}
	}
}
