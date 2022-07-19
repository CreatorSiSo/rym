use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

pub fn run_repl() {
	let mut rl = Editor::<()>::new().unwrap();
	if rl.load_history(".history").is_err() {
		println!("No previous history.");
	}
	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
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
	rl.save_history(".history").unwrap();
}
