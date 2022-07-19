#[derive(Debug)]
pub struct LexerError {
	pub msg: String,
	pub line: usize,
	pub col: usize,
}

impl LexerError {
	pub fn err<T>(msg: String, line: usize, col: usize) -> Result<T, Self> {
		Err(Self { msg, line, col })
	}
}
