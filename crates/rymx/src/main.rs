use std::fs::read_to_string;

use clap::{arg, command, ArgMatches, Command};
use rustyline::{error::ReadlineError, Editor};

fn main() -> anyhow::Result<()> {
	let mut command = command!()
		.subcommand(Command::new("repl").about("Start a repl session"))
		.subcommand(
			Command::new("run")
				.about("Execute a file")
				.arg(arg!([file] "File to execute").required(true)),
		);

	let help_str = command.render_help();
	let matches = command.get_matches();

	match matches.subcommand() {
		Some(("repl", matches)) => repl(matches)?,
		Some(("run", matches)) => run(read_to_string(matches.get_one::<String>("file").unwrap())?),
		_ => print!("{}", help_str.ansi()),
	}

	Ok(())
}

fn repl(_matches: &ArgMatches) -> anyhow::Result<()> {
	let mut editor: Editor<(), _> = Editor::new()?;
	if editor.load_history(".history").is_err() {
		println!("No previous history.");
	}

	loop {
		let readline = editor.readline("➤ ");
		match readline {
			Ok(line) => {
				editor.add_history_entry(line.as_str()).unwrap();
				run(line);
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

fn run(path: String) {}
