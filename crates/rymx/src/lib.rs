use std::{fmt::Debug, path::PathBuf};

mod ast;
mod parse;
mod tokenize;
pub use tokenize::tokenizer;
mod span;
pub use span::Span;
use tokenize::Token;

pub fn compile_module(diag: &mut Diagnostics, src: &str) -> Result<(), ()> {
	let tokens: Vec<(Token, Span)> = tokenize(diag, src)?;

	Ok(())
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_expr(diag: &mut Diagnostics, src: &str) -> Result<(), ()> {
	let tokens: Vec<(Token, Span)> = tokenize(diag, src)?;

	// diag.start_stage("parse");

	Ok(())
}

pub fn tokenize(diag: &mut Diagnostics, src: &str) -> Result<Vec<(Token, Span)>, ()> {
	diag.start_stage("tokenize");

	let results: Vec<_> = tokenizer(src).collect();
	diag.push_results(
		&results.iter().fold(String::new(), |accum, (result, span)| {
			let (token, span) = match result {
				Ok(token) => (format!("{token:?}"), span),
				Err(_) => ("Error".into(), span),
			};
			accum + &format!("{token} [{}]\n", span.src(src).escape_debug())
		}),
	);

	let mut tokens = vec![];
	for (result, span) in results {
		match result {
			Ok(token) => tokens.push((token, span)),
			Err(_) => {
				diag.push_message(format!(
					"Error: Invalid character <{}> at {:?}\n",
					span.src(src),
					span
				));
				return Err(());
			}
		}
	}
	Ok(tokens)
}

pub struct Diagnostics {
	results_path: Option<PathBuf>,
	output: Box<dyn std::io::Write>,
	stages: Vec<Stage>,
}

#[derive(Debug)]
struct Stage {
	name: &'static str,
	messages: Vec<String>,
	results: String,
}

impl Diagnostics {
	pub fn new(results_path: Option<PathBuf>, output: Box<dyn std::io::Write>) -> Self {
		Self {
			results_path,
			output,
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
		let string: String = message.into();
		self.output.write_all(string.as_bytes()).unwrap();
		self.stages.last_mut().unwrap().messages.push(string);
	}

	// TODO Incremantally write new results and messages to an output stream
	pub fn save_stages(&self) -> anyhow::Result<()> {
		let mut path = self
			.results_path
			.clone()
			.unwrap_or(PathBuf::from("./unknown.rym"));
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
