use std::collections::HashMap;

use crate::ast::{BinaryOp, Constant, Expr, Function, Module, UnaryOp, Value};

pub trait Interpret {
	fn eval(self, env: &mut Env) -> Value;
}

impl Interpret for Module {
	fn eval(self, env: &mut Env) -> Value {
		// TODO sort based on dependency
		// self
		// 	.constants
		// 	.sort_by(|Constant { expr: l, .. }, Constant { expr: r, .. }| match (l, r) {});

		for constant in self.constants {
			let val = constant.expr.eval(env);
			env.create(constant.name, VariableKind::Const, val);
		}

		// TODO only do this when requested, ie. in main.rym file
		if let Some(main) = env.variable_get("main") {
			match main {
				// TODO avoid cloning here
				Value::Function(val) => return val.clone().call(env, vec![]),
				_ => todo!(),
			}
		}

		Value::Unit
	}
}

impl Interpret for Expr {
	fn eval(self, env: &mut Env) -> Value {
		match self {
			Expr::Value(value) => value,
			Expr::Unary(op, expr) => match (op, expr.eval(env)) {
				(UnaryOp::Neg, Value::Float(val)) => Value::Float(-val),
				(UnaryOp::Neg, Value::Int(val)) => Value::Int(-val),
				(UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),

				(op, val) => todo!(),
			},
			Expr::Binary(op, lhs, rhs) => match (op, lhs.eval(env), rhs.eval(env)) {
				(BinaryOp::Add, Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs + rhs),
				(BinaryOp::Add, Value::Int(lhs), Value::Float(rhs)) => Value::Float(lhs as f64 + rhs),
				(BinaryOp::Add, Value::Float(lhs), Value::Int(rhs)) => Value::Float(lhs + rhs as f64),
				(BinaryOp::Sub, Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs - rhs),
				(BinaryOp::Sub, Value::Int(lhs), Value::Float(rhs)) => Value::Float(lhs as f64 - rhs),
				(BinaryOp::Sub, Value::Float(lhs), Value::Int(rhs)) => Value::Float(lhs - rhs as f64),
				(BinaryOp::Mul, Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs * rhs),
				(BinaryOp::Mul, Value::Int(lhs), Value::Float(rhs)) => Value::Float(lhs as f64 * rhs),
				(BinaryOp::Mul, Value::Float(lhs), Value::Int(rhs)) => Value::Float(lhs * rhs as f64),
				(BinaryOp::Div, Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs / rhs),
				(BinaryOp::Div, Value::Int(lhs), Value::Float(rhs)) => Value::Float(lhs as f64 / rhs),
				(BinaryOp::Div, Value::Float(lhs), Value::Int(rhs)) => Value::Float(lhs / rhs as f64),

				(BinaryOp::Add, Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs + rhs),
				(BinaryOp::Sub, Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs - rhs),
				(BinaryOp::Mul, Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs * rhs),
				(BinaryOp::Div, Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs / rhs),

				(BinaryOp::Add, Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),

				(BinaryOp::Eq, lhs, rhs) => Value::Bool(eval_eq(lhs, rhs)),
				(BinaryOp::NotEq, lhs, rhs) => Value::Bool(!eval_eq(lhs, rhs)),

				(op, lhs, rhs) => todo!(),
			},
			Expr::Ident(name) => {
				// TODO Only clone when needed / faster
				env.variable_get(&name).unwrap().clone()
			}
			Expr::Constant(Constant { name, expr }) => {
				let val = expr.eval(env);
				env.create(name, VariableKind::Const, val);
				Value::Unit
			}
		}
	}
}

fn eval_eq(lhs: Value, rhs: Value) -> bool {
	match (lhs, rhs) {
		(Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
		(Value::Int(lhs), Value::Int(rhs)) => lhs == rhs,
		(Value::Float(lhs), Value::Float(rhs)) => lhs == rhs,
		(Value::String(lhs), Value::String(rhs)) => lhs == rhs,
		(Value::Unit, Value::Unit) => todo!(),
		_ => todo!(),
	}
}

pub trait Call {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value;
}

impl Call for Function {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value {
		assert_eq!(self.params.len(), args.len());

		// TODO correct addition and removal of variables on Env
		for (param, arg) in self.params.iter().zip(args) {
			env.create(param.0.clone(), VariableKind::Let, arg)
		}
		self.body.clone().eval(env)
	}
}

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

	pub fn variable_assign(&mut self) {
		todo!()
	}

	pub fn variable_get(&mut self, name: &str) -> Option<&Value> {
		self
			.scopes
			.last_mut()
			.unwrap()
			.vars
			.get(name)
			.map(|variable| &variable.value)
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
