use std::fmt::Debug;

mod ast;
mod parse;
mod tokenize;
use ast::{Expr, Module};
pub use tokenize::tokenizer;
mod span;
pub use span::Span;
use tokenize::Token;

pub fn compile_module(diag: &mut Diagnostics, src: &str) -> Result<Module, ()> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src)?;

	diag.start_stage("parse");
	let module = match parse::parse_module(&tokens, src) {
		Ok(module) => module,
		Err(errs) => {
			for err in errs {
				diag.push_report(format!("{err:?\n}"));
			}
			return Err(());
		}
	};
	diag.push_result(&format!("{module:#?}\n"));

	// TODO Name resolution
	// TODO Typechecking
	// TODO Const evaluation
	// TODO Generate intermediate representation

	Ok(module)
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_expr(diag: &mut Diagnostics, src: &str) -> Result<Expr, ()> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src)?;

	diag.start_stage("parse");
	let expr = match parse::parse_expr(&tokens, src) {
		Ok(expr) => expr,
		Err(errs) => {
			for err in errs {
				diag.push_report(format!("{err:?}\n"));
			}
			return Err(());
		}
	};
	diag.push_result(&format!("{expr:#?}\n"));

	Ok(expr)
}

fn tokenize(diag: &mut Diagnostics, src: &str) -> Result<Vec<(Token, Span)>, ()> {
	let results: Vec<_> = tokenizer(src).collect();
	diag.push_result(
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
		let Ok(token) = result else {
			error(
				diag,
				format!("Invalid character [{}] at {:?}", span.src(src), span),
			);
			return Err(());
		};
		match token {
			Token::DocComment | Token::Comment | Token::VSpace | Token::HSpace => continue,
			_ => tokens.push((token, span)),
		}
	}
	Ok(tokens)
}

pub fn error(diag: &mut Diagnostics, message: String) {
	diag.push_report(format!("Error: {message}\n"));
}

pub struct Diagnostics {
	logs_out: Box<dyn std::io::Write>,
	runtime_out: Box<dyn std::io::Write>,
	stages: Vec<Stage>,
}

#[derive(Debug)]
struct Stage {
	name: &'static str,
	reports: Vec<String>,
	output: String,
}

impl Diagnostics {
	pub fn new(logs_out: Box<dyn std::io::Write>, runtime_out: Box<dyn std::io::Write>) -> Self {
		Self {
			logs_out,
			runtime_out,
			stages: vec![],
		}
	}

	/// All results and messages (debug, warning, error) pushed after this
	/// will be associated with this stage until another stage one is started
	pub fn start_stage(&mut self, name: &'static str) {
		self.stages.push(Stage {
			name,
			reports: vec![],
			output: String::new(),
		});
	}

	/// Associates some result data for debugging with the current stage
	/// (for example rendered tokenizer output, or an ast in text form)
	pub fn push_result(&mut self, data: &str) {
		self.stages.last_mut().unwrap().output.push_str(data);
	}

	/// Associates a message (debug, warning, error) with the current stage
	pub fn push_report(&mut self, message: impl Into<String>) {
		let string: String = message.into();
		write!(self.runtime_out, "{string}").unwrap();
		self.runtime_out.flush().unwrap();

		self.stages.last_mut().unwrap().reports.push(string);
	}

	pub fn save_outputs(&mut self) -> anyhow::Result<()> {
		self.logs_out.write_all(self.outputs_dump().as_bytes())?;
		self.logs_out.flush()?;
		Ok(())
	}

	pub fn outputs_dump(&self) -> String {
		self
			.stages
			.iter()
			.fold(String::new(), |accum, Stage { name, output, .. }| {
				accum + &format!("--- {name} ---\n{output}\n")
			})
	}

	pub fn save_reports(&mut self) -> anyhow::Result<()> {
		self.logs_out.write_all(self.reports_dump().as_bytes())?;
		self.logs_out.flush()?;
		Ok(())
	}

	pub fn reports_dump(&self) -> String {
		"--- reports ---\n".to_string()
			+ &self
				.stages
				.iter()
				.map(|Stage { reports, .. }| reports.join(""))
				.collect::<String>()
	}
}
