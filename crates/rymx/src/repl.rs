use crate::log;
use rym_ast::Parser;
use rym_tree_walk::Interpreter;
use tokenize::Lexer;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn exec() -> rustyline::Result<()> {
	Repl::new()?.watch();
	Ok(())
}

struct Repl {
	// interpreter: todo,
	editor: Editor<()>,
}

impl Repl {
	fn new() -> rustyline::Result<Self> {
		let mut editor = Editor::new()?;
		if editor.load_history(".history").is_err() {
			println!("No previous history.");
		}
		Ok(Self { editor })
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
		let correct_syntax = errors.is_empty();
		log::title("Lexer", correct_syntax);
		log::tokens(&tokens);
		log::errors(&errors);

		let (ast, errors) = Parser::parse(tokens);
		let correct_syntax = errors.is_empty();
		log::title("Parser", correct_syntax);
		log::ast(&ast);
		log::errors(&errors);

		if !correct_syntax {
			return;
		}

		log::title("Interpreter", true);
		if let Err(error) = Interpreter::new().eval(&ast) {
			println!("{error:?}");
		}
	}
}
