use crate::{Diagnostic, Level};
use annotate::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

fn level_to_annotation_type(level: &Level) -> AnnotationType {
	match level {
		Level::Error => AnnotationType::Error,
		Level::Warning => AnnotationType::Warning,
		Level::Note => AnnotationType::Note,
		Level::Help => AnnotationType::Help,
	}
}

fn index_to_line_col(source: &str, index: usize, tab_spaces: u8) -> (usize, usize) {
	let (_, after) = &source.split_at(index);
	let num_lines = Ord::max(1, after.lines().count());
	let num_cols = match after.lines().last() {
		Some(last_line) => last_line
			.chars()
			.map(|c| if c == '\t' { tab_spaces } else { 1 })
			.fold(1, |sum, len| sum + len as usize),
		None => 1,
	};
	(num_lines, num_cols)
}

#[test]
fn test_index_to_line_col() {
	let src = r#"if test == 0 {
	println("something")
} else {
	do_stuff()
}

{
	let test = 20
}


{
	..more stuff down here
}"#;
	assert_eq!(index_to_line_col(src, 2, 47), (4, 3));
	assert_eq!(index_to_line_col(src, 4, 106), (13, 26));
	assert_eq!(index_to_line_col(src, 4, 109), (14, 2));
}

pub fn diagnostic_to_snippet<'a>(
	diagnostic: &'a Diagnostic,
	origin: Option<&'a str>,
	source: Option<&'a str>,
) -> Snippet<'a> {
	let annotation_type = level_to_annotation_type(&diagnostic.level);
	let title = Annotation { id: diagnostic.code, label: Some(&diagnostic.title), annotation_type };
	let mut snippet = Snippet { title: Some(title), ..Default::default() };

	let Some(mut source) = source else {
		// No code snippet should be displayed
		return snippet;
	};

	// Just `^^^^^` below text inside span
	let annotations = diagnostic.primary_spans.iter().fold(vec![], |mut accum, span| {
		accum.push(SourceAnnotation { annotation_type, label: "", range: span.into() });
		accum
	});
	let annotations = diagnostic.span_labels.iter().fold(annotations, |mut accum, (span, label)| {
		accum.push(SourceAnnotation { annotation_type, label, range: span.into() });
		accum
	});

	let first_line = if let Some(first_span) = diagnostic.primary_spans.first() {
		// TODO                                    | tab_spaces should come from global config
		let line_start = index_to_line_col(source, first_span.start, 2).0;
		let start_index = source.lines().take(line_start - 1).fold(0, |len, line| len + line.len());
		source = &source[start_index..];
		line_start
	} else {
		1
	};
	snippet.slices.push(Slice { origin, source, annotations, line_start: first_line, fold: false });
	snippet
}
