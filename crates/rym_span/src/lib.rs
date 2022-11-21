use std::fmt::Display;
use std::ops::Range;

pub const DUMMY_SPAN: Span = Span::new(0, 0);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.start.partial_cmp(&other.start)
	}
}

impl Ord for Span {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.start.cmp(&other.start)
	}
}

impl Span {
	pub const fn new(start: usize, end: usize) -> Self {
		Self { start, end }
	}

	pub const fn is_dummy(&self) -> bool {
		self.start == 0 && self.end == 0
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

impl From<&Span> for (usize, usize) {
	fn from(span: &Span) -> (usize, usize) {
		(span.start, span.end)
	}
}

impl From<Span> for (usize, usize) {
	fn from(span: Span) -> (usize, usize) {
		(span.start, span.end)
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
