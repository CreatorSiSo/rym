use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use rym_tt::{DelimSpan, Token, TokenKind, TokenStream, TokenTree};

use super::linear::LinearLexer;

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

	fn bump(&mut self) -> Option<Token> {
		self.linear.next()
	}

	fn next_tt(&mut self, outer_token: Token) -> TokenTree {
		loop {
			match outer_token.kind {
				TokenKind::CloseDelim(_) => {
					self.handler.emit(Diagnostic::new_spanned(
						Level::Error,
						"Unexpected closing delimiter",
						outer_token.span,
					));
					continue;
				}
				TokenKind::OpenDelim(open_delim) => {
					let open_span = outer_token.span;
					let mut close_span = open_span;
					let mut token_stream = vec![];
					let mut unclosed = true;

					while let Some(inner_token) = self.bump() {
						let tt = match inner_token.kind {
							TokenKind::OpenDelim(..) => self.next_tt(inner_token),
							TokenKind::CloseDelim(close_delim) if close_delim == open_delim => {
								close_span = inner_token.span;
								unclosed = false;
								break;
							}
							_ => TokenTree::Token(inner_token),
						};
						token_stream.push(tt);
					}

					if unclosed {
						if let Some(tt) = token_stream.last() {
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

					break TokenTree::Delimited(
						DelimSpan {
							open: open_span,
							close: close_span,
							entire: Span::new(open_span.start, close_span.end),
						},
						open_delim,
						TokenStream(token_stream),
					);
				}
				_ => break TokenTree::Token(outer_token),
			}
		}
	}
}

impl<'a> Iterator for TreeLexer<'a> {
	type Item = TokenTree;

	fn next(&mut self) -> Option<Self::Item> {
		self.bump().map(|token| self.next_tt(token))
	}
}
