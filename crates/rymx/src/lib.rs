use std::fmt::Write;

mod ast;
mod compile;
mod diagnostics;
mod interpret;
mod parse;
mod span;
pub mod std_lib;
mod tokenize;

pub use diagnostics::{Diagnostics, SourceId};
pub use interpret::Env;
pub use span::Span;
pub use tokenize::tokenizer;

use ast::{Expr, Module};
use interpret::{Interpret, Value};
use span::SourceSpan;
use tokenize::Token;

pub fn interpret(diag: &mut Diagnostics, env: &mut Env, ast: impl Interpret) -> Value {
	diag.start_stage("interpret");
	// TODO does this make sense
	// Ignoring control flow
	let result = ast.eval(env).inner();

	let env_state: String = env
		.variables()
		.into_iter()
		.fold((0, String::new()), |(indent, mut accum), scope| {
			for (name, (kind, value)) in scope {
				writeln!(accum, "{}{kind} {name} = {}", "  ".repeat(indent), value)
					.expect("Internal Error: Unable to write into String");
			}
			(indent + 1, accum)
		})
		.1;
	diag.push_result(&env_state);

	result
}

pub fn compile_module(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Option<Module> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src, src_id.clone())?;

	diag.start_stage("parse");
	let module = match parse::parse_file(&tokens, src, src_id) {
		Ok(module) => module,
		Err(reports) => {
			for report in reports {
				diag.push_report(report);
			}
			return None;
		}
	};
	diag.push_result(&format!("{module:#?}\n"));

	// TODO Name resolution
	// TODO Typechecking
	// TODO Const evaluation
	// TODO Generate intermediate representation

	// i / / / ;

	Some(module)
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_expr(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Option<Expr> {
	diag.start_stage("tokenize");
	let tokens: Vec<(Token, Span)> = tokenize(diag, src, src_id.clone())?;

	diag.start_stage("parse");
	let expr = match parse::parse_expr(&tokens, src, src_id) {
		Ok(expr) => expr,
		Err(reports) => {
			for report in reports {
				diag.push_report(report);
			}
			return None;
		}
	};
	diag.push_result(&format!("{expr:#?}\n"));

	Some(expr)
}

fn tokenize(diag: &mut Diagnostics, src: &str, src_id: SourceId) -> Option<Vec<(Token, Span)>> {
	let results: Vec<_> = tokenizer(src).collect();
	diag.push_result(
		&results.iter().fold(String::new(), |accum, (result, span)| {
			let (token, span) = match result {
				Ok(token) => (format!("{token:?}"), span),
				Err(_) => ("Error".into(), span),
			};
			accum + &format!("{token} [{}]\n", span.src(src).escape_debug())
		}),
	);

	let mut tokens = vec![];
	for (result, span) in results {
		let Ok(token) = result else {
			diag.error(
				format!("Invalid character [{}]", span.src(src)),
				SourceSpan(src_id, span),
			);
			return None;
		};
		match token {
			Token::DocComment | Token::Comment | Token::VSpace | Token::HSpace => continue,
			_ => tokens.push((token, span)),
		}
	}
	Some(tokens)
}
