use std::fmt::Display;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl Span {
	pub const fn new(start: usize, end: usize) -> Self {
		Self { start, end }
	}

	pub const fn move_by(self, amount: i32) -> Self {
		const fn add(lhs: usize, rhs: i32) -> usize {
			if rhs.is_negative() {
				lhs - rhs.wrapping_abs() as u32 as usize
			} else {
				lhs + rhs as usize
			}
		}
		Self { start: add(self.start, amount), end: add(self.end, amount) }
	}
}

impl Into<(usize, usize)> for Span {
	fn into(self) -> (usize, usize) {
		(self.start, self.end)
	}
}

impl From<&Span> for Range<usize> {
	fn from(span: &Span) -> Range<usize> {
		Range { start: span.start, end: span.end }
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.start.to_string())?;
		f.write_str("..")?;
		f.write_str(&self.end.to_string())
	}
}
