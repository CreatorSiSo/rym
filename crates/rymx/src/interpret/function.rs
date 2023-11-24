use super::Value;
use crate::{
	ast::Function,
	interpret::{env::ScopeKind, Env, Interpret, VariableKind},
};

pub trait Call {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value;
}

impl Call for Function {
	fn call(&self, env: &mut Env, args: Vec<Value>) -> Value {
		assert!(self.params.len() == args.len());
		env.push_scope(ScopeKind::Function);

		for (param, arg) in self.params.iter().zip(args) {
			env.create(param.0.clone(), VariableKind::Let, arg)
		}
		let result = self.body.clone().eval(env);

		env.pop_scope();
		result
	}
}

#[derive(Debug, Clone)]
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
