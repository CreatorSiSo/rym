use crate::{
	interpret::{eval_eq, NativeFunction, Value},
	Env,
};

pub fn std_env() -> crate::Env {
	Env::from_constants([
		(
			"println",
			Value::NativeFunction(NativeFunction::ParamsVar(|args| {
				let mut line = args.iter().fold(String::new(), |accum, value| {
					accum + &value.to_string() + " "
				});
				line.pop();
				println!("{line}");
				Value::Unit
			})),
		),
		(
			"assert_eq",
			Value::NativeFunction(NativeFunction::Params2(|arg0, arg1| {
				if !eval_eq(arg0, arg1) {
					panic!("{arg0} != {arg1}")
				}
				Value::Unit
			})),
		),
	])
}
