use rym_span::{Span, DUMMY_SPAN};

mod emitter;
mod handler;
mod tests;

pub use handler::DiagnosticHandler;

pub type RymResult<T> = Result<T, Diagnostic>;

pub trait IntoDiagnostic
where
	Self: Sized,
{
	#[must_use]
	fn into_diagnostic(self) -> Diagnostic;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
	level: Level,
	code: Option<&'static str>,
	title: String,
	children: Vec<SubDiagnostic>,
	primary_spans: Vec<Span>,
	span_labels: Vec<(Span, String)>,
}

impl Diagnostic {
	pub fn new<S: Into<String>>(level: Level, title: S) -> Self {
		Self::new_with_code(level, None, title)
	}

	pub fn new_with_code<S: Into<String>>(
		level: Level,
		code: Option<&'static str>,
		title: S,
	) -> Self {
		Self::new_spanned_with_code(level, code, title, DUMMY_SPAN)
	}

	pub fn new_spanned<S: Into<String>>(level: Level, title: S, span: Span) -> Self {
		Self::new_spanned_with_code(level, None, title, span)
	}

	pub fn new_spanned_with_code<S: Into<String>>(
		level: Level,
		code: Option<&'static str>,
		title: S,
		span: Span,
	) -> Self {
		Self {
			level,
			code,
			title: title.into(),
			children: vec![],
			primary_spans: if span.is_dummy() { vec![] } else { vec![span] },
			span_labels: vec![],
		}
	}

	pub fn sub_diagnostic<S: Into<String>>(
		mut self,
		level: Level,
		code: Option<&'static str>,
		title: S,
	) -> Self {
		self.children.push(SubDiagnostic { level, code, title: title.into() });
		self
	}

	pub fn span_label<S: Into<String>>(&mut self, span: Span, label: S) {
		if span.is_dummy() {
			return;
		}
		self.span_labels.push((span, label.into()));
		self.span_labels.sort_unstable();
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct SubDiagnostic {
	level: Level,
	code: Option<&'static str>,
	title: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Level {
	Error,
	Warning,
	Note,
	Help,
}

pub trait HandleDiagnostic<T> {
	fn ok_or_emit(self, handler: &DiagnosticHandler) -> Option<T>;
}

impl<T> HandleDiagnostic<T> for RymResult<T> {
	fn ok_or_emit(self, handler: &DiagnosticHandler) -> Option<T> {
		self.map_err(|err| handler.emit(err)).ok()
	}
}
