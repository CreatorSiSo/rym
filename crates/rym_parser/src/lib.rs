use rym_lexer::{Cursor, PrimitiveLitKind, PrimitiveTokenKind};
use rym_span::Span;
use rym_tt::{Delimiter, LitKind, Token, TokenKind};
use rym_unescape::unquote;
use smol_str::SmolStr;

mod tests;

type Pos = usize;

pub struct TokenConverter<'a> {
	/// Absolute offset within the source of the current character.
	pos: Pos,
	/// Source text to tokenize.
	src: &'a str,
	/// Cursor for getting lexer tokens.
	cursor: Cursor<'a>,
}

impl<'a> TokenConverter<'a> {
	pub const fn new(src: &'a str, cursor: Cursor<'a>) -> Self {
		Self { pos: 0, src, cursor }
	}

	pub fn collect_tuple(&mut self) -> (Vec<Token>, Vec<Span>) {
		self.fold((Vec::new(), Vec::new()), |mut accum, result| match result {
			Ok(token) => {
				accum.0.push(token);
				accum
			}
			Err(error) => {
				accum.1.push(error);
				accum
			}
		})
	}

	fn next_token(&mut self) -> Option<(PrimitiveTokenKind, Span)> {
		self.cursor.next_token().map(|token| {
			let start_pos = self.pos;
			self.pos += token.len as usize;
			(token.kind, Span::new(start_pos, self.pos))
		})
	}

	fn src_from_span(&self, span: &Span) -> &'a str {
		&self.src[span.start..span.end]
	}
}

impl Iterator for TokenConverter<'_> {
	type Item = Result<Token, Span>;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO let mut preceeded_by_whitespace = false;
		while let Some((old_kind, span)) = self.next_token() {
			println!(">{}<", self.src_from_span(&span));

			let new_kind = match old_kind {
				PrimitiveTokenKind::Whitespace => {
					// TODO preceeded_by_whitespace = true;
					if self.src_from_span(&span).contains('\n') {
						TokenKind::Newline
					} else {
						continue;
					}
				}

				// Punctuation
				PrimitiveTokenKind::Semi => TokenKind::Semi,
				PrimitiveTokenKind::Colon => TokenKind::Colon, /* ColonColon */
				PrimitiveTokenKind::Comma => TokenKind::Comma,
				PrimitiveTokenKind::Dot => TokenKind::Dot,

				// Operator like
				PrimitiveTokenKind::Or => TokenKind::Or,           /* OrOr */
				PrimitiveTokenKind::And => TokenKind::And,         /* AndAnd */
				PrimitiveTokenKind::Plus => TokenKind::Plus,       /* PlusEq */
				PrimitiveTokenKind::Minus => TokenKind::Minus,     /* MinusEq */
				PrimitiveTokenKind::Star => TokenKind::Star,       /* StarEq */
				PrimitiveTokenKind::Slash => TokenKind::Slash,     /* SlashEq */
				PrimitiveTokenKind::Percent => TokenKind::Percent, /* PercentEq */
				PrimitiveTokenKind::Eq => TokenKind::Eq,           /* Eq */
				PrimitiveTokenKind::Bang => TokenKind::Bang,       /* BangEq */
				PrimitiveTokenKind::LessThan => TokenKind::LessThan, /* LessThanEq */
				PrimitiveTokenKind::GreaterThan => TokenKind::GreaterThan, /* GreaterThanEq */

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
							Err(err) => todo!("Parse <i64> error: `{err}` in: {}", self.src_from_span(&span)),
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
							Err(err) => todo!("Parse <f64> error: {err}"),
						},
					)),
					string_or_char => {
						// TODO Report errors properly!
						if !string_or_char.is_terminated() {
							todo!("Unterminated literal: {string_or_char:?}");
						}
						let string = match unquote(self.src_from_span(&span)) {
							Ok(string) => string,
							Err(err) => todo!("Unquote error: {err}"),
						};
						if let PrimitiveLitKind::String { .. } = string_or_char {
							TokenKind::Literal(LitKind::String(SmolStr::new(string)))
						} else {
							TokenKind::Literal(LitKind::Char(match string.parse::<char>() {
								Ok(char) => char,
								Err(err) => todo!("Parse <char> error: `{err}` in: {string}"),
							}))
						}
					}
				},

				PrimitiveTokenKind::BlockComment { terminated } if !terminated => return Some(Err(span)),
				PrimitiveTokenKind::Unkown
				| PrimitiveTokenKind::At
				| PrimitiveTokenKind::Caret
				| PrimitiveTokenKind::Dollar
				| PrimitiveTokenKind::Pound
				| PrimitiveTokenKind::Tilde
				| PrimitiveTokenKind::Question => return Some(Err(span)),
				_ => continue,
			};

			return Some(Ok(Token::new(new_kind, span)));
		}
		None
	}
}
