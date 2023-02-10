#![cfg(test)]

use super::*;
use crate::emitter::diagnostic_to_snippet;
use annotate::display_list::DisplayList;

#[track_caller]
fn assert_output(
	origin: Option<&str>,
	source: Option<&str>,
	diagnostics: &[Diagnostic],
	expected: &[&str],
) {
	let strings: Vec<String> = diagnostics
		.iter()
		.map(|diagnostic| {
			let snippet = diagnostic_to_snippet(diagnostic, origin, source);
			let dl = DisplayList::from(snippet).to_string();
			println!("{dl}");
			dl
		})
		.collect();
	assert_eq!(&strings, expected);
}

#[test]
fn no_src() {
	assert_output(
		None,
		None,
		&[
			Diagnostic::new(Level::Error, "Just testing"),
			Diagnostic::new(Level::Warning, "Just testing"),
			Diagnostic::new(Level::Note, "Just testing"),
			Diagnostic::new(Level::Help, "Just testing"),
		],
		&["error: Just testing", "warning: Just testing", "note: Just testing", "help: Just testing"],
	);
}

#[test]
fn src() {
	// assert_output(
	// 	Some("crates/tests/src/integration/control_flow.rym"),
	// 	Some(&include_str!("../../tests/src/integration/control_flow.rym")[0..103]),
	// 	&[Diagnostic::new_spanned(Level::Error, "Undeclared variable `say_hello`", Span::new(11, 20))],
	// 	&[&[
	// 		"error: Undeclared variable `say_hello`",
	// 		" --> crates/tests/src/integration/control_flow.rym:3:7",
	// 		"  |",
	// 		"1 | //!",
	// 		"2 | ",
	// 		"3 | const say_hello = false",
	// 		"  |       ^^^^^^^^^",
	// 		"4 | ",
	// 		"5 | if say_hello {",
	// 		"6 | \tprintln(\"Hello World!\")",
	// 		"7 | } else {",
	// 		"8 | \tprintln(\"Bye World!\")",
	// 		"9 | }",
	// 		"  |",
	// 	]
	// 	.join("\n")],
	// );
	assert_output(
		Some("some_where/some_path.rym"),
		Some(r#""not closed :("#),
		&[Diagnostic::new_spanned(Level::Error, "Unterminated string literal", Span::new(0, 14))
			.sub_diagnostic(Level::Note, None, "Missing trailing `\"` to terminate the string literal")],
		&[&[
			"error: Unterminated string literal",
			" --> some_where/some_path.rym:1:1",
			"  |",
			"1 | \"not closed :(",
			"  | ^^^^^^^^^^^^^^",
			"  |",
			"  = note: Missing trailing `\"` to terminate the string literal",
		]
		.join("\n")],
	);
}
