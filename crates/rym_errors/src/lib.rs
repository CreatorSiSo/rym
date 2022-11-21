use rym_span::{Span, DUMMY_SPAN};

mod emitter;
mod tests;

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
			primary_spans: if span.is_dummy() { vec![] } else { vec![span] },
			span_labels: vec![],
		}
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
pub enum Level {
	Error,
	Warning,
	Note,
	Help,
}