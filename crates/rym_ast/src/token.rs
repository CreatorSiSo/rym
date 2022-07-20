#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue<'src> {
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

	Int(i64),
	Number(f64),
	Char(char), // TODO
	String(String),
	Identifier(&'src str),

	If,
	Else,
	For,
	While,
	Loop,
	Return,
	Break,

	False,
	True,
	And,
	Or,

	Fn,
	Const,
	Mut,

	Struct,
	Selff,

	Print,

	Eof,
}

pub const KEYWORDS: &[(&str, TokenValue)] = &[
	("if", TokenValue::If),
	("else", TokenValue::Else),
	("for", TokenValue::For),
	("while", TokenValue::While),
	("loop", TokenValue::Loop),
	("return", TokenValue::Return),
	("break", TokenValue::Break),
	//
	("false", TokenValue::False),
	("true", TokenValue::True),
	("and", TokenValue::And),
	("or", TokenValue::Or),
	//
	("fn", TokenValue::Fn),
	("const", TokenValue::Const),
	("mut", TokenValue::Mut),
	//
	("struct", TokenValue::Struct),
	("self", TokenValue::Selff),
	//
	("print", TokenValue::Print),
];

#[derive(Debug)]
pub struct Token<'src> {
	pub value: TokenValue<'src>,
	pub start: usize,
}

impl<'src> Token<'src> {
	pub fn new(typ: TokenValue<'src>, start: usize) -> Self {
		Self { value: typ, start }
	}
}
