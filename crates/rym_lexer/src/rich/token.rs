use rym_span::Span;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct LexerError {
	span: Span,
	msg: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Token {
	// Punctuation
	/// `;`
	Semi,
	/// `:`
	Colon,
	/// `::`
	ColonColon,
	/// `.`
	Dot,
	/// `..`
	DotDot,
	/// `,`
	Comma,

	// Operator like
	/// `|`
	Or,
	/// `||`
	OrOr,
	/// `&`
	And,
	/// `&&`
	AndAnd,
	/// `+`
	Plus,
	/// `+=`
	PlusEq,
	/// `-`
	Minus,
	/// `-=`
	MinusEq,
	/// `*`
	Star,
	/// `*=`
	StarEq,
	/// `/`
	Slash,
	/// `/=`
	SlashEq,
	/// `%`
	Percent,
	/// `%=`
	PercentEq,
	/// `!`
	Bang,
	/// `!=`
	BangEq,
	/// `=`
	Eq,
	/// `==`
	EqEq,
	/// `<`
	LessThan,
	/// `<=`
	LessThanEq,
	/// `>`
	GreaterThan,
	/// `>=`
	GreaterThanEq,
	/// `->`
	ThinArrow,
	/// `=>`
	FatArrow,

	// Keywords
	/// `const`
	Const,
	/// `else`
	Else,
	/// `enum`
	Enum,
	/// `for`
	For,
	/// `func`
	Func,
	/// `if`
	If,
	/// `impl`
	Impl,
	/// `in`
	In,
	/// `loop`
	Loop,
	/// `mod`
	Mod,
	/// `mut`
	Mut,
	/// `struct`
	Struct,
	/// `trait`
	Trait,
	/// `type`
	Type,
	/// `use`
	Use,
	/// `while`
	While,

	/// Opening Delimiter token.
	/// `(`
	OpenParen,
	/// `{`
	OpenBrace,
	/// `[`
	OpenBracket,

	/// Closing Delimiter token.
	/// `)`
	CloseParen,
	/// `}`
	CloseBrace,
	/// `]`
	CloseBracket,

	/// Indentifier token: `some_thing`, `test`
	Ident(String),

	/// `36_254`
	Int(i64),
	/// `0.2346`
	Float(u32, u32),
	/// `'a'`, `'\n'`
	Char(char),
	/// `"abcde"`, `"Hello World!\n"`
	String(String),

	/// `\n`
	Newline,
	/// Well thats where it ends...
	Eof,
}

impl Token {
	pub const fn is_newline(&self) -> bool {
		matches!(self, Token::Newline)
	}
}

const KEYWORDS_LEN: usize = 16;
pub const KEYWORDS_MAP: ([&str; KEYWORDS_LEN], [Token; KEYWORDS_LEN]) = (
	[
		// a
		// b
		"const", // c
		// d
		"else", // e
		"enum", // e
		"for", "func", // f
		// g
		// h
		"if", "impl", "in", // i
		// j
		// k
		"loop", // l
		"mod", "mut", // m
		// n
		// o
		// p
		// q
		// r
		"struct", // s
		"trait", "type", // t
		"use",  // u
		// v
		"while", // w
		         // x
		         // y
		         // z
	],
	[
		Token::Const,
		Token::Else,
		Token::Enum,
		Token::For,
		Token::Func,
		Token::If,
		Token::Impl,
		Token::In,
		Token::Loop,
		Token::Mod,
		Token::Mut,
		Token::Struct,
		Token::Trait,
		Token::Type,
		Token::Use,
		Token::While,
	],
);
