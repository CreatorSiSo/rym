/// TODO: Implement other escape codes
///
/// Maybe use the rust_lexer module?
/// https://doc.rust-lang.org/stable/nightly-rustc/rustc_lexer/unescape/
/// https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/unescape.rs.html
///
/// Or snailquote crate?
/// https://docs.rs/snailquote/latest/snailquote
pub(super) fn unescape(slice: &str) -> String {
	let mut iter = slice.chars();
	let mut result = String::new();

	while let Some(c) = iter.next() {
		if c == '\\' {
			match iter.clone().next() {
				Some('t') => {
					iter.next();
					result.push('\t');
				}
				Some('n') => {
					iter.next();
					result.push('\n');
				}
				_ => result.push('\\'),
			}
		} else {
			result.push(c)
		}
	}
	result
}
