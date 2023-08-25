use core::ops::Range;
use std::{fmt::Debug, num::TryFromIntError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span<Idx> {
	/// Inclusive starting index
	pub start: Idx,
	/// Exclusive ending index
	pub end: Idx,
}

impl<Idx: TryInto<usize>> Span<Idx> {
	pub fn as_range(self) -> Range<usize>
	where
		<Idx as TryInto<usize>>::Error: Debug,
	{
		Range {
			start: self.start.try_into().unwrap(),
			end: self.end.try_into().unwrap(),
		}
	}
}

impl TryFrom<Range<usize>> for Span<u32> {
	type Error = TryFromIntError;

	fn try_from(Range { start, end }: Range<usize>) -> Result<Self, Self::Error> {
		Ok(Self {
			start: start.try_into()?,
			end: end.try_into()?,
		})
	}
}
