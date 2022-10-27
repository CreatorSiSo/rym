use colored::Colorize;

use crate::Lexer;
use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum LexError {
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

impl LexError {
	pub fn invalid_char<T>(lexer: &Lexer) -> Result<T, Self> {
		Err(Self::InvalidChar {
			msg: format!("Found invalid character `{}`", lexer.c),
			line: lexer.line,
			col: lexer.col,
		})
	}

	pub fn parse_int<T>(lexer: &Lexer, err: ParseIntError) -> Result<T, Self> {
		Err(Self::ParseInt {
			msg: format!("{} -> {err:?}", &lexer.src[lexer.start..lexer.current]),
			line: lexer.line,
			col: lexer.col,
		})
	}

	pub fn parse_float<T>(lexer: &Lexer, err: ParseFloatError) -> Result<T, Self> {
		Err(Self::ParseFloat {
			msg: format!("{} -> {err:?}", &lexer.src[lexer.start..lexer.current]),
			line: lexer.line,
			col: lexer.col,
		})
	}
}

impl Display for LexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let err = "Error:".red().bold();

		match self {
			LexError::InvalidChar { msg, line, col } => {
				writeln!(f, "{err} {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
			LexError::ParseInt { msg, line, col } => {
				writeln!(f, "{err} Could not parse integer {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
			LexError::ParseFloat { msg, line, col } => {
				writeln!(f, "{err} Could not parse float {msg}")?;
				writeln!(f, "       {line}:{col}")
			}
		}
	}
}
