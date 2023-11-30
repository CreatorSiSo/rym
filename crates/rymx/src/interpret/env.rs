use super::Value;
use crate::ast::VariableKind;
use std::collections::HashMap;

pub struct Env {
	scopes: Vec<Scope>,
}

impl Env {
	pub fn new() -> Self {
		Self {
			scopes: vec![Scope::new(ScopeKind::Module)],
		}
	}

	pub fn push_scope(&mut self, kind: ScopeKind) {
		self.scopes.push(Scope::new(kind));
	}

	pub fn pop_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn variables(&self) -> impl Iterator<Item = (&String, &(VariableKind, Value))> {
		self.scopes.iter().flat_map(|scope| scope.vars.iter())
	}

	pub fn create(&mut self, name: impl Into<String>, kind: VariableKind, value: Value) {
		self
			.scopes
			.last_mut()
			.unwrap()
			.vars
			.insert(name.into(), (kind, value));
	}

	pub fn assign(&mut self, name: &str, value: Value) {
		let Some((kind, value_mut)) = self.scopes.last_mut().unwrap().vars.get_mut(name) else {
			todo!()
		};
		match kind {
			VariableKind::Const => todo!("Cannot assign to const"),
			VariableKind::Let => todo!("Cannot assign to let"),
			VariableKind::LetMut => *value_mut = value,
		}
	}

	pub fn get(&self, name: &str) -> Option<&Value> {
		let mut in_function = false;
		for scope in self.scopes.iter().rev() {
			if in_function && scope.kind == ScopeKind::Function {
				continue;
			}
			if let Some((_, value)) = scope.vars.get(name) {
				return Some(value);
			}
			in_function = matches!(scope.kind, ScopeKind::Function);
		}
		None
	}
}

impl Default for Env {
	fn default() -> Self {
		Self::new()
	}
}

struct Scope {
	vars: HashMap<String, (VariableKind, Value)>,
	kind: ScopeKind,
}

impl Scope {
	fn new(kind: ScopeKind) -> Self {
		Self {
			vars: HashMap::new(),
			kind,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScopeKind {
	Module,
	Function,
	Expr,
}
