use rym_span::Span;
use smol_str::SmolStr;

/// ```
/// # use rym_tt::*;
/// # use rym_span::*;
///
/// let tt = TokenTree::Token(Token { kind: TokenKind::And, span: Span::new(2, 3) });
/// let token_stream =
/// TokenStream(vec![WrappedTt::Newline, WrappedTt::Tt(tt.clone())]);
///
/// let mut iter = token_stream.iter();
/// let mut into_iter = token_stream.clone().into_iter();
///
/// assert_eq!(iter.peek(), Some(&tt));
/// assert_eq!(into_iter.peek(), Some(&tt));
///
/// assert_eq!(iter.peek_wrapped(), Some(&WrappedTt::Newline));
/// assert_eq!(into_iter.peek_wrapped(), Some(&WrappedTt::Newline));
///
/// assert_eq!(iter.is_next_newline(), true);
/// assert_eq!(into_iter.is_next_newline(), true);
///
/// assert_eq!(iter.next(), Some(&tt));
/// assert_eq!(into_iter.next(), Some(tt));
///
/// assert_eq!(iter.is_next_newline(), false);
/// assert_eq!(into_iter.is_next_newline(), false);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream(pub Vec<WrappedTt>);

impl TokenStream {
	pub fn iter(&self) -> Iter<'_> {
		Iter(self.0.iter())
	}
}

impl FromIterator<WrappedTt> for TokenStream {
	fn from_iter<T: IntoIterator<Item = WrappedTt>>(iter: T) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl From<Vec<WrappedTt>> for TokenStream {
	fn from(vec: Vec<WrappedTt>) -> Self {
		TokenStream(vec)
	}
}

impl From<Vec<TokenTree>> for TokenStream {
	fn from(vec: Vec<TokenTree>) -> Self {
		TokenStream(vec.into_iter().map(WrappedTt::Tt).collect())
	}
}

pub trait TokenStreamIter {
	fn peek(&self) -> Option<&TokenTree>;
	fn peek_wrapped(&self) -> Option<&WrappedTt>;

	fn is_next_newline(&self) -> bool {
		matches!(self.peek_wrapped(), Some(WrappedTt::Newline))
	}
}

#[derive(Debug, Clone)]
pub struct Iter<'a>(std::slice::Iter<'a, WrappedTt>);

impl TokenStreamIter for Iter<'_> {
	fn peek(&self) -> Option<&TokenTree> {
		self.clone().next()
	}

	fn peek_wrapped(&self) -> Option<&WrappedTt> {
		self.0.clone().next()
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a TokenTree;

	fn next(&mut self) -> Option<Self::Item> {
		for next in self.0.by_ref() {
			match next {
				WrappedTt::Newline => continue,
				WrappedTt::Tt(tt) => return Some(tt),
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
pub struct IntoIter(Vec<WrappedTt>);

impl TokenStreamIter for IntoIter {
	fn peek(&self) -> Option<&TokenTree> {
		for wrapped in self.0.iter() {
			match wrapped {
				WrappedTt::Tt(tt) => return Some(tt),
				_ => continue,
			}
		}
		None
	}

	fn peek_wrapped(&self) -> Option<&WrappedTt> {
		// Internal vec is reversed, so we get the last element
		self.0.last()
	}
}

impl Iterator for IntoIter {
	type Item = TokenTree;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(next) = self.0.pop() {
			match next {
				WrappedTt::Newline => continue,
				WrappedTt::Tt(tt) => return Some(tt),
			}
		}
		None
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum WrappedTt {
	Newline,
	Tt(TokenTree),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
	/// Single token.
	Token(Token),
	/// Delimited sequence of token trees.
	Delimited(DelimSpan, Delimiter, TokenStream),
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
pub enum TokenKind {
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
