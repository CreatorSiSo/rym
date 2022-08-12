use super::error::RuntimeError;
use rym_ast::Literal;
use std::collections::HashMap;

pub struct Variable<'src> {
	value: Literal<'src>,
	mutable: bool,
}

type Scope<'src> = HashMap<&'src str, Variable<'src>>;

pub struct Env<'src> {
	scopes: Vec<Scope<'src>>,
}

impl<'src> Env<'src> {
	pub fn new() -> Self {
		Self {
			scopes: vec![Scope::new()],
		}
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new())
	}

	pub fn pop_scope(&mut self) {
		if self.scopes.len() > 1 {
			self.scopes.pop();
		}
	}

	pub fn get(&self, name: &str) -> Result<&Literal, RuntimeError> {
		for scope in self.iter() {
			match scope.get(name) {
				Some(var) => return Ok(&var.value),
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub fn set(&mut self, name: &str, new: Literal<'src>) -> Result<(), RuntimeError> {
		for scope in self.iter_mut() {
			match scope.get_mut(name) {
				Some(var) => {
					if !var.mutable {
						return RuntimeError::assignment(name, new);
					}
					var.value = new;
					return Ok(());
				}
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub fn declare(&mut self, name: &'src str, value: Literal<'src>, mutable: bool) {
		self.last_mut().insert(
			name,
			Variable {
				// TODO: Clone?
				value,
				mutable,
			},
		);
	}
}

impl<'src> Env<'src> {
	fn last_mut(&mut self) -> &mut Scope<'src> {
		self
			.scopes
			.last_mut()
			.expect("Internal Error: Stack should never be empty!")
	}

	fn iter(&self) -> std::iter::Rev<std::slice::Iter<Scope>> {
		self.scopes.iter().rev()
	}

	fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<Scope<'src>>> {
		self.scopes.iter_mut().rev()
	}
}
