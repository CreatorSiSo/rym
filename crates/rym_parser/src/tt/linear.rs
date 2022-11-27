use rym_errors::{Diagnostic, Handler, Level};
use rym_lexer::{Cursor, PrimitiveLitKind, PrimitiveTokenKind};
use rym_span::Span;
use rym_tt::{Delimiter, LitKind, Token, TokenKind};
use rym_unescape::unquote;
use smol_str::SmolStr;

type Pos = usize;

#[derive(Clone, Debug)]
pub(crate) struct LinearLexer<'a> {
	/// Absolute offset within the source of the current character.
	pos: Pos,
	/// Source text to tokenize.
	src: &'a str,
	/// Cursor for getting lexer tokens.
	cursor: Cursor<'a>,
	/// Global struct to submit diagnostics to.
	handler: &'a Handler,
}

impl<'a> LinearLexer<'a> {
	pub(crate) fn new(src: &'a str, handler: &'a Handler) -> Self {
		Self { pos: 0, src, cursor: Cursor::new(src), handler }
	}

	pub(crate) fn is_next_newline(&self) -> bool {
		if let Some((kind, span)) = self.peek() {
			dbg!(format!("{span}: >{}<", self.src_from_span(&span)));
			if kind == PrimitiveTokenKind::Whitespace && self.src_from_span(&span).contains('\n') {
				return true;
			}
		}
		false
	}

	fn bump(&mut self) -> Option<(PrimitiveTokenKind, Span)> {
		self.cursor.next_token().map(|token| {
			let start_pos = self.pos;
			self.pos += token.len as usize;
			(token.kind, Span::new(start_pos, self.pos))
		})
	}

	fn peek(&self) -> Option<(PrimitiveTokenKind, Span)> {
		self.cursor.clone().next_token().map(|token| {
			let end_pos = self.pos + token.len as usize;
			(token.kind, Span::new(self.pos, end_pos))
		})
	}

	fn match_next(
		&mut self,
		span: &mut Span,
		condition: PrimitiveTokenKind,
		then: LinearTokenKind,
		otherwise: LinearTokenKind,
	) -> LinearTokenKind {
		if let Some((primitive_kind, end_span)) = self.peek() {
			if primitive_kind == condition {
				*span = Span::new(span.start, end_span.end);
				self.bump();
				return then;
			}
		}
		otherwise
	}

	fn src_from_span(&self, span: &Span) -> &'a str {
		&self.src[span.start..span.end]
	}
}

