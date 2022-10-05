use crate::ast::Literal;

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

	Print,

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
	//
	("print", TokenType::Print),
];

#[derive(Debug, Clone)]
pub struct Token<'src> {
	pub typ: TokenType,
	pub literal: Option<Literal<'src>>,
	pub start: usize,
}

impl<'src> Token<'src> {
	pub fn new(typ: TokenType, start: usize) -> Self {
		Self {
			typ,
			literal: None,
			start,
		}
	}

	pub fn literal(typ: TokenType, literal: Literal<'src>, start: usize) -> Self {
		Self {
			typ,
			literal: Some(literal),
			start,
		}
	}
}
