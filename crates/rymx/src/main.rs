use clap::{arg, command, ArgMatches, Command};
use rustyline::{error::ReadlineError, Editor};
use rymx::Span;
use std::{fs::read_to_string, path::PathBuf};

mod parse;
mod tokenize;
use tokenize::tokenize;

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
		Some(("repl", matches)) => cmd_repl(matches)?,
		Some(("run", matches)) => cmd_run(matches.get_one::<String>("file").unwrap().into())?,
		_ => print!("{}", help_str.ansi()),
	}

	Ok(())
}

fn cmd_repl(_matches: &ArgMatches) -> anyhow::Result<()> {
	let mut editor: Editor<(), _> = Editor::new()?;
	if editor.load_history(".history").is_err() {
		println!("No previous history.");
	}

	loop {
		let readline = editor.readline("➤ ");
		match readline {
			Ok(line) => {
				editor.add_history_entry(line.as_str()).unwrap();
				compile(Diagnostics::default(), line);
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

fn cmd_run(path: PathBuf) -> anyhow::Result<()> {
	let src = read_to_string(&path)?;
	compile(Diagnostics::new(path), src);
	Ok(())
}

fn compile(mut diag: Diagnostics, src: String) {
	let (stage_data, maybe_tokens) = match tokenize(&src) {
		Ok(tokens) => (
			tokens
				.iter()
				.map(|token| token.debug_string(&src) + "\n")
				.collect(),
			Some(tokens),
		),
		Err(span @ Span { start, end }) => (
			format!("Error [{start}..{end}]: \"{}\"", &src[span.as_range()]),
			None,
		),
	};
	diag.debug_stage("tokenize", stage_data);
	let Some(tokens) = maybe_tokens else {
		return;
	};
}

#[derive(Debug, Default)]
struct Diagnostics {
	path: Option<PathBuf>,
	stages: Vec<(&'static str, String)>,
}

impl Diagnostics {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path: Some(path),
			stages: vec![],
		}
	}

	pub fn debug_stage(&mut self, stage: &'static str, data: String) {
		println!(">==> {stage} >==>");
		print!("{data}");
		println!("<==< {stage} <==<");

		self.stages.push((stage, data));
	}
}
