use rym_errors::{Diagnostic, Level, RymResult};
use rym_span::Span;
use smol_str::SmolStr;

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

	pub fn expect(&mut self, kind: TokenKind) -> RymResult<Token> {
		let Some(token) = self.peek() else {
			return Err(Diagnostic::new(Level::Error, format!("Expected `{kind:?}`")));
		};

		if token.kind == kind {
			return Ok(self.next().unwrap());
		}

		Err(Diagnostic::new(Level::Error, format!("Expected `{kind:?}` got `{:?}`", token.kind)))
	}

	pub fn peek(&self) -> Option<&Token> {
		dbg!(self);
		self.tokens.iter().filter(|tt| !tt.is_newline()).last()
	}

	pub fn peek_unfiltered(&self) -> Option<&Token> {
		self.tokens.iter().last()
	}

	pub fn is_next_newline(&self) -> bool {
		matches!(self.peek_unfiltered(), Some(tt) if tt.is_newline())
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

impl Token {
	pub const fn is_stmt_end(&self) -> bool {
		self.is_newline() || matches!(self.kind, TokenKind::Semi)
	}

	pub const fn is_newline(&self) -> bool {
		matches!(self.kind, TokenKind::Newline)
	}
}

#[cfg(test)]
#[test]
fn token_stream() {
	let mut token_stream = TokenStream::new(vec![
		Token { kind: TokenKind::Newline, span: Span::new(0, 1) },
		Token { kind: TokenKind::And, span: Span::new(1, 2) },
		Token { kind: TokenKind::And, span: Span::new(2, 3) },
	]);

	assert_eq!(token_stream.peek(), Some(&Token { kind: TokenKind::And, span: Span::new(1, 2) }));
	assert_eq!(token_stream.is_next_newline(), true);
	assert_eq!(token_stream.next(), Some(Token { kind: TokenKind::And, span: Span::new(1, 2) }));
	assert_eq!(token_stream.is_next_newline(), false);
	assert_eq!(
		token_stream.expect(TokenKind::And),
		Ok(Token { kind: TokenKind::And, span: Span::new(2, 3) })
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
