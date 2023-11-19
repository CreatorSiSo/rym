use core::ops::Range;
use std::fmt::Debug;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Span {
	/// Inclusive first index
	pub start: usize,
	/// Exclusive last index
	pub end: usize,
}

impl Span {
	pub fn src(self, src: &str) -> &str {
		&src[std::ops::Range::<usize>::from(self)]
	}
}

impl chumsky::span::Span for Span {
	type Context = ();
	type Offset = usize;

	fn new(_context: Self::Context, range: Range<Self::Offset>) -> Self {
		range.into()
	}

	fn context(&self) -> Self::Context {
		()
	}

	fn start(&self) -> Self::Offset {
		self.start
	}

	fn end(&self) -> Self::Offset {
		self.end
	}
}

impl From<Range<usize>> for Span {
	fn from(value: Range<usize>) -> Self {
		Self {
			start: value.start,
			end: value.end,
		}
	}
}

impl From<Span> for Range<usize> {
	fn from(value: Span) -> Self {
		Self {
			start: value.start,
			end: value.end,
		}
	}
}

impl Debug for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{:?}..{:?}", self.start, self.end))
	}
}
