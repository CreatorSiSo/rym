mod env;
mod function;

pub use self::env::Env;
use self::env::ScopeKind;
pub use self::function::{Call, NativeFunction};
use crate::ast::{BinaryOp, Expr, Function, Literal, Module, UnaryOp, VariableKind};

#[derive(Debug, Clone)]
pub enum Value {
	Bool(bool),
	Int(i64),
	Float(f64),
	String(String),
	Function(Function),
	NativeFunction(NativeFunction),
	// Type(Type),
	Unit,
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(val) => f.write_str(if *val { "true" } else { "false" }),
			Self::Int(val) => f.write_str(&val.to_string()),
			Self::Float(val) => f.write_str(&val.to_string()),
			Self::String(val) => f.write_fmt(format_args!("\"{val}\"")),
			Self::Function(_val) => f.write_str("<function>"),
			Self::NativeFunction(_val) => f.write_str("<native function>"),
			// Self::Type(_val) => f.write_str("<type>"),
			Self::Unit => f.write_str("()"),
		}
	}
}

pub trait Interpret {
	fn eval(self, env: &mut Env) -> Value;
}

impl Interpret for Module {
	fn eval(self, env: &mut Env) -> Value {
		// TODO sort based on dependency
		// self
		// 	.constants
		// 	.sort_by(|Constant { expr: l, .. }, Constant { expr: r, .. }| match (l, r) {});

		for (name, expr) in self.constants {
			let val = expr.eval(env);
			env.create(name, VariableKind::Const, val);
		}

		// TODO only do this when requested, ie. in main.rym file
		if let Some(main) = env.get("main") {
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
			Expr::Unit => Value::Unit,
			Expr::Literal(lit) => match lit {
				Literal::Bool(inner) => Value::Bool(inner),
				Literal::Int(inner) => Value::Int(inner),
				Literal::Float(inner) => Value::Float(inner),
				Literal::String(inner) => Value::String(inner),
			},
			Expr::Ident(name) => {
				// TODO Only clone when needed / faster
				env.get(&name).unwrap().clone()
			}
			Expr::Function(func) => Value::Function(func),

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
			Expr::Call(lhs, args) => match lhs.eval(env) {
				Value::Function(inner) => {
					let args = args.into_iter().map(|expr| expr.eval(env)).collect();
					inner.call(env, args)
				}
				_ => todo!(),
			},

			Expr::Block(exprs) => {
				env.push_scope(ScopeKind::Expr);
				for expr in exprs {
					expr.eval(env);
				}
				env.pop_scope();
				Value::Unit
			}
			Expr::Break(_) => todo!(),
			Expr::Return(_) => todo!(),

			Expr::Var(kind, name, expr) => {
				let val = expr.eval(env);
				env.create(name, kind, val);
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
