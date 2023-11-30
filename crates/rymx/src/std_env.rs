use crate::{
	interpret::{NativeFunction, Value},
	Env,
};

pub fn std_env() -> crate::Env {
	Env::from_constants([(
		"println",
		Value::NativeFunction(NativeFunction::ParamsVar(|args| {
			let mut line = args.iter().fold(String::new(), |accum, value| {
				accum + &value.to_string() + " "
			});
			line.pop();
			println!("{line}");
			Value::Unit
		})),
	)])
}