impl Iterator for LinearLexer<'_> {
	type Item = LinearToken;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((primitive_kind, mut span)) = self.bump() {
			let kind = match primitive_kind {
				PrimitiveTokenKind::Whitespace | PrimitiveTokenKind::LineComment => continue,
				PrimitiveTokenKind::BlockComment { terminated } => {
					if terminated {
						continue;
					}
					// TODO: Special reporting for nested block comments
					self.handler.emit(
						Diagnostic::new_spanned(Level::Error, "Unterminated block comment", span)
							.sub_diagnostic(
								Level::Note,
								None,
								"Missing trailing `*/` to terminate the block comment",
							),
					);
					continue;
				}

				// Punctuation
				PrimitiveTokenKind::Semi => LinearTokenKind::Semi,
				PrimitiveTokenKind::Colon => self.match_next(
					&mut span,
					PrimitiveTokenKind::Colon,
					LinearTokenKind::ColonColon,
					LinearTokenKind::Colon,
				),
				PrimitiveTokenKind::Comma => LinearTokenKind::Comma,
				PrimitiveTokenKind::Dot => LinearTokenKind::Dot,

				// Operator like
				PrimitiveTokenKind::Or => self.match_next(
					&mut span,
					PrimitiveTokenKind::Or,
					LinearTokenKind::OrOr,
					LinearTokenKind::Or,
				),
				PrimitiveTokenKind::And => self.match_next(
					&mut span,
					PrimitiveTokenKind::And,
					LinearTokenKind::AndAnd,
					LinearTokenKind::And,
				),
				PrimitiveTokenKind::Plus => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::PlusEq,
					LinearTokenKind::Plus,
				),
				PrimitiveTokenKind::Minus => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::MinusEq,
					LinearTokenKind::Minus,
				),
				PrimitiveTokenKind::Star => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::StarEq,
					LinearTokenKind::Star,
				),
				PrimitiveTokenKind::Slash => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::SlashEq,
					LinearTokenKind::Slash,
				),
				PrimitiveTokenKind::Percent => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::PercentEq,
					LinearTokenKind::Percent,
				),
				PrimitiveTokenKind::Eq => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::EqEq,
					LinearTokenKind::Eq,
				),
				PrimitiveTokenKind::Bang => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::BangEq,
					LinearTokenKind::Bang,
				),
				PrimitiveTokenKind::LessThan => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::LessThanEq,
					LinearTokenKind::LessThan,
				),
				PrimitiveTokenKind::GreaterThan => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					LinearTokenKind::GreaterThanEq,
					LinearTokenKind::GreaterThan,
				),

				// Delimiter
				PrimitiveTokenKind::OpenParen => LinearTokenKind::OpenDelim(Delimiter::Paren),
				PrimitiveTokenKind::CloseParen => LinearTokenKind::CloseDelim(Delimiter::Paren),
				PrimitiveTokenKind::OpenBrace => LinearTokenKind::OpenDelim(Delimiter::Brace),
				PrimitiveTokenKind::CloseBrace => LinearTokenKind::CloseDelim(Delimiter::Brace),
				PrimitiveTokenKind::OpenBracket => LinearTokenKind::OpenDelim(Delimiter::Bracket),
				PrimitiveTokenKind::CloseBracket => LinearTokenKind::CloseDelim(Delimiter::Bracket),

				// Indentifier or Keyword
				PrimitiveTokenKind::Ident => {
					LinearTokenKind::Ident(SmolStr::new(self.src_from_span(&span)))
				}

				PrimitiveTokenKind::Literal { kind } => match kind {
					PrimitiveLitKind::Integer => LinearTokenKind::Literal(LitKind::Integer(
						match self
							.src_from_span(&span)
							.chars()
							.filter(|c| c != &'_')
							.collect::<String>()
							.parse::<i64>()
						{
							Ok(int) => int,
							Err(err) => unreachable!(
								"Internal Error: Could not parse <i64> from {0}: `{err}`",
								self.src_from_span(&span)
							),
						},
					)),
					PrimitiveLitKind::Float => LinearTokenKind::Literal(LitKind::Float(
						match self
							.src_from_span(&span)
							.chars()
							.filter(|c| c != &'_')
							.collect::<String>()
							.parse::<f64>()
						{
							Ok(float) => float,
							Err(err) => unreachable!(
								"Internal Error: Could not parse <f64> from {0}: {err}",
								self.src_from_span(&span)
							),
						},
					)),
					PrimitiveLitKind::Char { terminated } => {
						if !terminated {
							self.handler.emit(
								Diagnostic::new_spanned(Level::Error, "Unterminated character literal", span)
									.sub_diagnostic(
										Level::Note,
										None,
										"Missing trailing `'` to terminate the character literal",
									),
							);
							continue;
						}
						let string = match unquote(self.src_from_span(&span)) {
							Ok(string) => string,
							Err(err) => todo!("Unquote error: {err}"),
						};
						LinearTokenKind::Literal(LitKind::Char(match string.parse::<char>() {
							Ok(char) => char,
							Err(err) => {
								self.handler.emit(Diagnostic::new_spanned(
									Level::Error,
									format!("Could not parse <char>: {err}"),
									span,
								));
								continue;
							}
						}))
					}
					PrimitiveLitKind::String { terminated } => {
						if !terminated {
							self.handler.emit(
								Diagnostic::new_spanned(Level::Error, "Unterminated string literal", span)
									.sub_diagnostic(
										Level::Note,
										None,
										"Missing trailing `\"` to terminate the string literal",
									),
							);
							continue;
						}
						let string = match unquote(self.src_from_span(&span)) {
							Ok(string) => string,
							Err(err) => {
								self.handler.emit(Diagnostic::new_spanned(Level::Error, err.to_string(), span));
								continue;
							}
						};
						LinearTokenKind::Literal(LitKind::String(string))
					}
				},

				PrimitiveTokenKind::Unkown => {
					self.handler.emit(Diagnostic::new_spanned(Level::Error, "Invalid character", span));
					continue;
				}
				PrimitiveTokenKind::At
				| PrimitiveTokenKind::Caret
				| PrimitiveTokenKind::Dollar
				| PrimitiveTokenKind::Pound
				| PrimitiveTokenKind::Tilde
				| PrimitiveTokenKind::Question => {
					self.handler.emit(Diagnostic::new_spanned(Level::Error, "Reserved character", span));
					continue;
				}
			};

			return Some(LinearToken::new(kind, span));
		}
		None
	}
}

