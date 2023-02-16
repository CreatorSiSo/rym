use rym_unescape::unquote;
use Mapped::{Multi, Single};

mod test;
mod token;
pub use token::*;

use crate::primitive;

type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
	/// Absolute offset within the source of the current character.
	pos: usize,
	/// Source text to tokenize.
	src: &'a str,
	/// Cursor for getting lexer tokens.
	primitive_lexer: primitive::Lexer<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(src: &'a str) -> Self {
		Self { pos: 0, src, primitive_lexer: primitive::Lexer::new(src) }
	}

	fn bump(&mut self) -> Option<(primitive::TokenKind, Span)> {
		self.primitive_lexer.next_token().map(|token| {
			let start_pos = self.pos;
			self.pos += token.len as usize;
			(token.kind, start_pos..self.pos)
		})
	}

	fn peek(&self) -> Option<(primitive::TokenKind, Span)> {
		self.primitive_lexer.clone().next_token().map(|token| {
			let end_pos = self.pos + token.len as usize;
			(token.kind, self.pos..end_pos)
		})
	}

	fn src_from_span(&self, span: &Span) -> &'a str {
		&self.src[span.start..span.end]
	}

	fn map_next(
		&mut self,
		span: &mut Span,
		map_fn: impl Fn(Option<primitive::TokenKind>) -> Mapped,
	) -> Token {
		let mut end = span.end;
		match if let Some((kind, end_span)) = self.peek() {
			end = end_span.end;
			map_fn(Some(kind))
		} else {
			map_fn(None)
		} {
			Mapped::Multi(kind) => {
				*span = span.start..end;
				self.bump();
				kind
			}
			Mapped::Single(kind) => kind,
		}
	}
}

enum Mapped {
	Single(Token),
	Multi(Token),
}

