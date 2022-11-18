use rym_errors::annotate::{Annotation, AnnotationType, DisplayList, Slice, Snippet};
use rym_span::Span;
use stringx::Join;

fn main() {
	let src_1 =
		include_str!("../../tests/src/integration/lex_invalid.rym").lines().skip(2).join("\n");
	const N: i32 = -14;

	let snippet = Snippet::new()
		.title(Annotation::new(AnnotationType::Error).id("E4320").label("Unexpected character"))
		.add_slice(
			Slice { source: &src_1, line_start: 3 }
				.origin("crates/tests/src/integration/lex_invalid.rym")
				.annotation(Span::new(17, 18).move_by(N), AnnotationType::Error, "invalid")
				.annotation(Span::new(40, 46).move_by(N), AnnotationType::Error, "invalid"),
		)
		.add_slice(
			Slice { source: "Ident::Test", line_start: 129 }.origin("src/display.rs").annotation(
				(0, 11),
				AnnotationType::Warning,
				"Unused",
			),
		);

	let dl = DisplayList::from(snippet);
	println!("\n{}", dl.to_string());
}
