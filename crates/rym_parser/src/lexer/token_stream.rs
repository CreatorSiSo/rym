use rym_errors::{Diagnostic, Level, RymResult};
use rym_span::{Span, DUMMY_SPAN};
use smol_str::SmolStr;
use std::fmt::Debug;
use stringx::Join;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct TokenStream {
	tokens: Vec<Token>,
	previous_span: Span,
}

impl TokenStream {
	pub fn new(mut tokens: Vec<Token>) -> Self {
		tokens.reverse();
		Self::new_reversed(tokens)
	}

	// Assumes that tokens is already reversed
	const fn new_reversed(tokens: Vec<Token>) -> Self {
		Self { tokens, previous_span: DUMMY_SPAN }
	}

	pub fn consume_while<P: Pattern>(&mut self, pattern: P) {
		self.tokens = self.skip_while(|token| token.matches(pattern.clone())).collect();
	}

	pub fn expect_ident(&mut self) -> RymResult<(SmolStr, Span)> {
		let Token { kind: TokenKind::Ident(name), span } = self.expect(Tk::Ident)? else {
			// SAFETY: TokenKind is checked to be Ident inside self.expect()
			unsafe { std::hint::unreachable_unchecked() }
		};
		Ok((name, span))
	}

	/// Consume next token if it matches one of the token kinds in the pattern
	/// otherwise return an error
	pub fn expect<P: Pattern>(&mut self, pattern: P) -> RymResult<Token> {
		match self.matches(pattern.clone()) {
			Some(token) => Ok(token),
			None => match self.peek(pattern.includes(&Tk::Newline)) {
				Some(got) => Err(Diagnostic::new_spanned(
					Level::Error,
					format!("Expected `{}` got `{:?}`", pattern.joined_str(" | "), got.kind),
					got.span,
				)),
				None => {
					Err(Diagnostic::new(Level::Error, format!("Expected `{}`", pattern.joined_str(" | "))))
				}
			},
		}
	}

	/// Consume next token if it matches one of the token kinds in the pattern
	pub fn matches<P: Pattern>(&mut self, pattern: P) -> Option<Token> {
		// Only process newline tokens if the pattern contains them
		let process_newlines = pattern.includes(&Tk::Newline);
		let Some(got) = self.peek(process_newlines) else {
			return if pattern.includes(&Tk::Eof) {
				Some(Token::new(TokenKind::Newline, DUMMY_SPAN))
			} else {
				None
			};
		};

		if pattern.matches(&got.kind) {
			return Some(
				// SAFETY: Next token must exist because self.peek() is Some(..)
				if process_newlines {
					unsafe { self.next_unfiltered().unwrap_unchecked() }
				} else {
					unsafe { self.next().unwrap_unchecked() }
				},
			);
		}

		None
	}

	pub fn peek(&self, process_newline: bool) -> Option<&Token> {
		if process_newline {
			self.tokens.iter().last()
		} else {
			self.tokens.iter().filter(|token| !token.is_newline()).last()
		}
	}

	pub fn previous_span(&self) -> Span {
		self.previous_span
	}

	pub fn is_next_newline(&self) -> bool {
		matches!(self.peek(true), Some(token) if token.is_newline())
	}

	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}

	fn next_unfiltered(&mut self) -> Option<Token> {
		self.tokens.pop().map(|token| {
			self.previous_span = token.span;
			token
		})
	}
}

impl Iterator for TokenStream {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(token) = self.tokens.pop() {
			match token {
				_ if token.is_newline() => continue,
				_ => {
					self.previous_span = token.span;
					return Some(token);
				}
			}
		}
		None
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
	assert_eq!(token_stream.expect(Tk::Eof), Ok(Token::new(TokenKind::Newline, DUMMY_SPAN)));
	assert_eq!(token_stream.matches(Tk::Eof), Some(Token::new(TokenKind::Newline, DUMMY_SPAN)));
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

