use crate::Lexer;
use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
	InvalidChar {
		msg: String,
		line: usize,
		col: usize,
	},
	ParseInt {
		msg: String,
		line: usize,
		col: usize,
	},
	ParseFloat {
		msg: String,
		line: usize,
		col: usize,
	},
}

impl LexerError {
	pub fn invalid_char<T>(lexer: &Lexer) -> Result<T, Self> {
		Err(Self::InvalidChar {
			msg: format!("Found invalid character `{}`", lexer.c),
			line: lexer.line,
			col: lexer.col,
		})
	}

	pub fn parse_int<T>(lexer: &Lexer, err: ParseIntError) -> Result<T, Self> {
		Err(Self::ParseInt {
			msg: format!("{} -> {err:?}", &lexer.source[lexer.start..lexer.current]),
			line: lexer.line,
			col: lexer.col,
		})
	}

	pub fn parse_float<T>(lexer: &Lexer, err: ParseFloatError) -> Result<T, Self> {
		Err(Self::ParseFloat {
			msg: format!("{} -> {err:?}", &lexer.source[lexer.start..lexer.current]),
			line: lexer.line,
			col: lexer.col,
		})
	}
}

impl Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LexerError::InvalidChar { msg, line, col } => {
				writeln!(f, "Error: {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
			LexerError::ParseInt { msg, line, col } => {
				writeln!(f, "Error: Could not parse integer {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
			LexerError::ParseFloat { msg, line, col } => {
				writeln!(f, "Error: Could not parse float {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
		}
	}
}
