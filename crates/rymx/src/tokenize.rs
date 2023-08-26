use std::fmt::Debug;

use crate::Span;
use logos::{Lexer, Logos};

pub fn tokenize(src: &str) -> Vec<Result<Token, Span<u32>>> {
	let mut result = vec![];
	for (maybe_kind, span) in TokenKind::lexer(src).spanned() {
		let span = span.try_into().expect("Internal Error: File too long");
		result.push(
			maybe_kind
				.map(|kind| Token { span, kind })
				.map_err(|_| span),
		);
	}
	result
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span<u32>,
}

impl Token {
	pub fn debug_string(self, src: &str) -> String {
		format!("{:?} [{}]", self.kind, self.src(src).escape_debug())
	}

	pub fn src<'src>(self, src: &'src str) -> &'src str {
		&src[self.span.as_range()]
	}
}

fn line_comment(lexer: &mut Lexer<TokenKind>) {
	if let Some(new_line_index) = lexer.remainder().find("\n") {
		lexer.bump(new_line_index);
	} else {
		lexer.bump(lexer.remainder().len())
	}
}

#[derive(Debug, Clone, Copy, Logos, PartialEq)]
pub enum TokenKind {
	#[regex(r"-?[0-9][0-9_]*")]
	Int,
	#[regex(r"-?[0-9][0-9_]*\.[0-9_]+")]
	Float,
	#[regex(r#""(\\"|[^"])*""#)]
	String,
	#[regex(r#"'(\\'|[^'])*'"#)]
	Char,

	#[regex("[a-zA-Z_][a-zA-Z1-9_]*")]
	Ident,
	#[regex("(\n|\r\n)+")]
	#[token("///", line_comment)]
	DocComment,
	#[token("//", line_comment)]
	Comment,
	VSpace,
	#[regex("[ \t]+")]
	HSpace,

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
	#[token("..")]
	DotDot,
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
	#[token("%")]
	Percent,
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
