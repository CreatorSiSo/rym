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
		for result in self.tokens.by_ref() {
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

	pub fn is_next_newline(&self) -> bool {
		self.tokens.is_next_newline()
	}

	fn next_tree(&mut self, outer_token: Token) -> Option<Result<TokenTree, Diagnostic>> {
		let tt = match outer_token.kind {
			TokenKind::OpenDelim(open_delim) => {
				let open = outer_token.span;
				let mut close = open;
				let mut tokens = vec![];
				// TODO Make nesting work
				while let Some(inner_token) = self.bump() {
					match inner_token.kind {
						TokenKind::OpenDelim(..) => return self.next_tree(inner_token),
						TokenKind::CloseDelim(close_delim) if open_delim == close_delim => {
							close = inner_token.span;
							break;
						}
						_ => tokens.push(TokenTree::Token(inner_token)),
					}
				}
				return Some(Ok(TokenTree::Delimited(
					DelimSpan { open, close, entire: Span::new(open.start, close.end) },
					open_delim,
					tokens,
				)));
			}
			TokenKind::CloseDelim(_) => {
				return Some(Err(Diagnostic::new_spanned(
					Level::Error,
					"Unexpected closing delimiter",
					outer_token.span,
				)))
			}
			_ => TokenTree::Token(outer_token),
		};
		Some(Ok(tt))
	}
}

impl<'a> Iterator for BuildTree<'a> {
	type Item = Result<TokenTree, Diagnostic>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.bump() {
			Some(token) => self.next_tree(token),
			None => None,
		}
	}
}
