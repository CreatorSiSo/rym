use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
pub struct Cursor<'a> {
	len_remaining: usize,
	chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
	pub fn new(src: &'a str) -> Self {
		Self { len_remaining: src.len(), chars: src.chars() }
	}

	pub(crate) fn bump(&mut self) -> Option<char> {
		self.chars.next()
	}

	/// Eats chars while predicate returns true or until the end of file is reached.
	pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
		while predicate(self.peek()) && !self.is_eof() {
			self.bump();
		}
	}

	/// Peeks the next symbol from the input stream without consuming it.
	/// Returns `'\0'` if the requested position doesn't exist.
	pub(crate) fn peek(&self) -> char {
		self.chars.clone().next().unwrap_or('\0')
	}

	/// Returns amount of already consumed symbols.
	pub(crate) fn len_consumed(&self) -> u32 {
		(self.len_remaining - self.chars.as_str().len()) as u32
	}

	/// Resets the number of bytes consumed to 0.
	pub(crate) fn reset_len_consumed(&mut self) {
		self.len_remaining = self.chars.as_str().len();
	}

	/// Checks if there is nothing more to consume.
	pub(crate) fn is_eof(&self) -> bool {
		self.chars.as_str().is_empty()
	}
}
