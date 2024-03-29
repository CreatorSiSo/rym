use super::Value;
use crate::{ast::VariableKind, error::Diagnostic};
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct Env {
    scopes: Vec<Scope>,
    pub emitter: Sender<Diagnostic>,
}

impl Env {
    pub fn new(sender: Sender<Diagnostic>) -> Self {
        Self {
            scopes: vec![Scope::new(ScopeKind::Module)],
            emitter: sender,
        }
    }

    pub fn with_constants(
        mut self,
        constants: impl IntoIterator<Item = (&'static str, Value)>,
    ) -> Self {
        for (name, value) in constants {
            self.create(name, VariableKind::Const, value)
        }
        self
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn variables(&self) -> Vec<Vec<(String, (VariableKind, Value))>> {
        self.scopes
            .iter()
            .map(|scope| {
                // TODO Too many clones
                let mut vars: Vec<_> = scope.vars.clone().into_iter().collect();
                vars.sort_by_key(|(name, _)| name.clone());
                vars
            })
            .collect()
    }

    pub fn create(&mut self, name: impl Into<String>, kind: VariableKind, value: Value) {
        self.scopes
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
        // search local scopes from inner-most outwards
        let mut prev_kind = ScopeKind::Expr;
        for scope in self.scopes.iter().rev() {
            // jump out of nested function scopes,
            // closures are not yet supported
            if prev_kind == ScopeKind::Function && scope.kind == ScopeKind::Function {
                continue;
            }

            if let Some((_, value)) = scope.vars.get(name) {
                return Some(value);
            }

            prev_kind = scope.kind;
        }

        // search modules
        // for scope in self.scopes.iter() {
        // 	// TODO
        // }

        None
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Function,
    Expr,
}
