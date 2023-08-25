use std::fmt::{Debug, Display};

use logos::{Lexer, Logos};
use rymx::Span;

pub fn tokenize(src: &str) -> Result<Vec<Token>, Span<u32>> {
	let mut result = vec![];
	for (maybe_kind, span) in TokenKind::lexer(src).spanned() {
		let span = span.try_into().expect("Internal Error: File too long");
		let kind = maybe_kind.map_err(|_| span)?;
		result.push(Token { span, kind })
	}
	Ok(result)
}

pub struct Token {
	pub kind: TokenKind,
	pub span: Span<u32>,
}

impl Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{:?}({}..{})",
			self.kind, self.span.start, self.span.end
		))
	}
}

fn line_comment(lexer: &mut Lexer<TokenKind>) {
	if let Some(new_line_index) = lexer.remainder().find("\n") {
		lexer.bump(new_line_index);
	} else {
		lexer.bump(lexer.remainder().len())
	}
}

#[derive(Debug, Logos, PartialEq)]
pub enum TokenKind {
	#[regex("(\n|\r\n)+")]
	VSpace,
	#[regex("[ \t]+")]
	HSpace,
	#[token("//", line_comment)]
	Comment,
	#[token("///", line_comment)]
	DocComment,
	#[regex("[a-zA-Z_][a-zA-Z1-9_]*")]
	Ident,
	#[regex("-?[0-9][0-9_]*")]
	Int,
	#[regex("-?[0-9][0-9_]*.[0-9_]*")]
	Float,

	#[regex(r#""([\s\S]|\\")*""#)]
	String,
	#[regex(r"'([\s\S]|\\')*'")]
	Char,

	// keywords
	#[token("as")]
	As,
	#[token("const")]
	Const,
	#[token("enum")]
	Enum,
	#[token("for")]
	For,
	#[token("let")]
	Let,
	#[token("mut")]
	Mut,
	#[token("not")]
	Not,
	#[token("self")]
	LowerSelf,
	#[token("Self")]
	UpperSelf,
	#[token("struct")]
	Struct,
	#[token("use")]
	Use,

	#[token("{")]
	BraceOpen,
	#[token("}")]
	BraceClose,
	#[token("[")]
	BracketOpen,
	#[token("]")]
	BracketClose,
	#[token("(")]
	ParenOpen,
	#[token(")")]
	ParenClose,

	#[token("&")]
	Ampersand,
	#[token("=")]
	Assign,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,
	#[token("+")]
	Plus,
	#[token("|")]
	Pipe,
	#[token("-")]
	Minus,
	#[token("*")]
	Star,
	#[token("/")]
	Slash,
	#[token("#")]
	Pound,
	#[token(";")]
	Semi,
	#[token(":")]
	Colon,

	#[token("==")]
	Eq,
	#[token("!=")]
	NotEq,
	#[token("<")]
	LessThan,
	#[token("<=")]
	LessThanEq,
	#[token(">")]
	GreaterThan,
	#[token(">=")]
	GreaterThanEq,
}
