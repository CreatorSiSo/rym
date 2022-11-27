use std::fmt::Debug;

use rym_span::Span;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream(pub Vec<TokenTree>);

impl TokenStream {
	pub fn iter(&self) -> Iter<'_> {
		Iter(self.0.iter())
	}
}

impl FromIterator<TokenTree> for TokenStream {
	fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl From<Vec<TokenTree>> for TokenStream {
	fn from(vec: Vec<TokenTree>) -> Self {
		TokenStream(vec)
	}
}

pub trait TokenStreamIter: Debug {
	fn peek_unfiltered(&self) -> Option<&TokenTree>;
	fn peek(&self) -> Option<&TokenTree>;

	fn is_next_newline(&self) -> bool {
		matches!(self.peek_unfiltered(), Some(tt) if tt.is_newline())
	}
}

#[derive(Debug, Clone)]
pub struct Iter<'a>(std::slice::Iter<'a, TokenTree>);

impl TokenStreamIter for Iter<'_> {
	fn peek_unfiltered(&self) -> Option<&TokenTree> {
		self.0.clone().next()
	}

	fn peek(&self) -> Option<&TokenTree> {
		self.clone().next()
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a TokenTree;

	fn next(&mut self) -> Option<Self::Item> {
		for tt in self.0.by_ref() {
			match tt {
				_ if tt.is_newline() => continue,
				_ => return Some(tt),
			}
		}
		None
	}
}

impl IntoIterator for TokenStream {
	type Item = TokenTree;
	type IntoIter = IntoIter;

	fn into_iter(mut self) -> Self::IntoIter {
		// Reverse so we can use pop of the "first" token tree in the iterator
		self.0.reverse();
		IntoIter(self.0)
	}
}

#[derive(Debug)]
pub struct IntoIter(Vec<TokenTree>);

impl TokenStreamIter for IntoIter {
	fn peek(&self) -> Option<&TokenTree> {
		self.0.iter().filter(|tt| !tt.is_newline()).last()
	}

	fn peek_unfiltered(&self) -> Option<&TokenTree> {
		self.0.iter().last()
	}
}

impl Iterator for IntoIter {
	type Item = TokenTree;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(tt) = self.0.pop() {
			match tt {
				_ if tt.is_newline() => continue,
				_ => return Some(tt),
			}
		}
		None
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
	/// Single token.
	Token(Token),
	/// Delimited sequence of token trees.
	Delimited(DelimSpan, Delimiter, TokenStream),
}

impl TokenTree {
	fn is_newline(&self) -> bool {
		matches!(self, TokenTree::Token(Token { kind: TokenKind::Newline, .. }))
	}
}

#[cfg(test)]
#[test]
fn token_stream() {
	let tt_1 = TokenTree::Token(Token { kind: TokenKind::And, span: Span::new(1, 2) });
	let token_stream = TokenStream(vec![
		TokenTree::Token(Token { kind: TokenKind::Newline, span: Span::new(0, 1) }),
		tt_1.clone(),
	]);

	let mut iter = token_stream.iter();
	let mut into_iter = token_stream.clone().into_iter();

	assert_eq!(iter.peek(), Some(&tt_1));
	assert_eq!(into_iter.peek(), Some(&tt_1));

	assert_eq!(iter.is_next_newline(), true);
	assert_eq!(into_iter.is_next_newline(), true);

	assert_eq!(iter.next(), Some(&tt_1));
	assert_eq!(into_iter.next(), Some(tt_1));

	assert_eq!(iter.is_next_newline(), false);
	assert_eq!(into_iter.is_next_newline(), false);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
}

impl Token {
	pub const fn new(kind: TokenKind, span: Span) -> Self {
		Self { kind, span }
	}
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TokenKind {
	/// `\n`
	Newline,

	// Punctuation token.
	/// `;`
	Semi,
	/// `:`
	Colon,
	/// `::`
	ColonColon,
	/// `,`
	Comma,
	/// `.`
	Dot,

	// Operator like token.
	/// `|`
	Or,
	/// `||`
	OrOr,
	/// `&`
	And,
	/// `&&`
	AndAnd,
	/// `+`
	Plus,
	/// `+=`
	PlusEq,
	/// `-`
	Minus,
	/// `-=`
	MinusEq,
	/// `*`
	Star,
	/// `*=`
	StarEq,
	/// `/`
	Slash,
	/// `/=`
	SlashEq,
	/// `%`
	Percent,
	/// `%=`
	PercentEq,
	/// `!`
	Bang,
	/// `!=`
	BangEq,
	/// `=`
	Eq,
	/// `==`
	EqEq,
	/// `<`
	LessThan,
	/// `<=`
	LessThanEq,
	/// `>`
	GreaterThan,
	/// `>=`
	GreaterThanEq,

	/// Delimiter token.
	OpenDelim(Delimiter),
	CloseDelim(Delimiter),

	/// Indentifier token: `some_thing`, `test`
	Ident(SmolStr),

	/// Literal token: `"Hello World!"`, `'\n'`, `36_254`, `0.2346`
	Literal(LitKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
	/// `( .. )`
	Paren,
	/// `{ .. }`
	Brace,
	/// `[ .. ]`
	Bracket,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelimSpan {
	pub open: Span,
	pub close: Span,
	pub entire: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LitKind {
	Integer(i64),
	Float(f64),
	Char(char),
	String(String),
}

impl From<i64> for LitKind {
	fn from(int: i64) -> Self {
		LitKind::Integer(int)
	}
}

impl From<f64> for LitKind {
	fn from(float: f64) -> Self {
		LitKind::Float(float)
	}
}

impl From<String> for LitKind {
	fn from(string: String) -> Self {
		LitKind::String(string)
	}
}

impl From<char> for LitKind {
	fn from(char: char) -> Self {
		LitKind::Char(char)
	}
}
