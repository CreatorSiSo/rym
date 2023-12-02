mod env;
mod function;

pub use self::env::Env;
use self::env::ScopeKind;
pub use self::function::{Call, Function, NativeFunction};
use crate::ast::{BinaryOp, Expr, Literal, Module, UnaryOp, VariableKind};

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

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
			(Self::Int(l0), Self::Int(r0)) => l0 == r0,
			(Self::Float(l0), Self::Float(r0)) => l0 == r0,
			(Self::String(l0), Self::String(r0)) => l0 == r0,
			(Self::Function(l0), Self::Function(r0)) => l0 == r0,
			(Self::NativeFunction(l0), Self::NativeFunction(r0)) => l0 == r0,
			(Value::Unit, Value::Unit) => todo!(),
			// TODO These cases should not be accessible, protect them via type checking
			_ => false,
		}
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(val) => f.write_str(if *val { "true" } else { "false" }),
			Self::Int(val) => f.write_str(&val.to_string()),
			Self::Float(val) => f.write_str(&val.to_string()),
			Self::String(val) => f.write_str(val),
			Self::Function(val) => f.write_str(&val.to_string()),
			Self::NativeFunction(val) => f.write_str(&val.to_string()),
			// Self::Type(_val) => f.write_str("<type>"),
			Self::Unit => f.write_str("()"),
		}
	}
}

pub enum ControlFlow {
	None(Value),
	Break(Value),
	Return(Value),
}

impl ControlFlow {
	pub fn inner(self) -> Value {
		match self {
			ControlFlow::None(inner) | ControlFlow::Break(inner) | ControlFlow::Return(inner) => inner,
		}
	}
}

macro_rules! default_flow {
	($control_flow:expr) => {
		match $control_flow {
			ControlFlow::None(inner) => inner,
			control_flow => return control_flow,
		}
	};
}

pub trait Interpret {
	fn eval(self, env: &mut Env) -> ControlFlow;
}

impl Interpret for Module {
	fn eval(self, env: &mut Env) -> ControlFlow {
		// TODO sort based on dependency
		// self
		// 	.constants
		// 	.sort_by(|Constant { expr: l, .. }, Constant { expr: r, .. }| match (l, r) {});

		for (name, expr) in self.constants {
			// Top level, ignoring control flow
			let val = match expr.eval(env) {
				ControlFlow::None(inner) | ControlFlow::Break(inner) | ControlFlow::Return(inner) => inner,
			};
			env.create(name, VariableKind::Const, val);
		}

		// TODO only do this when requested, ie. in main.rym file
		if let Some(main) = env.get("main") {
			match main {
				Value::Function(val) => {
					// TODO avoid cloning here
					val.clone().call(env, vec![]);
				}
				_ => todo!(),
			}
		}

		ControlFlow::Break(Value::Unit)
	}
}

impl Interpret for Expr {
	fn eval(self, env: &mut Env) -> ControlFlow {
		let result = match self {
			Expr::Unit => Value::Unit,
			Expr::Literal(lit) => match lit {
				Literal::Bool(inner) => Value::Bool(inner),
				Literal::Int(inner) => Value::Int(inner),
				Literal::Float(inner) => Value::Float(inner),
				Literal::String(inner) => Value::String(inner),
			},
			Expr::Ident(name) => match env.get(&name) {
				// TODO Only clone when needed / faster
				Some(val) => val.clone(),
				None => panic!("Unable to find <{name}>"),
			},
			Expr::Function(func) => Value::Function(func),

			Expr::Unary(op, expr) => match (op, default_flow!(expr.eval(env))) {
				(UnaryOp::Neg, Value::Float(val)) => Value::Float(-val),
				(UnaryOp::Neg, Value::Int(val)) => Value::Int(-val),
				(UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),

				(op, val) => todo!(),
			},
			Expr::Binary(op, lhs, rhs) => match (
				op,
				default_flow!(lhs.eval(env)),
				default_flow!(rhs.eval(env)),
			) {
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

				(BinaryOp::Eq, lhs, rhs) => Value::Bool(lhs == rhs),
				(BinaryOp::NotEq, lhs, rhs) => Value::Bool(lhs != rhs),

				(op, lhs, rhs) => todo!(),
			},
			Expr::Call(lhs, args) => match default_flow!(lhs.eval(env)) {
				Value::Function(inner) => {
					let mut arg_values = vec![];
					for expr in args {
						arg_values.push(default_flow!(expr.eval(env)));
					}
					inner.call(env, arg_values)
				}
				Value::NativeFunction(inner) => {
					let mut arg_values = vec![];
					for expr in args {
						arg_values.push(default_flow!(expr.eval(env)));
					}
					inner.call(env, arg_values)
				}
				_ => todo!("Add error, value is not a function."),
			},

			Expr::IfElse(cond_expr, then_expr, else_expr) => {
				let Value::Bool(condition) = default_flow!(cond_expr.eval(env)) else {
					todo!();
				};
				if condition {
					default_flow!(then_expr.eval(env))
				} else {
					default_flow!(else_expr.eval(env))
				}
			}
			Expr::Block(exprs) => {
				env.push_scope(ScopeKind::Expr);
				let mut result = Value::Unit;
				'exprs_loop: for expr in exprs {
					match expr.eval(env) {
						ControlFlow::None(_) => (),
						ControlFlow::Break(inner) => {
							result = inner;
							break 'exprs_loop;
						}
						control_flow => return control_flow,
					}
				}
				env.pop_scope();
				result
			}
			Expr::Break(expr) => return ControlFlow::Break(default_flow!(expr.eval(env))),
			Expr::Return(expr) => return ControlFlow::Return(default_flow!(expr.eval(env))),

			Expr::Var(kind, name, expr) => {
				let val = default_flow!(expr.eval(env));
				env.create(name, kind, val);
				Value::Unit
			}
		};

		ControlFlow::None(result)
	}
}
