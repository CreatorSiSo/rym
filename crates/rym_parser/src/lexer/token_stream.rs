use rym_errors::{Diagnostic, Level, RymResult};
use rym_span::Span;
use smol_str::SmolStr;
use std::fmt::Debug;
use stringx::Join;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct TokenStream {
	tokens: Vec<Token>,
	previous: Option<TokenKind>,
}

impl TokenStream {
	pub fn new(mut tokens: Vec<Token>) -> Self {
		tokens.reverse();
		Self::new_reversed(tokens)
	}

	// Assumes that tokens is already reversed
	const fn new_reversed(tokens: Vec<Token>) -> Self {
		Self { tokens, previous: None }
	}

	/// Consume next token if it matches one of the token kinds in the pattern
	pub fn expect<P: Pattern>(&mut self, pattern: P) -> RymResult<Token> {
		// Only process newline tokens if we are expecting them
		let process_newlines = pattern.contains_newline();
		let Some(got) = self.peek(process_newlines) else {
			return Err(Diagnostic::new(Level::Error, format!("Expected `{}`", pattern.joined_str(" | "))));
		};

		if pattern.matches(&got.kind) {
			return Ok(
				// SAFETY: Next token must exist because self.peek() is Some(..)
				if process_newlines {
					unsafe { self.next_unfiltered().unwrap_unchecked() }
				} else {
					unsafe { self.next().unwrap_unchecked() }
				},
			);
		}

		Err(Diagnostic::new(
			Level::Error,
			format!("Expected `{}` got `{:?}`", pattern.joined_str(" | "), got.kind),
		))
	}

	pub fn peek(&self, process_newline: bool) -> Option<&Token> {
		if process_newline {
			self.tokens.iter().last()
		} else {
			self.tokens.iter().filter(|token| !token.is_newline()).last()
		}
	}

	pub fn is_next_newline(&self) -> bool {
		matches!(self.peek(true), Some(token) if token.is_newline())
	}

	fn next_unfiltered(&mut self) -> Option<Token> {
		self.tokens.pop()
	}
}

impl FromIterator<Token> for TokenStream {
	fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
		Self::new(iter.into_iter().collect())
	}
}

impl From<Vec<Token>> for TokenStream {
	fn from(tokens: Vec<Token>) -> Self {
		TokenStream::new(tokens)
	}
}

impl Iterator for TokenStream {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(token) = self.tokens.pop() {
			match token {
				_ if token.is_newline() => continue,
				_ => return Some(token),
			}
		}
		None
	}
}

