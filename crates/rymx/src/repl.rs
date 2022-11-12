use crate::log;
use colored::Colorize;
use lex::Lexer;
use parse::Parser;
use tree_walk::Interpreter;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn exec() -> rustyline::Result<()> {
	Repl::new()?.watch();
	Ok(())
}

struct Repl {
	interpreter: Interpreter,
	editor: Editor<()>,
}

impl Repl {
	fn new() -> rustyline::Result<Self> {
		let mut editor = Editor::new()?;
		if editor.load_history(".history").is_err() {
			println!("No previous history.");
		}
		Ok(Self {
			interpreter: Interpreter::default(),
			editor,
		})
	}

	fn watch(mut self) {
		loop {
			let readline = self.editor.readline(" ➤ ");
			match readline {
				Ok(line) => {
					self.editor.add_history_entry(line.as_str());
					self.eval_line(line);
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
		self.editor.save_history(".history").unwrap();
	}

	fn eval_line(&mut self, line: String) {
		let (tokens, errors) = Lexer::lex(&line);
		log::block("Lexer", || {
			log::tokens(&tokens);
			log::errors(&errors);
			errors.is_empty()
		});

		let (ast, errors) = Parser::parse(tokens);
		let correct_syntax = errors.is_empty();
		log::block("Parser", || {
			log::ast(&ast);
			log::errors(&errors);
			errors.is_empty()
		});

		if !correct_syntax {
			return;
		}

		log::block("Interpreter", || {
			if let Err(error) = self.interpreter.eval(&ast) {
				println!("{} {error:?}", "Error:".red().bold());
				false
			} else {
				true
			}
		});
	}
}
