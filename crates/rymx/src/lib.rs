use core::ops::Range;
use std::{fmt::Debug, num::TryFromIntError, path::PathBuf};

mod ast;
mod parse;
mod tokenize;
pub use tokenize::tokenize;
use tokenize::Token;

pub fn compile(diag: &mut Diagnostics, src: &str) {
	diag.start_stage("tokenize");

	let results = tokenize(src);
	diag.push_results(&results.iter().fold(String::new(), |accum, result| {
		let (kind, span) = match result {
			Ok(Token { kind, span }) => (format!("{kind:?}"), span),
			Err(span) => ("Error".into(), span),
		};
		accum + &format!("{kind} [{}]\n", span.src(src).escape_debug())
	}));

	// let Some(tokens) = maybe_tokens else {
	// 	return;
	// };
}

#[derive(Debug, Default)]
pub struct Diagnostics {
	path: Option<PathBuf>,
	stages: Vec<Stage>,
}

#[derive(Debug)]
struct Stage {
	name: &'static str,
	messages: Vec<String>,
	results: String,
}

impl Diagnostics {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path: Some(path),
			stages: vec![],
		}
	}

	/// All results and messages (debug, warning, error) pushed after this
	/// will be associated with this stage until another stage one is started
	pub fn start_stage(&mut self, name: &'static str) {
		self.stages.push(Stage {
			name,
			messages: vec![],
			results: String::new(),
		});
	}

	/// Associates some result data for debugging with the current stage
	/// (for example rendered tokenizer output, or an ast in text form)
	pub fn push_results(&mut self, data: &str) {
		self.stages.last_mut().unwrap().results.push_str(data);
	}

	/// Associates a message (debug, warning, error) with the current stage
	pub fn push_message(&mut self, message: impl Into<String>) {
		self
			.stages
			.last_mut()
			.unwrap()
			.messages
			.push(message.into());
	}

	// TODO Incremantally write new results and messages to an output stream
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
			.fold(String::new(), |dump, Stage { name, results, .. }| {
				dump + &format!(">==> {name} >==>\n{results}<==< {name} <==<\n")
			})
	}
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Span<Idx> {
	/// Inclusive first index
	pub start: Idx,
	/// Exclusive last index
	pub end: Idx,
}

impl Span<u32> {
	pub fn src(self, src: &str) -> &str {
		&src[self.as_range()]
	}
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
