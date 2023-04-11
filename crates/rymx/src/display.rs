use std::{
	borrow::Cow,
	collections::VecDeque,
	ops::{AddAssign, Range},
	str::from_utf8,
};
use stringx::Join;

type Str<'a> = Cow<'a, str>;

struct Lines<'a> {
	lines: Vec<VecDeque<Str<'a>>>,
}

impl<'a> Lines<'a> {
	fn new() -> Self {
		Self { lines: vec![] }
	}

	fn push_line<const N: usize>(&mut self, content: [Str<'a>; N]) {
		self.lines.push(VecDeque::from(content))
	}

	fn extend<T, I>(&mut self, other: T)
	where
		T: IntoIterator<Item = I>,
		I: Into<VecDeque<Str<'a>>>,
	{
		self.lines.extend(other.into_iter().map(|line| line.into()))
	}

	fn len(&self) -> usize {
		self.lines.len()
	}

	fn lines(&mut self, range: Range<usize>) -> &mut [VecDeque<Str<'a>>] {
		&mut self.lines[range]
	}

	fn render(&self) -> String {
		self
			.lines
			.iter()
			.join_format("\n", |line_segments| line_segments.iter().join(""))
	}
}

impl AddAssign<String> for Lines<'_> {
	fn add_assign(&mut self, rhs: String) {
		self.push_line([Cow::Owned(rhs)])
	}
}

impl<'a> AddAssign<&'a str> for Lines<'a> {
	fn add_assign(&mut self, rhs: &'a str) {
		self.push_line([Cow::Borrowed(rhs)])
	}
}

pub fn render_box(width: usize, title: &str, content: &str) -> String {
	let inner_width = width - 4;
	let mut lines = Lines::new();

	lines.push_line([
		Cow::Borrowed("╭╴"),
		Cow::Borrowed(title),
		Cow::Borrowed("╶"),
		Cow::Owned("─".repeat(width - 2 - 2 - title.len())),
		Cow::Borrowed("╮"),
	]);

	let content_lines = content
		.split("\n")
		.flat_map(|line| line.as_bytes().chunks(inner_width))
		.map(|chunk| {
			[
				Cow::Borrowed(from_utf8(chunk).unwrap()),
				Cow::Owned(" ".repeat(inner_width - chunk.len())),
			]
		});
	lines.extend(content_lines);

	lines.push_line([
		Cow::Borrowed("╰"),
		Cow::Owned("─".repeat(width - 2)),
		Cow::Borrowed("╯"),
	]);

	for line in lines.lines(1..(lines.len() - 1)) {
		line.push_front(Cow::Borrowed("│ "));
		line.push_back(Cow::Borrowed(" │"));
	}

	lines.render()
}
