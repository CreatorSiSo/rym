use rym_errors::{Diagnostic, Level, RymResult};
use rym_span::{Span, DUMMY_SPAN};
use smol_str::SmolStr;
use std::fmt::Debug;
use stringx::Join;

pub trait Pattern: Clone + Debug {
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
		self.tokens.reverse()
	}

	pub fn consume_until<P: Pattern>(&mut self, pattern: P) {
		self.tokens = self.skip_while(|token| !token.matches(pattern.clone())).collect();
		self.tokens.reverse()
	}

	pub fn expect_ident(&mut self) -> RymResult<(SmolStr, Span)> {
		let Token { kind: TokenKind::Ident(name), span } = self.expect(TokenKind::AnyIdent)? else {
			// SAFETY: self.expect() ensures that kind is always TokenKind::Ident()
			// unsafe { std::hint::unreachable_unchecked() }
			panic!()
		};
		Ok((name, span))
	}

	/// Consume next token if it matches one of the token kinds in the pattern
	/// otherwise return an error
	pub fn expect<P: Pattern>(&mut self, pattern: P) -> RymResult<Token> {
		if let Some(token) = self.matches(pattern.clone()) {
			return Ok(token);
		}
		if let Some(got) = self.peek(pattern.includes(&TokenKind::Newline)) {
			return Err(Diagnostic::new_spanned(
				Level::Error,
				format!("Expected `{}` got `{:?}`", pattern.joined_str(" | "), got.kind),
				got.span,
			));
		}
		Err(Diagnostic::new(Level::Error, format!("Expected `{}`", pattern.joined_str(" | "))))
	}

	/// Consume next token if it matches one of the token kinds in the pattern
	pub fn matches<P: Pattern>(&mut self, pattern: P) -> Option<Token> {
		// Only process newline tokens if the pattern contains them
		let process_newlines = pattern.includes(&TokenKind::Newline);

		let Some(got) = self.peek(process_newlines) else {
			return pattern.includes(&TokenKind::Eof).then_some(Token::new(TokenKind::Newline, DUMMY_SPAN));
		};

		if pattern.matches(&got.kind) {
			return Some(
				// SAFETY: Next token must exist because self.peek() is Some(..)
				if process_newlines { self.next_unfiltered().unwrap() } else { self.next().unwrap() },
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
		Token { kind: TokenKind::OpenDelim(Delimiter::Paren), span: Span::new(0, 0) },
		Token { kind: TokenKind::OpenDelim(Delimiter::Bracket), span: Span::new(0, 0) },
		Token { kind: TokenKind::CloseDelim(Delimiter::Bracket), span: Span::new(0, 0) },
		Token { kind: TokenKind::CloseDelim(Delimiter::Paren), span: Span::new(0, 0) },
	]);

	assert_eq!(
		token_stream.peek(false),
		Some(&Token { kind: TokenKind::And, span: Span::new(1, 2) })
	);

	assert_eq!(token_stream.is_next_newline(), true);
	assert_eq!(token_stream.next(), Some(Token { kind: TokenKind::And, span: Span::new(1, 2) }));
	assert_eq!(token_stream.is_next_newline(), false);
	assert_eq!(
		token_stream.expect(TokenKind::AnyIdent),
		Ok(Token { kind: TokenKind::Ident(SmolStr::new("test")), span: Span::new(2, 6) })
	);

	token_stream.consume_until(TokenKind::OpenDelim(Delimiter::Paren));
	assert_eq!(
		token_stream.next_unfiltered().map(|token| token.kind),
		Some(TokenKind::OpenDelim(Delimiter::Paren))
	);

	token_stream.consume_while(TokenKind::OpenDelim(Delimiter::Bracket));
	assert_eq!(
		token_stream.next_unfiltered().map(|token| token.kind),
		Some(TokenKind::CloseDelim(Delimiter::Bracket))
	);

	token_stream.consume_until(TokenKind::Eof);

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
	/// `.`
	Dot,
	/// `..`
	DotDot,
	/// `,`
	Comma,

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
	/// `->`
	ThinArrow,
	/// `=>`
	FatArrow,

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

	/// Special token kinds used in patterns
	AnyIdent,
	AnyLiteral,
	AnyLiteralInt,
	AnyLiteralFloat,
	AnyLiteralChar,
	AnyLiteralString,

	/// `\n`
	Newline,
	/// Well thats where it ends...
	Eof,
}

impl TokenKind {
	pub fn matches<P: Pattern>(&self, pattern: &P) -> bool {
		pattern.matches(self)
	}
}

impl<const N: usize> Pattern for [TokenKind; N] {
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

	fn matches(&self, other: &Self) -> bool {
		match (self, other) {
			_ if self == other => true,
			(Self::Ident(_), Self::AnyIdent) | (Self::AnyIdent, Self::Ident(_)) => true,
			(Self::Literal(_), Self::AnyLiteral)
			| (Self::Literal(LitKind::Int(_)), Self::AnyLiteralInt)
			| (Self::Literal(LitKind::Float(_)), Self::AnyLiteralFloat)
			| (Self::Literal(LitKind::Char(_)), Self::AnyLiteralChar)
			| (Self::Literal(LitKind::String(_)), Self::AnyLiteralString)
			| (Self::AnyLiteral, Self::Literal(_))
			| (Self::AnyLiteralInt, Self::Literal(LitKind::Int(_)))
			| (Self::AnyLiteralFloat, Self::Literal(LitKind::Float(_)))
			| (Self::AnyLiteralChar, Self::Literal(LitKind::Char(_)))
			| (Self::AnyLiteralString, Self::Literal(LitKind::String(_))) => true,
			_ => false,
		}
	}

	fn includes(&self, x: &Self) -> bool {
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