#[cfg(test)]
#[test]
fn token_stream() {
	let mut token_stream = TokenStream::new(vec![
		Token { kind: TokenKind::Newline, span: Span::new(0, 1) },
		Token { kind: TokenKind::And, span: Span::new(1, 2) },
		Token { kind: TokenKind::Ident(SmolStr::new("test")), span: Span::new(2, 6) },
	]);

	assert_eq!(
		token_stream.peek(false),
		Some(&Token { kind: TokenKind::And, span: Span::new(1, 2) })
	);

	assert_eq!(token_stream.is_next_newline(), true);
	assert_eq!(token_stream.next(), Some(Token { kind: TokenKind::And, span: Span::new(1, 2) }));
	assert_eq!(token_stream.is_next_newline(), false);
	assert_eq!(
		token_stream.expect(Tk::Ident),
		Ok(Token { kind: TokenKind::Ident(SmolStr::new("test")), span: Span::new(2, 6) })
	);
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

	pub const fn is_newline(&self) -> bool {
		matches!(self.kind, TokenKind::Newline)
	}
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TokenKind {
	/// `\n`
	Newline,

	// Punctuation
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

	// Operator like
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

	// Keywords
	/// `const`
	Const,
	/// `mut`
	Mut,
	/// `fn`
	Fn,
	/// `if`
	If,
	/// `else`
	Else,
	/// `loop`
	Loop,
	/// `while`
	While,
	/// `for`
	For,

	/// Delimiter token.
	OpenDelim(Delimiter),
	CloseDelim(Delimiter),

	/// Indentifier token: `some_thing`, `test`
	Ident(SmolStr),

	/// Literal token: `"Hello World!"`, `'\n'`, `36_254`, `0.2346`
	Literal(LitKind),
}

pub const KEYWORDS_MAP: ([&str; 8], [TokenKind; 8]) = (
	["const", "mut", "fn", "if", "else", "loop", "while", "for"],
	[
		TokenKind::Const,
		TokenKind::Mut,
		TokenKind::Fn,
		TokenKind::If,
		TokenKind::Else,
		TokenKind::Loop,
		TokenKind::While,
		TokenKind::For,
	],
);

pub trait Pattern {
	fn joined_str(&self, sep: &str) -> String;
	fn matches(&self, other: &TokenKind) -> bool;
	fn contains_newline(&self) -> bool;
}

impl<const N: usize> Pattern for &[Tk; N] {
	fn joined_str(&self, sep: &str) -> String {
		self.iter().join_format(sep, |kind| format!("{kind:?}"))
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self.iter().any(|kind| kind.matches(other))
	}

	fn contains_newline(&self) -> bool {
		self.contains(&Tk::Newline)
	}
}

impl Pattern for Tk {
	fn joined_str(&self, _: &str) -> String {
		format!("{self:?}")
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self.matches(other)
	}

	fn contains_newline(&self) -> bool {
		self == &Tk::Newline
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tk {
	Newline,

	// Punctuation
	Semi,
	Colon,
	ColonColon,
	Comma,
	Dot,

	// Operator like
	Or,
	OrOr,
	And,
	AndAnd,
	Plus,
	PlusEq,
	Minus,
	MinusEq,
	Star,
	StarEq,
	Slash,
	SlashEq,
	Percent,
	PercentEq,
	Bang,
	BangEq,
	Eq,
	EqEq,
	LessThan,
	LessThanEq,
	GreaterThan,
	GreaterThanEq,

	// Keywords
	Const,
	Mut,
	Fn,
	If,
	Else,
	Loop,
	While,
	For,

	OpenDelim(Delimiter),
	CloseDelim(Delimiter),

	Ident,

	LiteralString,
	LiteralChar,
	LiteralInt,
	LiteralFloat,
}

impl Tk {
	const fn matches(&self, other: &TokenKind) -> bool {
		match (other, self) {
			(TokenKind::Newline, Tk::Newline) => true,

			// Punctuation
			(TokenKind::Semi, Tk::Semi)
			| (TokenKind::Colon, Tk::Colon)
			| (TokenKind::ColonColon, Tk::ColonColon)
			| (TokenKind::Comma, Tk::Comma)
			| (TokenKind::Dot, Tk::Dot) => true,

			// Operator like
			(TokenKind::Or, Tk::Or)
			| (TokenKind::OrOr, Tk::OrOr)
			| (TokenKind::And, Tk::And)
			| (TokenKind::AndAnd, Tk::AndAnd)
			| (TokenKind::Plus, Tk::Plus)
			| (TokenKind::PlusEq, Tk::PlusEq)
			| (TokenKind::Minus, Tk::Minus)
			| (TokenKind::MinusEq, Tk::MinusEq)
			| (TokenKind::Star, Tk::Star)
			| (TokenKind::StarEq, Tk::StarEq)
			| (TokenKind::Slash, Tk::Slash)
			| (TokenKind::SlashEq, Tk::SlashEq)
			| (TokenKind::Percent, Tk::Percent)
			| (TokenKind::PercentEq, Tk::PercentEq)
			| (TokenKind::Bang, Tk::Bang)
			| (TokenKind::BangEq, Tk::BangEq)
			| (TokenKind::Eq, Tk::Eq)
			| (TokenKind::EqEq, Tk::EqEq)
			| (TokenKind::LessThan, Tk::LessThan)
			| (TokenKind::LessThanEq, Tk::LessThanEq)
			| (TokenKind::GreaterThan, Tk::GreaterThan)
			| (TokenKind::GreaterThanEq, Tk::GreaterThanEq) => true,

			// Keywords
			(TokenKind::Const, Tk::Const)
			| (TokenKind::Mut, Tk::Mut)
			| (TokenKind::Fn, Tk::Fn)
			| (TokenKind::If, Tk::If)
			| (TokenKind::Else, Tk::Else)
			| (TokenKind::Loop, Tk::Loop)
			| (TokenKind::While, Tk::While)
			| (TokenKind::For, Tk::For) => true,

			(TokenKind::OpenDelim(delim), Tk::OpenDelim(m_delim)) => matches!(
				(delim, m_delim),
				(Delimiter::Paren, Delimiter::Paren)
					| (Delimiter::Brace, Delimiter::Brace)
					| (Delimiter::Bracket, Delimiter::Bracket)
			),

			(TokenKind::Ident(..), Tk::Ident) => true,

			(TokenKind::Literal(lit), kind_match) => matches!(
				(lit, kind_match),
				(LitKind::Int(_), Tk::LiteralInt)
					| (LitKind::Float(_), Tk::LiteralFloat)
					| (LitKind::Char(_), Tk::LiteralChar)
					| (LitKind::String(_), Tk::LiteralString)
			),

			_ => false,
		}
	}
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
	Int(i64),
	Float(f64),
	Char(char),
	String(String),
}

impl From<i64> for LitKind {
	fn from(int: i64) -> Self {
		LitKind::Int(int)
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
