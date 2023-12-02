use crate::interpret::{NativeFunction, Value};

pub const CONSTANTS: [(&'static str, Value); 5] = [
	(
		"print",
		wrap_fn_var(|args| {
			print!("{}", apply_format(args));
			Value::Unit
		}),
	),
	(
		"println",
		wrap_fn_var(|args| {
			println!("{}", apply_format(args));
			Value::Unit
		}),
	),
	(
		"read_to_string",
		wrap_fn_1(|arg| {
			let Value::String(path) = arg else { panic!() };
			Value::String(std::fs::read_to_string(path).unwrap())
		}),
	),
	(
		"assert",
		wrap_fn_1(|arg| {
			if *arg == Value::Bool(true) {
				panic!("assertion failed: {arg}")
			}
			Value::Unit
		}),
	),
	(
		"assert_eq",
		wrap_fn_2(|arg0, arg1| {
			if arg0 != arg1 {
				panic!("assertion `{arg0} == {arg1}` failed\n\tleft: {arg0}\n\tright: {arg1}")
			}
			Value::Unit
		}),
	),
];

fn apply_format(args: &[Value]) -> String {
	let mut result = args.iter().fold(String::new(), |accum, value| {
		accum + &value.to_string() + " "
	});
	result.pop();
	result
}

pub const OTHER: [(&'static str, Value); 1] = [(
	"fib_native",
	wrap_fn_1(|arg0| match arg0 {
		Value::Int(int) => Value::Int(fib_native(*int)),
		_ => panic!(),
	}),
)];

fn fib_native(n: i64) -> i64 {
	if n == 0 {
		0
	} else if n == 1 {
		1
	} else {
		fib_native(n - 1) + fib_native(n - 2)
	}
}

const fn wrap_fn_1(f: fn(&Value) -> Value) -> Value {
	Value::NativeFunction(NativeFunction::Params1(f))
}

const fn wrap_fn_2(f: fn(&Value, &Value) -> Value) -> Value {
	Value::NativeFunction(NativeFunction::Params2(f))
}

const fn wrap_fn_var(f: fn(&[Value]) -> Value) -> Value {
	Value::NativeFunction(NativeFunction::ParamsVar(f))
}
