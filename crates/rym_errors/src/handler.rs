use crate::{Diagnostic, RymResult};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Handler(RefCell<HandlerInner>);

#[derive(Debug, Default)]
struct HandlerInner {
	diagnostics: Vec<Diagnostic>,
}

/// Deals with errors and other output.
impl Handler {
	// TODO: Add new trait that provides a ok_or_emit(handler: &Handler) -> Option<T> {} function for RymResult<T>
	pub fn handle<T>(&self, result: RymResult<T>) -> Option<T> {
		match result {
			Ok(val) => Some(val),
			Err(err) => {
				self.emit(err);
				None
			}
		}
	}

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
