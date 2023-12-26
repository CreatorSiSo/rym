use super::Value;
use crate::{
	ast,
	interpret::{env::ScopeKind, Env, Interpret, VariableKind},
};
use std::fmt::Display;

pub trait Call {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value;
}

impl Call for ast::Function {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value {
		assert!(self.params.len() == args.len());
		env.push_scope(ScopeKind::Function);

		for ((name, typ), arg) in self.params.iter().zip(args) {
			env.create(name.clone(), VariableKind::Let, arg)
		}
		let result = self.body.clone().eval(env);

		env.pop_scope();
		result.inner()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum NativeFunction {
	Params1(fn(&Value) -> Value),
	Params2(fn(&Value, &Value) -> Value),
	ParamsVar(fn(&[Value]) -> Value),
}

impl Call for NativeFunction {
	fn call(&self, _env: &mut Env, args: Vec<Value>) -> Value {
		match self {
			NativeFunction::Params1(inner) => {
				assert!(args.len() == 1);
				inner(&args[0])
			}
			NativeFunction::Params2(inner) => {
				assert!(args.len() == 2);
				inner(&args[0], &args[1])
			}
			NativeFunction::ParamsVar(inner) => inner(&args),
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
