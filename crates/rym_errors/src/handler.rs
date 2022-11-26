use crate::Diagnostic;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Handler(RefCell<HandlerInner>);

#[derive(Debug, Default)]
struct HandlerInner {
	diagnostics: Vec<Diagnostic>,
}

/// Deals with errors and other output.
impl Handler {
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
