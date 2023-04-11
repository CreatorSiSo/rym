use rustyline::history::FileHistory;
use rym_ast::Visitor;
use rym_ast_passes::NodeCounter;
use rym_lexer::rich::Lexer;
use rym_parser::parse_expr;
// use tree_walk::Interpreter;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn exec() -> rustyline::Result<()> {
	Repl::new()?.watch();
	Ok(())
}

struct Repl {
	// interpreter: Interpreter,
	editor: Editor<(), FileHistory>,
}

impl Repl {
	fn new() -> rustyline::Result<Self> {
		let mut editor = Editor::new()?;
		if editor.load_history(".history").is_err() {
			println!("No previous history.");
		}
		Ok(Self {
			// interpreter: Interpreter::default(),
			editor,
		})
	}

	fn watch(mut self) {
		loop {
			let readline = self.editor.readline(" ➤ ");
			match readline {
				Ok(line) => {
					self.editor.add_history_entry(line.as_str()).unwrap();
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
		let tokens: Vec<_> = Lexer::new(&line).collect();
		let rym_parser::ParseResult { ast, errors } = parse_expr(&tokens);

		let correct_syntax = errors.is_empty();

		println!("--- Ast --- \n{ast:?}\n---");
		if !correct_syntax {
			println!("--- Errors --- \n{errors:?}\n---");
		}

		if let Some(expr) = ast {
			let mut counter = NodeCounter::new();
			counter.walk_expr(&expr);
			println!("--- {} ---", counter.count)
		}

		// log::block("Parser", || {
		// 	log::ast(&ast);
		// 	log::errors(&errors);
		// 	errors.is_empty()
		// });

		// if !correct_syntax {
		// 	return;
		// }

		// log::block("Interpreter", || {
		// 	if let Err(error) = self.interpreter.eval(&ast) {
		// 		println!("{} {error:?}", "Error:".red().bold());
		// 		false
		// 	} else {
		// 		true
		// 	}
		// });
	}
}
