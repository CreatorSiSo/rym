//! This Rust library unquotes and unescapes strings.
//!
//! # Examples
//! ```
//!
//! ```

mod error;
#[cfg(test)]
mod tests;

pub use error::Error;
use smol_str::SmolStr;

/// Unquotes `s`.
pub fn unquote(s: &str) -> Result<SmolStr, Error> {
	if s.chars().count() < 2 {
		return Err(Error::NotEnoughChars);
	}

	let quote = s.chars().next().unwrap();

	if quote != '"' && quote != '\'' && quote != '`' {
		return Err(Error::UnrecognizedQuote);
	}

	if s.chars().last().unwrap() != quote {
		return Err(Error::UnexpectedEOF);
	}

	// removes quote characters
	// the sanity checks performed above ensure that the quotes will be ASCII and this will not
	// panic
	let s = &s[1..s.len() - 1];

	unescape(s, Some(quote))
}

/// Returns `s` after processing escapes such as `\n` and `\x00`.
pub fn unescape(s: &str, illegal: Option<char>) -> Result<SmolStr, Error> {
	let mut chars = s.chars();
	let mut unescaped = String::new();
	loop {
		match chars.next() {
			None => break,
			Some(c) => unescaped.push(match c {
				_ if Some(c) == illegal => return Err(Error::IllegalChar),
				'\\' => match chars.next() {
					None => return Err(Error::UnexpectedEOF),
					Some(c) => match c {
						'\\' | '"' | '\'' | '`' => c,
						'a' => '\x07',
						'b' => '\x08',
						'f' => '\x0c',
						'n' => '\n',
						'r' => '\r',
						't' => '\t',
						'v' => '\x0b',
						// octal
						'0'..='9' => {
							let octal = c.to_string() + &take(&mut chars, 2);
							u8::from_str_radix(&octal, 8).map_err(|_| Error::UnrecognizedEscape)? as char
						}
						// hex
						'x' => {
							let hex = take(&mut chars, 2);
							u8::from_str_radix(&hex, 16).map_err(|_| Error::UnrecognizedEscape)? as char
						}
						// unicode
						'u' => decode_unicode(&take(&mut chars, 4))?,
						'U' => decode_unicode(&take(&mut chars, 8))?,
						_ => return Err(Error::UnrecognizedEscape),
					},
				},
				_ => c,
			}),
		}
	}

	Ok(SmolStr::new(unescaped))
}

#[inline]
// Iterator#take cannot be used because it consumes the iterator
fn take<I: Iterator<Item = char>>(iterator: &mut I, n: usize) -> String {
	let mut s = String::with_capacity(n);
	for _ in 0..n {
		s.push(iterator.next().unwrap_or_default());
	}
	s
}

fn decode_unicode(code_point: &str) -> Result<char, Error> {
	match u32::from_str_radix(code_point, 16) {
		Err(_) => Err(Error::UnrecognizedEscape),
		Ok(n) => std::char::from_u32(n).ok_or(Error::InvalidUnicode),
	}
}
