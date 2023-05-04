use std::fs::read_to_string;

use clap::{arg, command, ArgMatches, Command};
use rustyline::{error::ReadlineError, Editor};
use rym_ast::{mut_visitor, visitor};
use rym_ast_passes::{NodeCounter, ToCPass, ValidationPass};
use rym_lexer::rich::Lexer;
use rym_parser::{parse_script_file, ParseResult};
use stringx::Join;
use termsize::Size;

mod display;
use display::render_box;

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
		Some(("run", matches)) => eval_src(read_to_string(matches.get_one::<String>("file").unwrap())?),
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
				eval_src(line);
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

fn eval_src(src: String) {
	let tokens: Vec<_> = Lexer::new(&src).collect();
	let ParseResult { ast, errors } = parse_script_file(&tokens);

	// TODO Use ✓ and ✗ for successful / unsuccessful stages
	// TODO Add colored edges

	let Size { cols, .. } = termsize::get().unwrap_or(Size {
		rows: u16::MAX,
		cols: 80,
	});
	let cols = cols.min(120) as usize;

	{
		#[cfg(feature = "expand")]
		let ast_str = format!("{ast:#?}");
		#[cfg(not(feature = "expand"))]
		let ast_str = format!("{ast:?}");

		println!("{}", render_box(cols, "Ast", &ast_str));
	}

	if !errors.is_empty() {
		#[cfg(feature = "expand")]
		let err_str = format!("{errors:#?}");
		#[cfg(not(feature = "expand"))]
		let err_str = format!("{errors:?}");

		println!("{}", render_box(cols, "Errors", &err_str));
	}

	let mut ast = ast;
	if let Some(stmts) = &mut ast {
		let mut counter = NodeCounter;

		let counts = visitor::walk_stmts(&mut counter, stmts).join(", ");
		println!("{}", render_box(cols, "Node Counts", &counts));

		let mut pass = ValidationPass::new();
		mut_visitor::walk_stmts(&mut pass, stmts);
		println!(
			"{}",
			render_box(cols, "Simplified Ast", &format!("{:?}", stmts))
		);

		let counts = visitor::walk_stmts(&mut counter, stmts).join(", ");
		println!("{}", render_box(cols, "Node Counts", &counts));

		let mut pass = ToCPass::new();
		visitor::walk_stmts(&mut pass, stmts).for_each(drop);

		let wrapped_c_src = format!(
			"int main() {{\n{}\n}}",
			pass
				.out
				.lines()
				.map(|line| String::from("  ") + line)
				.join("\n")
		);
		// println!("{}", render_box(cols, "C Output", &wrapped_c_src));
		println!("{}", &wrapped_c_src);
	}
}
