use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use rym_tt::{DelimSpan, Token, TokenKind, TokenStream, TokenTree, WrappedTt};

use super::linear::{LinearLexer, LinearToken, LinearTokenKind};

#[derive(Debug, Clone)]
pub struct TreeLexer<'a> {
	/// Previous lexer stage that creates the tokens.
	linear: LinearLexer<'a>,
	/// Global struct to submit diagnostics to.
	handler: &'a Handler,
}

impl<'a> TreeLexer<'a> {
	pub fn new(src: &'a str, handler: &'a Handler) -> Self {
		Self { linear: LinearLexer::new(src, handler), handler }
	}

	fn bump(&mut self) -> Option<LinearToken> {
		self.linear.next()
	}

	fn next_tt(&mut self, outer_token: LinearToken) -> WrappedTt {
		loop {
			match outer_token.kind {
				LinearTokenKind::CloseDelim(_) => {
					self.handler.emit(Diagnostic::new_spanned(
						Level::Error,
						"Unexpected closing delimiter",
						outer_token.span,
					));
					continue;
				}
				LinearTokenKind::OpenDelim(open_delim) => {
					let open_span = outer_token.span;
					let mut close_span = open_span;
					let mut token_stream = vec![];
					let mut unclosed = true;

					while let Some(inner_token) = self.bump() {
						let tt = match inner_token.kind {
							LinearTokenKind::Newline => {
								token_stream.push(WrappedTt::Newline);
								continue;
							}
							LinearTokenKind::OpenDelim(..) => self.next_tt(inner_token),
							LinearTokenKind::CloseDelim(close_delim) if close_delim == open_delim => {
								close_span = inner_token.span;
								unclosed = false;
								break;
							}
							_ => linear_token_into_wrapped_tt(inner_token),
						};
						token_stream.push(tt);
					}

					if unclosed {
						if let Some(WrappedTt::Tt(tt)) = token_stream.last() {
							close_span = match tt {
								TokenTree::Token(token) => token.span,
								TokenTree::Delimited(delim_span, ..) => delim_span.close,
							}
						}
						self.handler.emit(Diagnostic::new_spanned(
							Level::Error,
							"Unclosed delimiter",
							Span::new(open_span.start, close_span.end),
						))
					}

					break WrappedTt::Tt(TokenTree::Delimited(
						DelimSpan {
							open: open_span,
							close: close_span,
							entire: Span::new(open_span.start, close_span.end),
						},
						open_delim,
						TokenStream(token_stream),
					));
				}
				_ => break linear_token_into_wrapped_tt(outer_token),
			}
		}
	}
}

// TODO: Benchmark if #[inline] makes a difference here
/// ## Panic
/// Panics if linear_token.kind is `LinearTokenKind::OpenDelim` | `LinearTokenKind::CloseDelim`
fn linear_token_into_wrapped_tt(LinearToken { kind, span }: LinearToken) -> WrappedTt {
	let kind = match kind {
		LinearTokenKind::Newline => return WrappedTt::Newline,
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
	WrappedTt::Tt(TokenTree::Token(Token { kind, span }))
}

impl<'a> Iterator for TreeLexer<'a> {
	type Item = WrappedTt;

	fn next(&mut self) -> Option<Self::Item> {
		self.bump().map(|token| self.next_tt(token))
	}
}
