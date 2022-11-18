pub use annotate_snippets::display_list::DisplayList;
pub use annotate_snippets::display_list::FormatOptions;
pub use annotate_snippets::snippet::AnnotationType;

use annotate_snippets::snippet::SourceAnnotation;
use annotate_snippets::snippet::{
	Annotation as __Annotation, Slice as __Slice, Snippet as __Snippet,
};

pub struct Snippet<'a> {
	title: Option<__Annotation<'a>>,
	footer: Vec<__Annotation<'a>>,
	slices: Vec<__Slice<'a>>,
	opt: FormatOptions,
}

impl<'a> Snippet<'a> {
	pub const fn new() -> Self {
		Self {
			title: None,
			footer: Vec::new(),
			slices: Vec::new(),
			opt: FormatOptions { color: true, anonymized_line_numbers: false, margin: None },
		}
	}

	pub const fn title(mut self, title: Annotation<'a>) -> Self {
		self.title = Some(title.build());
		self
	}

	pub fn add_footer(mut self, footer: Annotation<'a>) -> Self {
		self.footer.push(footer.build());
		self
	}

	pub fn add_slice(mut self, slice: impl SliceBuild<'a>) -> Self {
		self.slices.push(slice.build());
		self
	}

	pub fn format(mut self, opt: FormatOptions) -> Self {
		self.opt = opt;
		self
	}

	pub fn build(self) -> __Snippet<'a> {
		__Snippet { title: self.title, footer: self.footer, slices: self.slices, opt: self.opt }
	}
}

impl<'a> From<Snippet<'a>> for DisplayList<'a> {
	fn from(snippet: Snippet<'a>) -> Self {
		DisplayList::from(snippet.build())
	}
}

pub struct Annotation<'a> {
	id: Option<&'a str>,
	label: Option<&'a str>,
	annotation_type: AnnotationType,
}

impl<'a> Annotation<'a> {
	pub const fn new(annotation_type: AnnotationType) -> Self {
		Self { id: None, label: None, annotation_type }
	}

	/// Identifier of the annotation. Usually error code like "E0308".
	pub const fn id(mut self, id: &'a str) -> Self {
		self.id = Some(id);
		self
	}

	pub const fn label(mut self, label: &'a str) -> Self {
		self.label = Some(label);
		self
	}

	pub const fn build(self) -> __Annotation<'a> {
		__Annotation { id: self.id, label: self.label, annotation_type: self.annotation_type }
	}
}

pub trait SliceBuild<'a> {
	fn build(self) -> __Slice<'a>;
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
///
/// One `Slice` is meant to represent a single, continuous,
/// slice of source code that you want to annotate.
#[derive(Debug)]
pub struct Slice<'a> {
	pub source: &'a str,
	pub line_start: usize,
}

impl<'a> Slice<'a> {
	pub const fn origin(self, origin: &'a str) -> SliceBuilder<'a> {
		let mut builder = SliceBuilder::new(self.source, self.line_start);
		builder.origin = Some(origin);
		builder
	}

	pub fn annotation(
		self,
		range: impl Into<(usize, usize)>,
		annotation_type: AnnotationType,
		label: &'a str,
	) -> SliceBuilder<'a> {
		SliceBuilder::new(self.source, self.line_start).annotation(range, annotation_type, label)
	}

	/// If set explicitly to `true`, the snippet will fold
	/// parts of the slice that don't contain any annotations.
	pub fn fold(self, fold: bool) -> SliceBuilder<'a> {
		SliceBuilder::new(self.source, self.line_start).fold(fold)
	}
}

impl<'a> SliceBuild<'a> for Slice<'a> {
	fn build(self) -> __Slice<'a> {
		SliceBuilder::new(self.source, self.line_start).build()
	}
}

pub struct SliceBuilder<'a> {
	source: &'a str,
	line_start: usize,
	origin: Option<&'a str>,
	annotations: Vec<SourceAnnotation<'a>>,
	fold: bool,
}

impl<'a> SliceBuilder<'a> {
	const fn new(source: &'a str, line_start: usize) -> Self {
		Self { source, line_start, origin: None, annotations: Vec::new(), fold: false }
	}

	pub fn annotation(
		mut self,
		range: impl Into<(usize, usize)>,
		annotation_type: AnnotationType,
		label: &'a str,
	) -> SliceBuilder<'a> {
		self.annotations.push(SourceAnnotation { annotation_type, range: range.into(), label });
		self
	}

	/// If set explicitly to `true`, the snippet will fold
	/// parts of the slice that don't contain any annotations.
	pub const fn fold(mut self, fold: bool) -> SliceBuilder<'a> {
		self.fold = fold;
		self
	}
}

impl<'a> SliceBuild<'a> for SliceBuilder<'a> {
	fn build(self) -> __Slice<'a> {
		__Slice {
			source: self.source,
			line_start: self.line_start,
			origin: self.origin,
			annotations: self.annotations,
			fold: self.fold,
		}
	}
}
