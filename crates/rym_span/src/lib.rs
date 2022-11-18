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
