use std::fmt::Display;

use ast::SpannedToken;

#[derive(Debug)]
pub enum ParseError {
	TokenMismatch { token: SpannedToken, msg: String },
	InvalidAssignmentTarget { token: SpannedToken },
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: &SpannedToken, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch {
			token: token.clone(),
			msg: msg.into(),
		})
	}
}

// TODO: Improve error messages by using Spanned properly
impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::TokenMismatch { token, msg } => {
				writeln!(f, "Error:	{msg} got `{:?}`", token.0.typ)?;
				write!(f, "Pos:	{:?}", token.1)
			}
			ParseError::InvalidAssignmentTarget { token } => {
				writeln!(f, "Error:	Invalid assignment target `{:?}`", token.0.typ)?;
				write!(f, "Pos:	{:?}", token.1)
			}
		}
	}
}
