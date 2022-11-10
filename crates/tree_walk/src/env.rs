use crate::value::Value;

use super::error::RuntimeError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct Variable {
	value: Value,
	is_const: bool,
}

#[derive(Debug, Clone)]
struct Scope(HashMap<String, Variable>);

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

impl Scope {
	fn new() -> Self {
		Self(HashMap::new())
	}
}

pub(crate) struct GlobalEnv {
	pub env: Env,
	envs: Vec<Env>,
}

impl GlobalEnv {
	pub(crate) fn new() -> Self {
		Self {
			env: Env::new(),
			envs: vec![Env::new()],
		}
	}

	pub(crate) fn get(&self, name: &str) -> Result<&Value, RuntimeError> {
		self.current_env().get(name).or(self.env.get(name))
	}

	pub(crate) fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
		self
			.current_env_mut()
			.set(name, value.clone())
			.or(self.env.set(name, value))
	}

	pub(crate) fn declare(&mut self, name: &str, value: Value, is_const: bool) {
		self.current_env_mut().declare(name, value, is_const)
	}

	pub(crate) fn push_env(&mut self) {
		self.envs.push(Env::new());
	}

	pub(crate) fn pop_env(&mut self) {
		let envs = &mut self.envs;
		if envs.is_empty() {
			panic!("Internal Error: Tried to pop off last env of {:?}", envs)
		} else {
			envs.pop();
		}
	}

	pub(crate) fn push_scope(&mut self) {
		self.current_env_mut().scopes.push(Scope::new())
	}

	pub(crate) fn pop_scope(&mut self) {
		let scopes = &mut self.current_env_mut().scopes;
		if scopes.is_empty() {
			panic!("Internal Error: Tried to pop off last env of {:?}", scopes)
		} else {
			scopes.pop();
		}
	}

	fn current_env(&self) -> &Env {
		self
			.envs
			.last()
			.expect("Internal Error: Envs stack should never be empty")
	}

	fn current_env_mut(&mut self) -> &mut Env {
		self
			.envs
			.last_mut()
			.expect("Internal Error: Envs stack should never be empty")
	}
}

#[derive(Clone, Debug)]
pub(crate) struct Env {
	scopes: Vec<Scope>,
}

impl Env {
	pub(crate) fn new() -> Self {
		Self {
			scopes: vec![Scope::new()],
		}
	}
}

impl Env {
	pub(crate) fn get(&self, name: &str) -> Result<&Value, RuntimeError> {
		for scope in self.iter_scopes() {
			if let Some(var) = scope.0.get(name) {
				return Ok(&var.value);
			}
			continue;
		}
		RuntimeError::undeclared_var(name)
	}

	pub(crate) fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
		for scope in self.iter_scopes_mut() {
			if let Some(variable) = scope.0.get_mut(name) {
				if variable.is_const {
					return RuntimeError::const_assign(name, value);
				}
				variable.value = value;
				return Ok(());
			}
			continue;
		}
		RuntimeError::undeclared_var(name)
	}

	pub(crate) fn declare(&mut self, name: &str, value: Value, is_const: bool) {
		self.current_scope_mut().0.insert(
			name.to_owned(),
			Variable {
				// TODO: Clone?
				value,
				is_const,
			},
		);
	}

	fn iter_scopes(&self) -> impl Iterator<Item = &Scope> {
		self.scopes.iter().rev()
	}

	fn iter_scopes_mut(&mut self) -> impl Iterator<Item = &mut Scope> {
		self.scopes.iter_mut().rev()
	}

	fn current_scope_mut(&mut self) -> &mut Scope {
		self
			.scopes
			.last_mut()
			.expect("Internal Error: Scopes stack should never be empty")
	}
}
