use crate::TokenStream;
use rym_span::Span;
use smol_str::SmolStr;

#[derive(Debug, PartialEq)]
pub enum TokenTree {
	Group(Group),
	Punct(Punct),
	Ident(Ident),
	Literal(Literal),
}

/// A delimited token stream.
///
/// A `Group` internally contains a `TokenStream` which is surrounded by `Delimiter`s.
#[derive(Debug, PartialEq)]
pub struct Group {
	pub delimiter: Delimiter,
	pub stream: TokenStream,
	pub spam: DelimSpan,
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
	/// `( .. )`
	Paren,
	/// `{ .. }`
	Brace,
	/// `[ .. ]`
	Bracket,
}

#[derive(Debug, PartialEq)]
pub struct DelimSpan {
	pub open: Span,
	pub close: Span,
	pub entire: Span,
}

#[derive(Debug, PartialEq)]
pub struct Punct {
	pub char: char,
	pub joint: bool,
	pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Ident {
	pub name: String,
	pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
	pub kind: LitKind,
	pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum LitKind {
	Integer(i64),
	Float(f64),
	Char(char),
	String(SmolStr),
}

#[cfg(test)]
mod tests {
	use super::*;
}
