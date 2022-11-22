use super::BuildLinear;
use rym_errors::{Diagnostic, Level};
use rym_span::Span;
use rym_tt::{DelimSpan, Token, TokenKind, TokenTree};

pub struct BuildTree<'a> {
	/// Source text to tokenize.
	src: &'a str,
	/// Cursor for getting lexer tokens.
	tokens: BuildLinear<'a>,
	/// Saving all diagnostics to return them afterwards
	diagnostics: Vec<Diagnostic>,
}

impl<'a> BuildTree<'a> {
	pub fn new(src: &'a str) -> Self {
		Self { src, tokens: BuildLinear::new(src), diagnostics: vec![] }
	}

	fn bump(&mut self) -> Option<Token> {
		while let Some(result) = self.tokens.next() {
			match result {
				Ok(token) => return Some(token),
				Err(diagnostic) => {
					self.diagnostics.push(diagnostic);
					continue;
				}
			}
		}
		None
	}

	fn peek(&self) -> Option<Token> {
		while let Some(result) = self.tokens.clone().next() {
			match result {
				Ok(token) => return Some(token),
				_ => continue,
			}
		}
		None
	}
}

impl<'a> Iterator for BuildTree<'a> {
	type Item = Result<TokenTree, Diagnostic>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(outer_token) = self.bump() {
			let tt = match outer_token.kind {
				TokenKind::Newline => todo!(),

				TokenKind::OpenDelim(open_delim) => {
					let open = outer_token.span;
					let mut close = open;
					let mut tokens = vec![];
					// TODO Make nesting work
					while let Some(inner_token) = self.bump() {
						match inner_token.kind {
							TokenKind::CloseDelim(close_delim) if open_delim == close_delim => {
								close = inner_token.span;
								break;
							}
							_ => tokens.push(TokenTree::Token(inner_token)),
						}
					}
					TokenTree::Delimited(
						DelimSpan { open, close, entire: Span::new(open.start, close.end) },
						open_delim,
						tokens,
					)
				}
				TokenKind::CloseDelim(_) => {
					return Some(Err(Diagnostic::new_spanned(
						Level::Error,
						"Unexpected closing delimiter",
						outer_token.span,
					)))
				}
				// TODO TokenKind::LessThan => todo!(),
				// TODO TokenKind::GreaterThan => todo!(),
				_ => TokenTree::Token(outer_token),
			};
			return Some(Ok(tt));
		}
		None
	}
}