#[derive(Debug, PartialEq)]
pub(crate) struct LinearToken {
	pub(crate) kind: LinearTokenKind,
	pub(crate) span: Span,
}

impl LinearToken {
	pub(crate) const fn new(kind: LinearTokenKind, span: Span) -> Self {
		Self { kind, span }
	}

	// TODO: Benchmark if #[inline] makes a difference here
	/// ## Panic
	/// Panics if self.kind is `LinearTokenKind::OpenDelim` | `LinearTokenKind::CloseDelim`
	pub(crate) fn into_token(self) -> Token {
		let kind = match self.kind {
			LinearTokenKind::Semi => TokenKind::Semi,
			LinearTokenKind::Colon => TokenKind::Colon,
			LinearTokenKind::ColonColon => TokenKind::ColonColon,
			LinearTokenKind::Comma => TokenKind::Comma,
			LinearTokenKind::Dot => TokenKind::Dot,
			LinearTokenKind::Or => TokenKind::Or,
			LinearTokenKind::OrOr => TokenKind::OrOr,
			LinearTokenKind::And => TokenKind::And,
			LinearTokenKind::AndAnd => TokenKind::AndAnd,
			LinearTokenKind::Plus => TokenKind::Plus,
			LinearTokenKind::PlusEq => TokenKind::PlusEq,
			LinearTokenKind::Minus => TokenKind::Minus,
			LinearTokenKind::MinusEq => TokenKind::MinusEq,
			LinearTokenKind::Star => TokenKind::Star,
			LinearTokenKind::StarEq => TokenKind::StarEq,
			LinearTokenKind::Slash => TokenKind::Slash,
			LinearTokenKind::SlashEq => TokenKind::SlashEq,
			LinearTokenKind::Percent => TokenKind::Percent,
			LinearTokenKind::PercentEq => TokenKind::PercentEq,
			LinearTokenKind::Bang => TokenKind::Bang,
			LinearTokenKind::BangEq => TokenKind::BangEq,
			LinearTokenKind::Eq => TokenKind::Eq,
			LinearTokenKind::EqEq => TokenKind::EqEq,
			LinearTokenKind::LessThan => TokenKind::LessThan,
			LinearTokenKind::LessThanEq => TokenKind::LessThanEq,
			LinearTokenKind::GreaterThan => TokenKind::GreaterThan,
			LinearTokenKind::GreaterThanEq => TokenKind::GreaterThanEq,
			LinearTokenKind::Ident(name) => TokenKind::Ident(name),
			LinearTokenKind::Literal(lit) => TokenKind::Literal(lit),
			delim => unreachable!("Internal Error: Cannot convert {delim:?} to TokenKind"),
		};
		Token { kind, span: self.span }
	}
}

#[derive(Debug, PartialEq)]
pub(crate) enum LinearTokenKind {
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
