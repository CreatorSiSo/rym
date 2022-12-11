use rym_errors::{Diagnostic, DiagnosticHandler, Level};
use rym_lexer::{Cursor, PrimitiveLitKind, PrimitiveTokenKind};
use rym_span::Span;
use rym_unescape::unquote;
use smol_str::SmolStr;

mod test;
mod token_stream;
pub use token_stream::*;

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
	handler: &'a DiagnosticHandler,
}

impl<'a> LinearLexer<'a> {
	pub(crate) fn new(src: &'a str, handler: &'a DiagnosticHandler) -> Self {
		Self { pos: 0, src, cursor: Cursor::new(src), handler }
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

	fn src_from_span(&self, span: &Span) -> &'a str {
		&self.src[span.start..span.end]
	}

	fn map_next(
		&mut self,
		span: &mut Span,
		map_fn: impl Fn(Option<PrimitiveTokenKind>) -> Mapped,
	) -> TokenKind {
		let mut end = span.end;
		match if let Some((kind, end_span)) = self.peek() {
			end = end_span.end;
			map_fn(Some(kind))
		} else {
			map_fn(None)
		} {
			Mapped::Multi(kind) => {
				*span = Span::new(span.start, end);
				self.bump();
				kind
			}
			Mapped::Single(kind) => kind,
		}
	}
}

enum Mapped {
	Single(TokenKind),
	Multi(TokenKind),
}
use Mapped::{Multi, Single};

impl Iterator for LinearLexer<'_> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((primitive_kind, mut span)) = self.bump() {
			let kind = match primitive_kind {
				PrimitiveTokenKind::Whitespace => {
					if self.src_from_span(&span).contains('\n') {
						TokenKind::Newline
					} else {
						continue;
					}
				}
				PrimitiveTokenKind::LineComment => continue,
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
				PrimitiveTokenKind::Semi => TokenKind::Semi,
				PrimitiveTokenKind::Colon => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Colon) => Multi(TokenKind::ColonColon),
					_ => Single(TokenKind::Colon),
				}),
				PrimitiveTokenKind::Dot => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Dot) => Multi(TokenKind::DotDot),
					_ => Single(TokenKind::Dot),
				}),
				PrimitiveTokenKind::Comma => TokenKind::Comma,

				// Operator like
				PrimitiveTokenKind::Or => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Or) => Multi(TokenKind::OrOr),
					_ => Single(TokenKind::Or),
				}),
				PrimitiveTokenKind::And => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::And) => Multi(TokenKind::AndAnd),
					_ => Single(TokenKind::And),
				}),
				PrimitiveTokenKind::Plus => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::PlusEq),
					_ => Single(TokenKind::Plus),
				}),
				PrimitiveTokenKind::Minus => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::MinusEq),
					Some(PrimitiveTokenKind::GreaterThan) => Multi(TokenKind::ThinArrow),
					_ => Single(TokenKind::Minus),
				}),
				PrimitiveTokenKind::Star => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::StarEq),
					_ => Single(TokenKind::Star),
				}),
				PrimitiveTokenKind::Slash => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::SlashEq),
					_ => Single(TokenKind::Slash),
				}),
				PrimitiveTokenKind::Percent => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::PercentEq),
					_ => Single(TokenKind::Percent),
				}),
				PrimitiveTokenKind::Eq => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::EqEq),
					Some(PrimitiveTokenKind::GreaterThan) => Multi(TokenKind::FatArrow),
					_ => Single(TokenKind::Eq),
				}),
				PrimitiveTokenKind::Bang => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::BangEq),
					_ => Single(TokenKind::Bang),
				}),
				PrimitiveTokenKind::LessThan => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::LessThanEq),
					_ => Single(TokenKind::LessThan),
				}),
				PrimitiveTokenKind::GreaterThan => self.map_next(&mut span, |kind| match kind {
					Some(PrimitiveTokenKind::Eq) => Multi(TokenKind::GreaterThanEq),
					_ => Single(TokenKind::GreaterThan),
				}),

				// Delimiter
				PrimitiveTokenKind::OpenParen => TokenKind::OpenDelim(Delimiter::Paren),
				PrimitiveTokenKind::CloseParen => TokenKind::CloseDelim(Delimiter::Paren),
				PrimitiveTokenKind::OpenBrace => TokenKind::OpenDelim(Delimiter::Brace),
				PrimitiveTokenKind::CloseBrace => TokenKind::CloseDelim(Delimiter::Brace),
				PrimitiveTokenKind::OpenBracket => TokenKind::OpenDelim(Delimiter::Bracket),
				PrimitiveTokenKind::CloseBracket => TokenKind::CloseDelim(Delimiter::Bracket),

				// Indentifier or Keyword
				PrimitiveTokenKind::Ident => {
					let name = self.src_from_span(&span);
					match KEYWORDS_MAP.0.iter().position(|kw| *kw == name) {
						Some(pos) => KEYWORDS_MAP.1[pos].clone(),
						None => TokenKind::Ident(SmolStr::new(name)),
					}
				}

				PrimitiveTokenKind::Literal { kind } => match kind {
					PrimitiveLitKind::Integer => TokenKind::Literal(LitKind::Int(
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
					PrimitiveLitKind::Float => TokenKind::Literal(LitKind::Float(
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
						TokenKind::Literal(LitKind::Char(match string.parse::<char>() {
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
						TokenKind::Literal(LitKind::String(string))
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

			return Some(Token::new(kind, span));
		}
		None
	}
}
