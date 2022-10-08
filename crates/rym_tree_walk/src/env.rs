use crate::Value;

use super::error::RuntimeError;
use std::collections::HashMap;

pub(crate) struct Variable {
	value: Value,
	is_const: bool,
}

type Scope<'scope> = HashMap<&'scope str, Variable>;

pub(crate) struct Env<'scope> {
	scopes: Vec<Scope<'scope>>,
}

impl<'scope> Env<'scope> {
	pub(crate) fn new() -> Self {
		Self {
			scopes: vec![Scope::new()],
		}
	}

	pub(crate) fn push_scope(&mut self) {
		self.scopes.push(Scope::new())
	}

	pub(crate) fn pop_scope(&mut self) {
		if self.scopes.len() > 1 {
			self.scopes.pop();
		}
	}

	pub(crate) fn get(&self, name: &str) -> Result<&Value, RuntimeError> {
		for scope in self.iter() {
			match scope.get(name) {
				Some(var) => return Ok(&var.value),
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub(crate) fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
		for scope in self.iter_mut() {
			match scope.get_mut(name) {
				Some(var) => {
					if var.is_const {
						return RuntimeError::const_assign(name, value);
					}
					var.value = value;
					return Ok(());
				}
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub(crate) fn declare(&mut self, name: &'scope str, value: Value, is_const: bool) {
		self.last_mut().insert(
			name,
			Variable {
				// TODO: Clone?
				value,
				is_const,
			},
		);
	}
}

impl<'scope> Env<'scope> {
	fn last_mut(&mut self) -> &mut Scope<'scope> {
		self
			.scopes
			.last_mut()
			.expect("Internal Error: Stack should never be empty!")
	}

	fn iter(&self) -> std::iter::Rev<std::slice::Iter<Scope<'scope>>> {
		self.scopes.iter().rev()
	}

	fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<Scope<'scope>>> {
		self.scopes.iter_mut().rev()
	}
}
