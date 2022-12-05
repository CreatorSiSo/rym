use rym_errors::{Diagnostic, Level, RymResult};
use rym_span::{Span, DUMMY_SPAN};
use smol_str::SmolStr;
use std::fmt::Debug;
use stringx::Join;

pub trait Pattern: Clone {
	fn joined_str(&self, sep: &str) -> String;
	fn matches(&self, other: &TokenKind) -> bool;
	fn includes(&self, x: &TokenKind) -> bool;
}

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
		let Token { kind: TokenKind::Ident(name), span } = self.expect(ValueTokenKind::Ident)? else {
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
			None => match self.peek(pattern.includes(&TokenKind::Newline)) {
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
		let process_newlines = pattern.includes(&TokenKind::Newline);
		let Some(got) = self.peek(process_newlines) else {
			return if pattern.includes(&TokenKind::Eof) {
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
		token_stream.expect(ValueTokenKind::Ident),
		Ok(Token { kind: TokenKind::Ident(SmolStr::new("test")), span: Span::new(2, 6) })
	);
	assert_eq!(token_stream.expect(TokenKind::Eof), Ok(Token::new(TokenKind::Newline, DUMMY_SPAN)));
	assert_eq!(
		token_stream.matches(TokenKind::Eof),
		Some(Token::new(TokenKind::Newline, DUMMY_SPAN))
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

	pub fn matches<P: Pattern>(&self, pattern: P) -> bool {
		pattern.matches(&self.kind)
	}
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TokenKind {
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

	/// `\n`
	Newline,
	///
	Eof,
}

impl TokenKind {
	pub fn matches<P: Pattern>(&self, pattern: &P) -> bool {
		pattern.matches(self)
	}
}

impl<const N: usize> Pattern for &[TokenKind; N] {
	fn joined_str(&self, sep: &str) -> String {
		self.iter().join_format(sep, |kind| format!("{kind:?}"))
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self.iter().any(|kind| kind.matches(other))
	}

	fn includes(&self, x: &TokenKind) -> bool {
		self.contains(x)
	}
}

impl Pattern for TokenKind {
	fn joined_str(&self, _: &str) -> String {
		format!("{self:?}")
	}

	fn matches(&self, other: &TokenKind) -> bool {
		self == other
	}

	fn includes(&self, x: &TokenKind) -> bool {
		self == x
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

#[derive(Debug, Clone)]
pub enum ValueTokenKind {
	Ident,
	LiteralInt,
	LiteralFloat,
	LiteralChar,
	LiteralString,
}

impl Pattern for ValueTokenKind {
	fn joined_str(&self, _: &str) -> String {
		format!("{self:?}")
	}

	fn matches(&self, other: &TokenKind) -> bool {
		matches!(
			(other, self),
			(TokenKind::Ident(_), ValueTokenKind::Ident)
				| (TokenKind::Literal(LitKind::Int(_)), ValueTokenKind::LiteralInt)
				| (TokenKind::Literal(LitKind::Float(_)), ValueTokenKind::LiteralFloat)
				| (TokenKind::Literal(LitKind::Char(_)), ValueTokenKind::LiteralFloat)
				| (TokenKind::Literal(LitKind::String(_)), ValueTokenKind::LiteralString)
		)
	}

	fn includes(&self, x: &TokenKind) -> bool {
		self.matches(x)
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
