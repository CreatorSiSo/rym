use clap::{arg, command, Command};
use rustyline::{error::ReadlineError, Editor};
use rymx::{
	compile_expr, compile_module, interpret, Diagnostics, Env, NativeFunction, SourceId, VariableKind,
};
use std::{
	fs::{read_to_string, File},
	path::PathBuf,
};

#[derive(Debug)]
struct Arguments {}

fn main() -> anyhow::Result<()> {
	let mut command = command!()
		.arg(arg!(-w --write <"outputs|reports"> ... "Write outputs to a debug file"))
		.subcommand(Command::new("repl").about("Start a repl session"))
		.subcommand(
			Command::new("run")
				.about("Execute a file")
				.arg(arg!(<file> "File to execute")),
		);

	let help_str = command.render_help();
	let global_matches = command.get_matches();
	let write: Vec<String> = global_matches
		.get_many("write")
		.map_or(vec![], |option| option.cloned().collect());

	match global_matches.subcommand() {
		Some(("repl", _)) => cmd_repl(write)?,
		Some(("run", sub_matches)) => {
			cmd_run(write, sub_matches.get_one::<String>("file").unwrap().into())?
		}
		_ => print!("{}", help_str.ansi()),
	}

	Ok(())
}

fn cmd_repl(write: Vec<String>) -> anyhow::Result<()> {
	let mut editor: Editor<(), _> = Editor::new()?;
	if editor.load_history(".history").is_err() {
		println!("No previous history.");
	}
	let mut env = Env::new();
	env.create(
		"println",
		VariableKind::Const,
		rymx::Value::NativeFunction(NativeFunction::ParamsVar(|args| {
			let mut line = args.iter().fold(String::new(), |accum, value| {
				accum + &value.to_string() + " "
			});
			line.pop();
			println!("{line}");
			rymx::Value::Unit
		})),
	);

	loop {
		let readline = editor.readline("➤ ");
		let mut diag = Diagnostics::new(
			// Deletes the previous contents of repl.debug
			Box::new(File::create(PathBuf::from("repl.debug"))?),
			Box::new(std::io::stderr()),
		);

		match readline {
			Ok(line) => {
				match line.as_str() {
					"" => continue,
					":help" => continue,
					_ => (),
				}

				editor.add_history_entry(&line).unwrap();
				diag.set_other_src("repl", &line);

				if let Ok(expr) = compile_expr(&mut diag, &line, SourceId::Other("repl")) {
					let val = interpret(&mut diag, &mut env, expr);
					println!("{val}");
				}

				if write.contains(&"outputs".to_string()) {
					diag.save_outputs()?;
				}
				if write.contains(&"reports".to_string()) {
					diag.save_reports()?;
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}

	editor.save_history(".history")?;
	Ok(())
}

fn cmd_run(write: Vec<String>, path: PathBuf) -> anyhow::Result<()> {
	let src = read_to_string(&path)?;
	let mut diag = Diagnostics::new(
		if write.is_empty() {
			Box::new(std::io::sink())
		} else {
			Box::new(File::create(path.with_extension("debug"))?)
		},
		Box::new(std::io::stderr()),
	);
	let mut env = Env::new();

	// Ignoring the result here as it is already alvailabe
	// through the Diagnostics
	if let Ok(module) = compile_module(&mut diag, &src, SourceId::File(path)) {
		interpret(&mut diag, &mut env, module);
	}

	if write.contains(&"outputs".to_string()) {
		diag.save_outputs()?;
	}
	if write.contains(&"reports".to_string()) {
		diag.save_reports()?;
	}
	Ok(())
}
