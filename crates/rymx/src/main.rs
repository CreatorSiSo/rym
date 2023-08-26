use clap::{arg, command, Command};
use rustyline::{error::ReadlineError, Editor};
use rymx::{compile, Diagnostics};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug)]
struct Arguments {}

fn main() -> anyhow::Result<()> {
	let mut command = command!()
		.arg(arg!(--stages "Write compilation stages to debug files"))
		.subcommand(Command::new("repl").about("Start a repl session"))
		.subcommand(
			Command::new("run")
				.about("Execute a file")
				.arg(arg!(<file> "File to execute")),
		);

	let help_str = command.render_help();
	let global_matches = command.get_matches();
	let stages = global_matches.get_flag("stages");

	match global_matches.subcommand() {
		Some(("repl", _)) => cmd_repl(stages)?,
		Some(("run", sub_matches)) => cmd_run(
			stages,
			sub_matches.get_one::<String>("file").unwrap().into(),
		)?,
		_ => print!("{}", help_str.ansi()),
	}

	Ok(())
}

fn cmd_repl(stages: bool) -> anyhow::Result<()> {
	let mut editor: Editor<(), _> = Editor::new()?;
	if editor.load_history(".history").is_err() {
		println!("No previous history.");
	}

	loop {
		let readline = editor.readline("➤ ");
		match readline {
			Ok(line) => {
				editor.add_history_entry(line.as_str()).unwrap();
				let mut diag = Diagnostics::default();
				compile(&mut diag, line);
				if stages {
					diag.save_stages()?;
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

fn cmd_run(stages: bool, path: PathBuf) -> anyhow::Result<()> {
	let src = read_to_string(&path)?;
	let mut diag = Diagnostics::new(path);
	compile(&mut diag, src);
	if stages {
		diag.save_stages()?;
	}
	Ok(())
}
