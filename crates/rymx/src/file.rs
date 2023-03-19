use std::path::Path;
use std::process::exit;

// use ast::Spanned;
use rym_errors::DiagnosticHandler;
use rym_lexer::rich::Lexer;
use rym_parser::parse_expr;
// use tree_walk::Interpreter;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let src = std::fs::read_to_string(path)?;

	let handler = DiagnosticHandler::default();
	let tokens: Vec<_> = Lexer::new(&src).collect();
	let rym_parser::ParseResult { ast, .. } = parse_expr(&tokens);

	for diagnostic in handler.collect().into_iter() {
		println!("{diagnostic:?}");
	}

	let Some(_ast) = ast else {
		exit(65 ) // Data format error
	};

	// log::block("Interpreter", || {
	// 	if let Err(error) = Interpreter::default().eval(&[Spanned(0..0, ast::Stmt::Expr(expr))]) {
	// 		println!("{error:?}");
	// 		exit(1 /* Failure */)
	// 	} else {
	// 		true
	// 	}
	// });

	Ok(())
}
