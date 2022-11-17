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
