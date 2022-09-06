use crate::debug::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rym_ast::{Lexer, Parser};
use rym_tree_walk::Interpreter;

pub fn exec() {
	Repl::new().watch();
}

struct Repl {
	// interpreter: todo,
	editor: Editor<()>,
}

impl Repl {
	fn new() -> Self {
		let mut editor = Editor::<()>::new().unwrap();
		if editor.load_history(".history").is_err() {
			println!("No previous history.");
		}
		Self { editor }
	}

	fn watch(mut self) {
		loop {
			let readline = self.editor.readline(">> ");
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
		println!("--- Lexer ---");
		let (tokens, errors) = Lexer::lex(&line);
		print_tokens(&tokens);
		print_errors(&errors);
		println!("\n");

		println!("--- Parser ---");
		let (ast, errors) = Parser::parse(tokens);
		print_ast(&ast);
		print_errors(&errors);
		println!("\n");

		println!("--- Interpreter ---");
		if let Err(error) = Interpreter::new().eval(&ast) {
			println!("{error:?}");
			return;
		}
		println!();
	}
}