	pub fn matches<P: Pattern>(&self, pattern: P) -> bool {
		pattern.matches(&self.kind)
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
	/// `->`
	ThinArrow,
	/// `=>`
	FatArrow,

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
	/// `module`
	Module,
	/// `use`
	Use,
	/// `fn`
	Fn,
	/// `enum`
	Enum,
	/// `struct`
	Struct,
	/// `trait`
	Trait,
	/// `impl`
	Impl,
	/// `const`
	Const,
	/// `mut`
	Mut,
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

impl TokenKind {
	pub fn matches<P: Pattern>(&self, pattern: P) -> bool {
		pattern.matches(self)
	}
}

const KEYWORDS_NUM: usize = 14;
pub const KEYWORDS_MAP: ([&str; KEYWORDS_NUM], [TokenKind; KEYWORDS_NUM]) = (
	[
		"module", "use", "fn", "enum", "struct", "trait", "impl", "const", "mut", "if", "else", "loop",
		"while", "for",
	],
	[
		TokenKind::Module,
		TokenKind::Use,
		TokenKind::Fn,
		TokenKind::Enum,
		TokenKind::Struct,
		TokenKind::Trait,
		TokenKind::Impl,
		TokenKind::Const,
		TokenKind::Mut,
		TokenKind::If,
		TokenKind::Else,
		TokenKind::Loop,
		TokenKind::While,
		TokenKind::For,
	],
);

pub trait Pattern: Clone {
	fn joined_str(&self, sep: &str) -> String;
	fn matches(&self, other: &TokenKind) -> bool;
	fn includes(&self, x: &Tk) -> bool;
}

impl<const N: usize> Pattern for &[Tk; N] {
	fn joined_str(&self, sep: &str) -> String {
		self.iter().join_format(sep, |kind| format!("{kind:?}"))
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self.iter().any(|kind| kind.matches(other))
	}

	fn includes(&self, x: &Tk) -> bool {
		self.contains(x)
	}
}

impl Pattern for Tk {
	fn joined_str(&self, _: &str) -> String {
		format!("{self:?}")
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self.matches(other)
	}

	fn includes(&self, x: &Tk) -> bool {
		self == x
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tk {
	Eof,
	Newline,

	// Punctuation
	Semi,
	Colon,
	ColonColon,
	Comma,
	Dot,
	ThinArrow,
	FatArrow,

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
	Module,
	Use,
	Fn,
	Enum,
	Struct,
	Trait,
	Impl,
	Const,
	Mut,
	If,
	Else,
	Loop,
	While,
	For,

	OpenDelim(Delimiter),
	CloseDelim(Delimiter),

	Ident,

	Literal,
	LiteralInt,
	LiteralFloat,
	LiteralChar,
	LiteralString,
}

impl Tk {
	// TODO: Remove the Tk enum
	const fn matches(&self, other: &TokenKind) -> bool {
		match (other, self) {
			(TokenKind::Newline, Tk::Newline) => true,

			// Punctuation
			(TokenKind::Semi, Tk::Semi)
			| (TokenKind::Colon, Tk::Colon)
			| (TokenKind::ColonColon, Tk::ColonColon)
			| (TokenKind::Comma, Tk::Comma)
			| (TokenKind::Dot, Tk::Dot)
			| (TokenKind::ThinArrow, Tk::ThinArrow)
			| (TokenKind::FatArrow, Tk::FatArrow) => true,

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
			(TokenKind::Module, Tk::Module)
			| (TokenKind::Use, Tk::Use)
			| (TokenKind::Fn, Tk::Fn)
			| (TokenKind::Enum, Tk::Enum)
			| (TokenKind::Struct, Tk::Struct)
			| (TokenKind::Trait, Tk::Trait)
			| (TokenKind::Impl, Tk::Impl)
			| (TokenKind::Mut, Tk::Mut)
			| (TokenKind::If, Tk::If)
			| (TokenKind::Else, Tk::Else)
			| (TokenKind::Loop, Tk::Loop)
			| (TokenKind::While, Tk::While)
			| (TokenKind::For, Tk::For) => true,

			(TokenKind::OpenDelim(delim), Tk::OpenDelim(m_delim))
			| (TokenKind::CloseDelim(delim), Tk::CloseDelim(m_delim)) => matches!(
				(delim, m_delim),
				(Delimiter::Paren, Delimiter::Paren)
					| (Delimiter::Brace, Delimiter::Brace)
					| (Delimiter::Bracket, Delimiter::Bracket)
			),

			(TokenKind::Ident(..), Tk::Ident) => true,

			(TokenKind::Literal(lit), kind_match) => matches!(
				(lit, kind_match),
				(_, Tk::Literal)
					| (LitKind::Int(_), Tk::LiteralInt)
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
