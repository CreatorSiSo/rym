#![cfg(test)]

use super::BuildTree;
use rym_errors::Diagnostic;
use rym_span::Span;
use rym_tt::{Token, TokenKind, TokenTree};
use smol_str::SmolStr;

fn map_ts_errs(src: &str) -> (Vec<TokenTree>, Vec<Diagnostic>) {
	BuildTree::new(src).fold((vec![], vec![]), |mut accum, result| {
		match result {
			Ok(tt) => accum.0.push(tt),
			Err(err) => accum.1.push(err),
		};
		accum
	})
}

#[track_caller]
fn assert_ts_errs(src: &str, ts: Vec<TokenTree>, errs: Vec<Diagnostic>) {
	assert_eq!(map_ts_errs(src), (ts, errs));
}

#[test]
fn something() {
	assert_ts_errs(
		"test",
		vec![TokenTree::Token(Token::new(TokenKind::Ident(SmolStr::new("test")), Span::new(0, 4)))],
		vec![],
	)
}
