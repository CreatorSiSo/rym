use crate::Diagnostic;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct DiagnosticHandler(RefCell<DiagnosticHandlerInner>);

#[derive(Debug, Default)]
struct DiagnosticHandlerInner {
	diagnostics: Vec<Diagnostic>,
}

/// Deals with errors and other output.
impl DiagnosticHandler {
	pub fn emit(&self, diagnostic: Diagnostic) {
		match self.0.try_borrow_mut() {
			Ok(mut inner) => inner.diagnostics.push(diagnostic),
			Err(err) => panic!("Internal Error: Handler is already mutably borrowed: {err}"),
		}
	}

	pub fn collect(self) -> Vec<Diagnostic> {
		self.0.take().diagnostics
	}
}
