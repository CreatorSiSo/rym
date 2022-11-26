use super::LinearLexer;
use rym_errors::{Diagnostic, Handler, Level};
use rym_span::Span;
use rym_tt::{DelimSpan, Token, TokenKind, TokenTree};

#[derive(Debug)]
pub struct TreeLexer<'a> {
	/// Previous lexer stage that creates the tokens.
	lexer: LinearLexer<'a>,
	/// Global struct to submit diagnostics to.
	handler: &'a Handler,
}

impl<'a> TreeLexer<'a> {
	pub fn new(src: &'a str, handler: &'a Handler) -> Self {
		Self { lexer: LinearLexer::new(src, handler), handler }
	}

	pub fn is_next_newline(&self) -> bool {
		self.lexer.is_next_newline()
	}

	fn bump(&mut self) -> Option<Token> {
		self.lexer.next()
	}

	fn next_tree(&mut self, outer_token: Token) -> TokenTree {
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
					let mut tokens = vec![];
					let mut unclosed = true;

					while let Some(inner_token) = self.bump() {
						match inner_token.kind {
							TokenKind::OpenDelim(..) => tokens.push(self.next_tree(inner_token)),
							TokenKind::CloseDelim(close_delim) if close_delim == open_delim => {
								close_span = inner_token.span;
								unclosed = false;
								break;
							}
							_ => tokens.push(TokenTree::Token(inner_token)),
						}
					}

					if unclosed {
						match tokens.last() {
							Some(TokenTree::Token(token)) => close_span = token.span,
							Some(TokenTree::Delimited(delim_span, ..)) => close_span = delim_span.close,
							None => (),
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
						tokens,
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
		match self.bump() {
			Some(token) => Some(self.next_tree(token)),
			None => None,
		}
	}
}
