//! This Rust library unquotes and unescapes strings.
//!
//! # Examples
//! ```
//! use rym_unescape::unquote;
//!
//! assert_eq!(unquote("\'c\'").unwrap(), "c");
//! assert_eq!(unquote("\"Hello World!\\n\"").unwrap(), "Hello World!\n");
//! ```

#[cfg(test)]
mod tests;

use std::str::Chars;

/// Unquotes `input`.
pub fn unquote(input: &str) -> Result<String, UnquoteError> {
	if input.len() < 2 {
		return Err(UnquoteError::NotEnoughChars { need: 2 });
	}

	let quote = input.chars().next().unwrap();

	if quote != '"' && quote != '\'' {
		return Err(UnquoteError::UnrecognizedQuote);
	}

	if input.chars().last().unwrap() != quote {
		return Err(UnquoteError::Unterminated);
	}

	// removes quote characters
	// the sanity checks performed above ensure that the quotes will be ASCII and this will not
	// panic
	let str = &input[1..input.len() - 1];

	unescape(str, Some(quote))
}

/// Returns `input` after processing escapes such as `\n` and `\x00`.
pub fn unescape(input: &str, illegal: Option<char>) -> Result<String, UnquoteError> {
	let mut chars = input.chars();
	let mut unescaped = String::new();
	loop {
		let Some(char) = chars.next() else { break };
		let result_char = match char {
			_ if Some(char) == illegal => return Err(UnquoteError::IllegalChar),
			'\\' => match chars.next() {
				None => return Err(UnquoteError::Unterminated),
				Some(char) => match char {
					'\\' | '"' | '\'' | '`' => char,
					'a' => '\x07',
					'b' => '\x08',
					'f' => '\x0c',
					'n' => '\n',
					'r' => '\r',
					't' => '\t',
					'v' => '\x0b',
					// octal
					'0'..='9' => {
						let octal = char.to_string() + take(&mut chars, 2)?;
						u8::from_str_radix(&octal, 8)
							.map_err(|err| UnquoteError::UnrecognizedEscape(err.to_string()))? as char
					}
					// hex
					'x' => u8::from_str_radix(take(&mut chars, 2)?, 16)
						.map_err(|err| UnquoteError::UnrecognizedEscape(err.to_string()))? as char,
					// unicode
					'u' => decode_unicode(take(&mut chars, 4)?)?,
					'U' => decode_unicode(take(&mut chars, 8)?)?,
					_ => return Err(UnquoteError::UnrecognizedEscapePrefix(format!("\\{char}"))),
				},
			},
			_ => char,
		};
		unescaped.push(result_char);
	}

	Ok(unescaped)
}

#[inline]
fn take<'a>(chars: &mut Chars<'a>, n: usize) -> Result<&'a str, UnquoteError> {
	let prev = chars.as_str();
	for i in 0..n {
		chars.next().ok_or(UnquoteError::NotEnoughChars { need: n - i })?;
	}
	Ok(&prev[0..n])
}

fn decode_unicode(code_point: &str) -> Result<char, UnquoteError> {
	match u32::from_str_radix(code_point, 16) {
		Err(err) => Err(UnquoteError::UnrecognizedEscape(err.to_string())),
		Ok(n) => char::from_u32(n).ok_or(UnquoteError::InvalidUnicode { code_point: n }),
	}
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum UnquoteError {
	#[error("Not enough chars need <{need}> more")]
	NotEnoughChars { need: usize },
	#[error("Unrecognized quote character")]
	UnrecognizedQuote,
	#[error("Unterminated literal")]
	Unterminated,
	#[error("Illegal character")]
	IllegalChar,
	#[error("Unrecognized escape sequence prefix: <{0}>")]
	UnrecognizedEscapePrefix(String),
	#[error("Unrecognized escape sequence: <{0}>")]
	UnrecognizedEscape(String),
	#[error("Invalid unicode code point <{code_point}>")]
	InvalidUnicode { code_point: u32 },
}
