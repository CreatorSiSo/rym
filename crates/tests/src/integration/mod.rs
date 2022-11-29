use super::*;

mod sources;

#[derive(Debug)]
struct Should {
	fail_lex: bool,
	fail_parse: bool,
	fail_exec: bool,
}

impl Should {
	fn succeed() -> Self {
		Self { fail_lex: false, fail_parse: false, fail_exec: false }
	}
}

// FIXME #[test]
fn exec() {
	for (path, src) in sources::SOURCES {
		let path = path;
		let should = if src.starts_with("//! fail") {
			let flags: Vec<&str> = src[8..].lines().next().unwrap().split_whitespace().collect();
			Should {
				fail_lex: flags.contains(&"lex"),
				fail_parse: flags.contains(&"parse"),
				fail_exec: flags.contains(&"exec"),
			}
		} else {
			Should::succeed()
		};

		let (tokens, lex_errors) = tokens_from_src(&src);
		let (ast, parse_errors) = ast_from_src(tokens);

		let exec_result = Interpreter::default().eval(&ast);

		match should {
			Should { fail_lex, fail_parse, fail_exec } => {
				if fail_lex {
					if lex_errors.is_empty() {
						panic!("Expected lex errors got none, {path}");
					}
				} else {
					if !lex_errors.is_empty() {
						panic!("Expected no lex errors got `{lex_errors:?}`, {path}");
					}
				}
				if fail_parse {
					if parse_errors.is_empty() {
						panic!("Expected parse errors got none, {path}");
					}
				} else {
					if !parse_errors.is_empty() {
						panic!("Expected no parse errors got `{parse_errors:?}`, {path}");
					}
				}
				if fail_exec {
					if let Ok(()) = exec_result {
						panic!("Expected runtime error got nothing, {path}");
					}
				} else {
					if let Err(err) = exec_result {
						panic!("Expected no runtime errors got `{err:?}`, {path}");
					}
				}
			}
		};
	}
}
