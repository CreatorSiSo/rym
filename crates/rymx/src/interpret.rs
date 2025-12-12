use crate::error::Diagnostic;
use ast::*;
use std::fmt::Display;
use std::{collections::HashMap, sync::mpsc::Sender};

pub trait Interpret {
    fn eval() -> Value;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    NativeFunction(NativeFunction),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "Unit"),
            Value::Bool(val) => write!(f, "{val}"),
            Value::Int(val) => write!(f, "{val}"),
            Value::Float(val) => write!(f, "{val}"),
            Value::String(val) => write!(f, "{val}"),
            Value::NativeFunction(fun) => write!(f, "{fun}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    None(Value),
    Break(Value),
    Continue,
    Return(Value),
    Exit,
}

pub trait Call {
    fn call(&self, env: &mut Env, args: Vec<Value>) -> ControlFlow;
}

// impl Call for ast::Function<'_> {
//     fn call(&self, env: &mut Env, args: Vec<Value>) -> ControlFlow {
//         assert!(self.params.len() == args.len());
//         env.push_scope(ScopeKind::Function);

//         for (param, arg) in self.params.iter().zip(args) {
//             env.create(param.name.clone(), VariableKind::Let, arg)
//         }
//         let result = self.body.clone().eval(env);

//         env.pop_scope();
//         result
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum NativeFunction {
    Params1(fn(&Value) -> Value),
    Params2(fn(&Value, &Value) -> Value),
    ParamsVar(fn(&[Value]) -> Value),
}

impl Call for NativeFunction {
    fn call(&self, _env: &mut Env, args: Vec<Value>) -> ControlFlow {
        match self {
            NativeFunction::Params1(inner) => {
                assert!(args.len() == 1);
                ControlFlow::None(inner(&args[0]))
            }
            NativeFunction::Params2(inner) => {
                assert!(args.len() == 2);
                ControlFlow::None(inner(&args[0], &args[1]))
            }
            NativeFunction::ParamsVar(inner) => ControlFlow::None(inner(&args)),
        }
    }
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeFunction::Params1(_func) => f.write_str("extern fn(1)"),
            NativeFunction::Params2(_func) => f.write_str("extern fn(2)"),
            NativeFunction::ParamsVar(_func) => f.write_str("extern fn(..[]TODO)"),
        }
    }
}

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
