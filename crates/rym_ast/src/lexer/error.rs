use std::num::{ParseFloatError, ParseIntError};

use crate::Lexer;

#[derive(Debug)]
pub enum LexerError {
	UnexpectedChar {
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
	pub fn unexpected_char<T>(lexer: &Lexer) -> Result<T, Self> {
		Err(Self::UnexpectedChar {
			msg: format!("Unexpected character `{}`", lexer.c),
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
