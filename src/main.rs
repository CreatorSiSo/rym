use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
	// https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html

	let process = match Command::new("cargo")
		.args(["test", "--color", "always"])
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
	{
		Err(why) => panic!("Couldn't spawn cargo: {}", why),
		Ok(process) => process,
	};

	// The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
	let mut cargo_out = String::new();
	if let Err(why) = process.stderr.unwrap().read_to_string(&mut cargo_out) {
		panic!("couldn't read cargo stderr: {}", why)
	}

	let aliases = [
		("Token", "rym_lexer::rich::Token"),
		("Span", "std::ops::Range<usize>"),
		("Error", "chumsky::error::Rich<Token, Span>"),
		("Extra", "chumsky::extra::Full<Error, (), ()>"),
		("ParserInput", "SpannedInput<Token, Span, &[(Token, Span)]>"),
	];

	for (alias, expanded) in aliases {
		cargo_out = cargo_out.replace(expanded, alias);
	}

	println!("\n--- Unexpanded cargo test output ---\n\n{cargo_out}")
}
