use std::collections::HashMap;
use std::{fmt::Debug, path::PathBuf};

mod ast;
mod parse;
mod tokenize;
use ariadne::{Cache, FileCache, Label, Report, Source};
use ast::{Expr, Module};
use span::SourceSpan;
pub use tokenize::tokenizer;
mod span;
pub use span::Span;
use tokenize::Token;

pub fn compile_module(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Result<Module, ()> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src, src_id.clone())?;

	diag.start_stage("parse");
	let module = match parse::parse_module(&tokens, src, src_id) {
		Ok(module) => module,
		Err(reports) => {
			for report in reports {
				diag.push_report(report);
			}
			return Err(());
		}
	};
	diag.push_result(&format!("{module:#?}\n"));

	// TODO Name resolution
	// TODO Typechecking
	// TODO Const evaluation
	// TODO Generate intermediate representation

	// i / / / ;

	Ok(module)
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_expr(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Result<Expr, ()> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src, src_id.clone())?;

	diag.start_stage("parse");
	let expr = match parse::parse_expr(&tokens, src, src_id) {
		Ok(expr) => expr,
		Err(reports) => {
			for report in reports {
				diag.push_report(report);
			}
			return Err(());
		}
	};
	diag.push_result(&format!("{expr:#?}\n"));

	Ok(expr)
}

fn tokenize(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Result<Vec<(Token, Span)>, ()> {
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
				format!("Invalid character [{}]", span.src(src)),
				SourceSpan(src_id, span),
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

pub fn error(diag: &mut Diagnostics, message: String, src_span: SourceSpan) {
	diag.push_report(
		Report::build(ariadne::ReportKind::Error, src_span.0.clone(), 0)
			.with_label(Label::new(src_span).with_message(message))
			.finish(),
	);
}

pub struct Diagnostics {
	logs_out: Box<dyn std::io::Write>,
	runtime_out: Box<dyn std::io::Write>,
	stages: Vec<Stage>,
	cache: DynamicCache,
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
			cache: DynamicCache::new(),
		}
	}

	pub fn set_other_src(&mut self, src_id: &'static str, src: &str) {
		self.cache.other.insert(src_id, Source::from(src));
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

	/// Associates a report (debug, warning, error) with the current stage
	pub fn push_report(&mut self, report: ariadne::Report<'_, SourceSpan>) {
		report
			.write(&mut self.cache, &mut self.runtime_out)
			.unwrap();
		self.runtime_out.flush().unwrap();

		let mut string = String::new();
		// TODO Turn this into a safe operation
		report
			.write(&mut self.cache, unsafe { string.as_mut_vec() })
			.unwrap();
		self
			.stages
			.last_mut()
			.unwrap()
			.reports
			.push(strip_ansi_escapes::strip_str(string));
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

struct DynamicCache {
	files: FileCache,
	other: HashMap<&'static str, Source>,
}

impl Cache<SourceId> for &mut DynamicCache {
	fn fetch(&mut self, id: &SourceId) -> Result<&Source, Box<dyn std::fmt::Debug + '_>> {
		match id {
			SourceId::File(pathbuf) => self.files.fetch(&pathbuf),
			SourceId::Other(id) => self.other.get(id).ok_or(Box::new(format!(
				"Could not find source cache entry [{id}]"
			))),
		}
	}

	fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn std::fmt::Display + 'a>> {
		match id {
			SourceId::File(pathbuf) => Some(Box::new(pathbuf.display())),
			SourceId::Other(id) => Some(Box::new(*id)),
		}
	}
}

impl DynamicCache {
	fn new() -> Self {
		Self {
			files: FileCache::default(),
			other: HashMap::new(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceId {
	File(PathBuf),
	Other(&'static str),
}
