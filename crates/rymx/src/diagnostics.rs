use crate::SourceSpan;
use ariadne::{Cache, FileCache, Label, Report, Source};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::path::PathBuf;

pub struct Diagnostics {
	logs_out: Box<dyn std::io::Write>,
	runtime_out: Box<dyn std::io::Write>,
	stages: Vec<Stage>,
	cache: DynamicSourceCache,
}

impl Diagnostics {
	pub fn new(logs_out: Box<dyn std::io::Write>, runtime_out: Box<dyn std::io::Write>) -> Self {
		Self {
			logs_out,
			runtime_out,
			stages: vec![],
			cache: DynamicSourceCache::new(),
		}
	}

	pub fn error(&mut self, message: String, src_span: SourceSpan) {
		self.push_report(
			Report::build(
				ariadne::ReportKind::Error,
				src_span.0.clone(),
				src_span.1.start,
			)
			.with_label(Label::new(src_span).with_message(message))
			.finish(),
		);
	}

	pub fn set_other_src(&mut self, name: &'static str, src: &str) -> SourceId {
		self.cache.other.insert(name, Source::from(src));
		SourceId::Other(name)
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
		self
			.stages
			.last_mut()
			.expect("Internal Error: called Diagnostics::push_result() before adding a stage")
			.output
			.push_str(data);
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

	pub fn write_outputs(&mut self) -> anyhow::Result<()> {
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

	pub fn write_reports(&mut self) -> anyhow::Result<()> {
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

#[derive(Debug)]
struct Stage {
	name: &'static str,
	reports: Vec<String>,
	output: String,
}

struct DynamicSourceCache {
	files: FileCache,
	other: HashMap<&'static str, Source>,
}

impl Cache<SourceId> for &mut DynamicSourceCache {
	fn fetch(&mut self, id: &SourceId) -> Result<&Source, Box<dyn Debug + '_>> {
		match id {
			SourceId::File(pathbuf) => self.files.fetch(pathbuf),
			SourceId::Other(id) => self.other.get(id).ok_or(Box::new(format!(
				"Could not find source cache entry [{id}]"
			))),
		}
	}

	fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn Display + 'a>> {
		match id {
			SourceId::File(pathbuf) => Some(Box::new(pathbuf.display())),
			SourceId::Other(id) => Some(Box::new(*id)),
		}
	}
}

impl DynamicSourceCache {
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
