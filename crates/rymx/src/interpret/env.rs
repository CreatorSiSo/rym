use crate::ast::Value;
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

	pub fn variables(&self) -> impl Iterator<Item = (&String, &Variable)> {
		self.scopes.iter().map(|scope| scope.vars.iter()).flatten()
	}

	pub fn create(&mut self, name: impl Into<String>, kind: VariableKind, value: Value) {
		self
			.scopes
			.last_mut()
			.unwrap()
			.vars
			.insert(name.into(), Variable { value, kind });
	}

	pub fn assign(&mut self, name: &str, value: Value) {
		let Some(var) = self.scopes.last_mut().unwrap().vars.get_mut(name) else {
			todo!()
		};
		match var.kind {
			VariableKind::Const => todo!("Cannot assign to const"),
			VariableKind::Let => todo!("Cannot assign to let"),
			VariableKind::LetMut => var.value = value,
		}
	}

	pub fn get(&self, name: &str) -> Option<&Value> {
		let mut in_function = false;
		for scope in self.scopes.iter().rev() {
			if in_function && scope.kind == ScopeKind::Function {
				continue;
			}
			if let Some(val) = scope.vars.get(name) {
				return Some(&val.value);
			}
			in_function = matches!(scope.kind, ScopeKind::Function);
		}
		None
	}
}

struct Scope {
	vars: HashMap<String, Variable>,
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

pub struct Variable {
	pub value: Value,
	pub kind: VariableKind,
}

pub enum VariableKind {
	Const,
	Let,
	LetMut,
}
