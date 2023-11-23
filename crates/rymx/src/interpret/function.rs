use crate::{
	ast::{Function, Value},
	interpret::{Env, Interpret, VariableKind},
};

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

enum NativeFunction {
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
