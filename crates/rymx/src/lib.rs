use core::ops::Range;
use std::{fmt::Debug, num::TryFromIntError, path::PathBuf};

mod parse;
mod tokenize;
pub use tokenize::tokenize;

pub fn compile(diag: &mut Diagnostics, src: String) {
	let maybe_tokens = tokenize(&src);
	let stage_data = maybe_tokens
		.iter()
		.map(|maybe_token| match maybe_token {
			Ok(token) => token.debug_string(&src) + "\n",
			Err(span) => format!("Error [{span:?}]: \"{}\"\n", &src[span.as_range()]),
		})
		.collect();

	diag.debug_stage("tokenize", stage_data);
	// let Some(tokens) = maybe_tokens else {
	// 	return;
	// };
}

#[derive(Debug, Default)]
pub struct Diagnostics {
	path: Option<PathBuf>,
	stages: Vec<(&'static str, String)>,
}

impl Diagnostics {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path: Some(path),
			stages: vec![],
		}
	}

	pub fn debug_stage(&mut self, stage: &'static str, data: String) {
		self.stages.push((stage, data));
	}

	pub fn save_stages(&self) -> anyhow::Result<()> {
		let mut path = self.path.clone().unwrap_or(PathBuf::from("./unknown.rym"));
		path.set_extension("debug");
		std::fs::write(&path, self.dump_stages())?;
		Ok(())
	}

	pub fn dump_stages(&self) -> String {
		self
			.stages
			.iter()
			.map(|(stage, data)| format!(">==> {stage} >==>\n{data}<==< {stage} <==<\n"))
			.collect()
	}
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Span<Idx> {
	/// Inclusive first index
	pub start: Idx,
	/// Exclusive last index
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

impl<Idx: Debug> Debug for Span<Idx> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{:?}..{:?}", self.start, self.end))
	}
}
