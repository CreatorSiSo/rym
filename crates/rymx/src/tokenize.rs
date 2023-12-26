use crate::Span;
use logos::{Lexer, Logos};
use std::fmt::Debug;

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

#[derive(logos_display::Display, Debug, Clone, Copy, Logos, PartialEq)]
pub enum Token {
	#[regex(r"[0-9][0-9_]*")]
	Int,
	#[regex(r"[0-9][0-9_]*\.[0-9_]+")]
	Float,
	#[regex(r#"'(\\'|[^'])*'"#)]
	// Char,
	#[regex(r#""(\\"|[^"])*""#)]
	String,

	#[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
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
	#[token("break")]
	Break,
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
	#[token("return")]
	Return,
	#[token("struct")]
	Struct,
	#[token("then")]
	Then,
	#[token("type")]
	Type,
	#[token("union")]
	Union,
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
	#[token("->")]
	ThinArrow,
	#[token("=>")]
	ThickArrow,

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

impl Token {
	/// Extends the derived Display implementation
	pub fn display(&self) -> String {
		match self {
			Self::Int => "integer".into(),
			Self::Float => "float".into(),
			// Self::Char => "character".into(),
			Self::String => "string".into(),

			Self::Ident => "identifier".into(),

			Self::DocComment => "doc comment".into(),
			Self::Comment => "comment".into(),
			Self::VSpace | Self::HSpace => "whitespace".into(),

			token => format!("`{}`", token.to_string()),
		}
	}
}
