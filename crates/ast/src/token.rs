use std::fmt::Display;

use crate::Literal;

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
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

	Newline,
	/// Do not use outside of `lex` crate!
	Eof,
}

impl Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			TokenType::Plus => "Plus",
			TokenType::Minus => "Minus",
			TokenType::Star => "Star",
			TokenType::Slash => "Slash",
			TokenType::Dot => "Dot",
			TokenType::Comma => "Comma",
			TokenType::Semicolon => "Semicolon",
			TokenType::LeftParen => "LeftParen",
			TokenType::RightParen => "RightParen",
			TokenType::LeftBrace => "LeftBrace",
			TokenType::RightBrace => "RightBrace",
			TokenType::Bang => "Bang",
			TokenType::BangEqual => "BangEqual",
			TokenType::Equal => "Equal",
			TokenType::EqualEqual => "EqualEqual",
			TokenType::Greater => "Greater",
			TokenType::GreaterEqual => "GreaterEqual",
			TokenType::Less => "Less",
			TokenType::LessEqual => "LessEqual",
			TokenType::DoubleAmpersand => "DoubleAmpersand",
			TokenType::DoublePipe => "DoublePipe",
			TokenType::Int => "Int",
			TokenType::Number => "Number",
			TokenType::Char => "Char",
			TokenType::String => "String",
			TokenType::Identifier => "Identifier",
			TokenType::If => "If",
			TokenType::Else => "Else",
			TokenType::For => "For",
			TokenType::While => "While",
			TokenType::Loop => "Loop",
			TokenType::Return => "Return",
			TokenType::Break => "Break",
			TokenType::Continue => "Continue",
			TokenType::False => "False",
			TokenType::True => "True",
			TokenType::Fn => "Fn",
			TokenType::Const => "Const",
			TokenType::Mut => "Mut",
			TokenType::Struct => "Struct",
			TokenType::Self_ => "Self",
			TokenType::Newline => "Newline",
			TokenType::Eof => "Eof",
		})
	}
}

pub const LIT_TOKEN_TYPES: [TokenType; 4] = [
	TokenType::String,
	TokenType::Number,
	TokenType::True,
	TokenType::False,
];

pub const KEYWORDS: &[(&str, TokenType)] = &[
	("if", TokenType::If),
	("else", TokenType::Else),
	("for", TokenType::For),
	("while", TokenType::While),
	("loop", TokenType::Loop),
	("return", TokenType::Return),
	("break", TokenType::Break),
	("continue", TokenType::Continue),
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
	pub data: TokenData,
}

impl Token {
	pub const fn new(typ: TokenType) -> Self {
		Self {
			typ,
			data: TokenData::None,
		}
	}

	pub fn literal(typ: TokenType, literal: Literal) -> Self {
		Self {
			typ,
			data: TokenData::Literal(literal),
		}
	}

	pub fn ident(typ: TokenType, ident: String) -> Self {
		Self {
			typ,
			data: TokenData::Identifier(ident),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
	None,
	Literal(Literal),
	Identifier(String),
}

impl TokenData {
	pub fn lit(self, typ: TokenType) -> Literal {
		if LIT_TOKEN_TYPES.contains(&typ) {
			match self {
				TokenData::Literal(lit) => lit,
				TokenData::Identifier(ident) => {
					panic!(
						"Internal Error: Literal token should have data got `TokenData::Identifier({ident:?})`"
					)
				}
				TokenData::None => {
					panic!("Internal Error: Literal token should have data got `TokenData::None`")
				}
			}
		} else {
			panic!("Internal Error: Expected literal token got {self:?}")
		}
	}

	pub fn ident(self, typ: TokenType) -> String {
		if typ == TokenType::Identifier {
			match self {
				TokenData::Identifier(ident) => ident,
				TokenData::Literal(lit) => panic!(
					"Internal Error: Identifier token should have data got `TokenData::Literal({lit:?})`"
				),
				TokenData::None => {
					panic!("Internal Error: Identifier token should have data got `TokenData::None`")
				}
			}
		} else {
			panic!("Internal Error: Expected identifier token got {self:?}")
		}
	}
}
