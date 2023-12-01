use std::fmt::Debug;

use crate::Span;
use logos::{Lexer, Logos};

pub fn tokenizer(src: &str) -> impl Iterator<Item = (Result<Token, ()>, Span)> + '_ {
	Token::lexer(src)
		.spanned()
		.map(|(maybe_token, span)| (maybe_token, span.into()))
}

fn line_comment(lexer: &mut Lexer<Token>) {
	if let Some(new_line_index) = lexer.remainder().find('\n') {
		lexer.bump(new_line_index);
	} else {
		lexer.bump(lexer.remainder().len())
	}
}

#[derive(Debug, Clone, Copy, Logos, PartialEq)]
pub enum Token {
	#[regex(r"[0-9][0-9_]*")]
	Int,
	#[regex(r"[0-9][0-9_]*\.[0-9_]+")]
	Float,
	#[regex(r#""(\\"|[^"])*""#)]
	String,
	#[regex(r#"'(\\'|[^'])*'"#)]
	Char,

	#[regex("[a-zA-Z_][a-zA-Z1-9_]*")]
	Ident,
	#[token("///", line_comment)]
	DocComment,
	#[token("//", line_comment)]
	Comment,
	#[regex("(\n|\r\n)+")]
	VSpace,
	#[regex("[ \t]+")]
	HSpace,

	// keywords
	#[token("as")]
	As,
	#[token("const")]
	Const,
	#[token("else")]
	Else,
	#[token("enum")]
	Enum,
	#[token("fn")]
	Fn,
	#[token("for")]
	For,
	#[token("if")]
	If,
	#[token("impl")]
	Impl,
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
	#[token("then")]
	Then,
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