impl Iterator for Lexer<'_> {
	type Item = (Token, Span);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((primitive_kind, mut span)) = self.bump() {
			let kind = match primitive_kind {
				primitive::TokenKind::Whitespace => {
					if self.src_from_span(&span).contains('\n') {
						Token::Newline
					} else {
						continue;
					}
				}
				primitive::TokenKind::LineComment => continue,
				primitive::TokenKind::BlockComment { terminated } => {
					if terminated {
						continue;
					}
					// TODO: Special reporting for nested block comments
					// self.handler.emit(
					// 	Diagnostic::new_spanned(Level::Error, "Unterminated block comment", span)
					// 		.sub_diagnostic(
					// 			Level::Note,
					// 			None,
					// 			"Missing trailing `*/` to terminate the block comment",
					// 		),
					// );
					continue;
				}

				// Punctuation
				primitive::TokenKind::Semi => Token::Semi,
				primitive::TokenKind::Colon => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Colon) => Multi(Token::ColonColon),
					_ => Single(Token::Colon),
				}),
				primitive::TokenKind::Dot => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Dot) => Multi(Token::DotDot),
					_ => Single(Token::Dot),
				}),
				primitive::TokenKind::Comma => Token::Comma,

				// Operator like
				primitive::TokenKind::Or => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Or) => Multi(Token::OrOr),
					_ => Single(Token::Or),
				}),
				primitive::TokenKind::And => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::And) => Multi(Token::AndAnd),
					_ => Single(Token::And),
				}),
				primitive::TokenKind::Plus => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::PlusEq),
					_ => Single(Token::Plus),
				}),
				primitive::TokenKind::Minus => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::MinusEq),
					Some(primitive::TokenKind::GreaterThan) => Multi(Token::ThinArrow),
					_ => Single(Token::Minus),
				}),
				primitive::TokenKind::Star => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::StarEq),
					_ => Single(Token::Star),
				}),
				primitive::TokenKind::Slash => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::SlashEq),
					_ => Single(Token::Slash),
				}),
				primitive::TokenKind::Percent => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::PercentEq),
					_ => Single(Token::Percent),
				}),
				primitive::TokenKind::Eq => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::EqEq),
					Some(primitive::TokenKind::GreaterThan) => Multi(Token::FatArrow),
					_ => Single(Token::Eq),
				}),
				primitive::TokenKind::Bang => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::BangEq),
					_ => Single(Token::Bang),
				}),
				primitive::TokenKind::LessThan => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::LessThanEq),
					_ => Single(Token::LessThan),
				}),
				primitive::TokenKind::GreaterThan => self.map_next(&mut span, |kind| match kind {
					Some(primitive::TokenKind::Eq) => Multi(Token::GreaterThanEq),
					_ => Single(Token::GreaterThan),
				}),

				// Delimiter
				primitive::TokenKind::OpenParen => Token::OpenParen,
				primitive::TokenKind::CloseParen => Token::CloseParen,
				primitive::TokenKind::OpenBrace => Token::OpenBrace,
				primitive::TokenKind::CloseBrace => Token::CloseBrace,
				primitive::TokenKind::OpenBracket => Token::OpenBracket,
				primitive::TokenKind::CloseBracket => Token::CloseBracket,

				// Indentifier or Keyword
				primitive::TokenKind::Ident => {
					let name = self.src_from_span(&span);
					match KEYWORDS_MAP.0.iter().position(|kw| *kw == name) {
						Some(pos) => KEYWORDS_MAP.1[pos].clone(),
						None => Token::Ident(name.to_string()),
					}
				}

				primitive::TokenKind::Integer => Token::Int(
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
				),
				primitive::TokenKind::Float => {
					let Some((l, r)) = self.src_from_span(&span).split_once('.') else {
							unreachable!("Internal Error: Float literal does not contain a '.'")
						};
					let l_val: u32 = l.parse().expect(&format!(
						"Internal Error: Left hand side of float literal has invalid value `{l}`"
					));
					let r_val: u32 = r.parse().unwrap_or(0);
					Token::Float(l_val, r_val)
				}
				primitive::TokenKind::Char { terminated } => {
					if !terminated {
						todo!();
						// TODO
						// self.handler.emit(
						// 	Diagnostic::new_spanned(Level::Error, "Unterminated character literal", span)
						// 		.sub_diagnostic(
						// 			Level::Note,
						// 			None,
						// 			"Missing trailing `'` to terminate the character literal",
						// 		),
						// );
						// continue;
					}
					let string = match unquote(self.src_from_span(&span)) {
						Ok(string) => string,
						Err(err) => todo!("Unquote error: {err}"),
					};
					Token::Char(match string.parse::<char>() {
						Ok(char) => char,
						Err(_err) => {
							todo!();
							// TODO
							// self.handler.emit(Diagnostic::new_spanned(
							// 	Level::Error,
							// 	format!("Could not parse <char>: {err}"),
							// 	span,
							// ));
							// continue;
						}
					})
				}
				primitive::TokenKind::String { terminated } => {
					if !terminated {
						// TODO
						// self.handler.emit(
						// 	Diagnostic::new_spanned(Level::Error, "Unterminated string literal", span)
						// 		.sub_diagnostic(
						// 			Level::Note,
						// 			None,
						// 			"Missing trailing `\"` to terminate the string literal",
						// 		),
						// );
						continue;
					}
					let string = match unquote(self.src_from_span(&span)) {
						Ok(string) => string,
						Err(_err) => {
							// TODO
							// self.handler.emit(Diagnostic::new_spanned(Level::Error, err.to_string(), span));
							continue;
						}
					};
					Token::String(string)
				}

				primitive::TokenKind::Unkown => {
					// TODO
					// self.handler.emit(Diagnostic::new_spanned(Level::Error, "Invalid character", span));
					continue;
				}
				primitive::TokenKind::At
				| primitive::TokenKind::Caret
				| primitive::TokenKind::Dollar
				| primitive::TokenKind::Pound
				| primitive::TokenKind::Tilde
				| primitive::TokenKind::Question => {
					// TODO
					// self.handler.emit(Diagnostic::new_spanned(Level::Error, "Reserved character", span));
					continue;
				}
			};

			return Some((kind, span));
		}
		None
	}
}
