use std::path::Path;
use std::process::exit;

// use crate::log;
// use ast::Spanned;
use rym_errors::DiagnosticHandler;
use rym_parser::parse_expr_from_src;
// use tree_walk::Interpreter;

pub fn exec<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let src = std::fs::read_to_string(path)?;

	let handler = DiagnosticHandler::default();
	let maybe_expr = parse_expr_from_src(&src, &handler);

	for diagnostic in handler.collect().into_iter() {
		println!("{diagnostic:?}");
	}

	let Ok(expr) = maybe_expr else {
		exit(65 /* Data format error */)
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
