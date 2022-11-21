use rym_errors::{Diagnostic, Level, RymResult};
use rym_lexer::{Cursor, PrimitiveLitKind, PrimitiveTokenKind};
use rym_span::Span;
use rym_tt::{Delimiter, LitKind, Token, TokenKind};
use rym_unescape::unquote;
use smol_str::SmolStr;

type Pos = usize;

pub struct ConvertLinear<'a> {
	/// Absolute offset within the source of the current character.
	pos: Pos,
	/// Source text to tokenize.
	src: &'a str,
	/// Cursor for getting lexer tokens.
	cursor: Cursor<'a>,
}

impl<'a> ConvertLinear<'a> {
	#[cfg(test)]
	pub const fn new(src: &'a str, cursor: Cursor<'a>) -> Self {
		Self { pos: 0, src, cursor }
	}

	// pub fn collect_tuple(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
	// 	self.fold((Vec::new(), Vec::new()), |mut accum, result| match result {
	// 		Ok(token) => {
	// 			accum.0.push(token);
	// 			accum
	// 		}
	// 		Err(error) => {
	// 			accum.1.push(error);
	// 			accum
	// 		}
	// 	})
	// }

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
		then: TokenKind,
		otherwise: TokenKind,
	) -> TokenKind {
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

impl Iterator for ConvertLinear<'_> {
	type Item = RymResult<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((primitive_kind, mut span)) = self.bump() {
			println!("{span}: >{}<", self.src_from_span(&span));

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
					return Some(Err(
						Diagnostic::new_spanned(Level::Error, "Unterminated block comment", span)
							.sub_diagnostic(
								Level::Note,
								None,
								"Missing trailing `*/` to terminate the block comment",
							),
					));
				}

				// Punctuation
				PrimitiveTokenKind::Semi => TokenKind::Semi,
				PrimitiveTokenKind::Colon => self.match_next(
					&mut span,
					PrimitiveTokenKind::Colon,
					TokenKind::ColonColon,
					TokenKind::Colon,
				),
				PrimitiveTokenKind::Comma => TokenKind::Comma,
				PrimitiveTokenKind::Dot => TokenKind::Dot,

				// Operator like
				PrimitiveTokenKind::Or => {
					self.match_next(&mut span, PrimitiveTokenKind::Or, TokenKind::OrOr, TokenKind::Or)
				}
				PrimitiveTokenKind::And => {
					self.match_next(&mut span, PrimitiveTokenKind::And, TokenKind::AndAnd, TokenKind::And)
				}
				PrimitiveTokenKind::Plus => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::PlusEq, TokenKind::Plus)
				}
				PrimitiveTokenKind::Minus => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::MinusEq, TokenKind::Minus)
				}
				PrimitiveTokenKind::Star => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::StarEq, TokenKind::Star)
				}
				PrimitiveTokenKind::Slash => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::SlashEq, TokenKind::Slash)
				}
				PrimitiveTokenKind::Percent => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					TokenKind::PercentEq,
					TokenKind::Percent,
				),
				PrimitiveTokenKind::Eq => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::EqEq, TokenKind::Eq)
				}
				PrimitiveTokenKind::Bang => {
					self.match_next(&mut span, PrimitiveTokenKind::Eq, TokenKind::BangEq, TokenKind::Bang)
				}
				PrimitiveTokenKind::LessThan => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					TokenKind::LessThanEq,
					TokenKind::LessThan,
				),
				PrimitiveTokenKind::GreaterThan => self.match_next(
					&mut span,
					PrimitiveTokenKind::Eq,
					TokenKind::GreaterThanEq,
					TokenKind::GreaterThan,
				),

				// Delimiter
				PrimitiveTokenKind::OpenParen => TokenKind::OpenDelim(Delimiter::Paren),
				PrimitiveTokenKind::CloseParen => TokenKind::CloseDelim(Delimiter::Paren),
				PrimitiveTokenKind::OpenBrace => TokenKind::OpenDelim(Delimiter::Brace),
				PrimitiveTokenKind::CloseBrace => TokenKind::CloseDelim(Delimiter::Brace),
				PrimitiveTokenKind::OpenBracket => TokenKind::OpenDelim(Delimiter::Bracket),
				PrimitiveTokenKind::CloseBracket => TokenKind::CloseDelim(Delimiter::Bracket),

				// Indentifier or Keyword
				PrimitiveTokenKind::Ident => TokenKind::Ident(SmolStr::new(self.src_from_span(&span))),

				PrimitiveTokenKind::Literal { kind } => match kind {
					PrimitiveLitKind::Integer => TokenKind::Literal(LitKind::Integer(
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
							return Some(Err(
								Diagnostic::new_spanned(Level::Error, "Unterminated character literal", span)
									.sub_diagnostic(
										Level::Note,
										None,
										"Missing trailing `'` to terminate the character literal",
									),
							));
						}
						let string = match unquote(self.src_from_span(&span)) {
							Ok(string) => string,
							Err(err) => todo!("Unquote error: {err}"),
						};
						TokenKind::Literal(LitKind::Char(match string.parse::<char>() {
							Ok(char) => char,
							Err(err) => {
								return Some(Err(Diagnostic::new_spanned(
									Level::Error,
									format!("Could not parse <char>: {err}"),
									span,
								)))
							}
						}))
					}
					PrimitiveLitKind::String { terminated } => {
						if !terminated {
							return Some(Err(
								Diagnostic::new_spanned(Level::Error, "Unterminated string literal", span)
									.sub_diagnostic(
										Level::Note,
										None,
										"Missing trailing `\"` to terminate the string literal",
									),
							));
						}
						let string = match unquote(self.src_from_span(&span)) {
							Ok(string) => string,
							Err(err) => {
								return Some(Err(Diagnostic::new_spanned(Level::Error, err.to_string(), span)))
							}
						};
						TokenKind::Literal(LitKind::String(SmolStr::new(string)))
					}
				},

				PrimitiveTokenKind::Unkown => {
					return Some(Err(Diagnostic::new_spanned(Level::Error, "Invalid character", span)));
				}
				PrimitiveTokenKind::At
				| PrimitiveTokenKind::Caret
				| PrimitiveTokenKind::Dollar
				| PrimitiveTokenKind::Pound
				| PrimitiveTokenKind::Tilde
				| PrimitiveTokenKind::Question => {
					return Some(Err(Diagnostic::new_spanned(Level::Error, "Reserved character", span)));
				}
			};

			return Some(Ok(Token::new(kind, span)));
		}
		None
	}
}
