use std::fmt::Display;

use ast::Token;

#[derive(Debug)]
pub enum ParserError {
	TokenMismatch { token: Token, msg: String },
	InvalidAssignmentTarget { token: Token },
}

impl ParserError {
	pub(super) fn token_mismatch<T>(token: &Token, msg: &str) -> Result<T, Self> {
		Err(ParserError::TokenMismatch {
			token: token.clone(),
			msg: msg.into(),
		})
	}
}

impl Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParserError::TokenMismatch { token, msg } => {
				writeln!(f, "Error:	{msg} got `{:?}`", token.typ)?;
				write!(f, "Pos:	{}", token.start)
			}
			ParserError::InvalidAssignmentTarget { token } => {
				writeln!(f, "Error:	Invalid assignment target `{:?}`", token.typ)?;
				write!(f, "Pos:	{}", token.start)
			}
		}
	}
}
