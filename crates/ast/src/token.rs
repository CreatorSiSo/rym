use crate::{Identifier, Literal};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
	Plus,
	Minus,
	Star,
	Slash,

	Dot,
	Comma,
	Semicolon,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,

	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	DoubleAmpersand,
	DoublePipe,

	Int,
	Number,
	Char,
	String,
	Identifier,

	If,
	Else,
	For,
	While,
	Loop,
	Return,
	Break,
	Continue,

	False,
	True,

	Fn,
	Const,
	Mut,

	Struct,
	Self_,

	Eof,
}

pub const KEYWORDS: &[(&str, TokenType)] = &[
	("if", TokenType::If),
	("else", TokenType::Else),
	("for", TokenType::For),
	("while", TokenType::While),
	("loop", TokenType::Loop),
	("return", TokenType::Return),
	("break", TokenType::Break),
	//
	("false", TokenType::False),
	("true", TokenType::True),
	//
	("fn", TokenType::Fn),
	("const", TokenType::Const),
	("mut", TokenType::Mut),
	//
	("struct", TokenType::Struct),
	("self", TokenType::Self_),
];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub typ: TokenType,
	pub literal: Option<Literal>,
	pub ident: Option<Identifier>,
}

impl Token {
	pub fn new(typ: TokenType) -> Self {
		Self {
			typ,
			literal: None,
			ident: None,
		}
	}

	pub fn literal(typ: TokenType, literal: Literal) -> Self {
		Self {
			typ,
			literal: Some(literal),
			ident: None,
		}
	}

	pub fn ident(typ: TokenType, ident: Identifier) -> Self {
		Self {
			typ,
			literal: None,
			ident: Some(ident),
		}
	}
}
