use rym_span::Span;

#[derive(Debug, PartialEq)]
pub struct LexerError {
	span: Span,
	msg: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
	Pipe,
	/// `||`
	PipePipe,
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
	Break,
	Const,
	Continue,
	Do,
	Else,
	Enum,
	For,
	Func,
	If,
	Impl,
	In,
	Let,
	Loop,
	Mod,
	Mut,
	Return,
	Struct,
	Then,
	Trait,
	Type,
	Use,
	While,

	/// Delimiter token.
	/// `(`
	OpenParen,
	/// `)`
	CloseParen,
	/// `{`
	OpenBrace,
	/// `}`
	CloseBrace,
	/// `[`
	OpenBracket,
	/// `]`
	CloseBracket,

	/// Indentifier token: `some_thing`, `test`
	Ident(String),

	/// `36_254`
	Int(u64),
	/// `0.2346`
	Float(u64, u64),
	/// `'a'`, `'\n'`
	Char(char),
	/// `"abcde"`, `"Hello World!\n"`
	String(String),

	/// Well thats where it ends...
	Eof,
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut tmp = String::new();
		fn hoist_up(tmp: &mut String, data: String) -> &str {
			*tmp = data;
			&*tmp
		}

		let str = match self {
			Token::Semi => ";",
			Token::Colon => ":",
			Token::ColonColon => "::",
			Token::Dot => ".",
			Token::DotDot => "..",
			Token::Comma => ",",
			Token::Pipe => "|",
			Token::PipePipe => "||",
			Token::And => "&",
			Token::AndAnd => "&&",
			Token::Plus => "+",
			Token::PlusEq => "+=",
			Token::Minus => "-",
			Token::MinusEq => "-=",
			Token::Star => "*",
			Token::StarEq => "*=",
			Token::Slash => "/",
			Token::SlashEq => "/=",
			Token::Percent => "%",
			Token::PercentEq => "%=",
			Token::Bang => "!",
			Token::BangEq => "!=",
			Token::Eq => "=",
			Token::EqEq => "==",
			Token::LessThan => "<",
			Token::LessThanEq => "<=",
			Token::GreaterThan => ">",
			Token::GreaterThanEq => ">=",
			Token::ThinArrow => "->",
			Token::FatArrow => "=>",

			Token::Break => "break",
			Token::Const => "const",
			Token::Continue => "continue",
			Token::Do => "do",
			Token::Else => "else",
			Token::Enum => "enum",
			Token::For => "for",
			Token::Func => "func",
			Token::If => "if",
			Token::Impl => "impl",
			Token::In => "in",
			Token::Let => "let",
			Token::Loop => "loop",
			Token::Mod => "mod",
			Token::Mut => "mut",
			Token::Return => "return",
			Token::Struct => "struct",
			Token::Then => "then",
			Token::Trait => "trait",
			Token::Type => "type",
			Token::Use => "use",
			Token::While => "while",

			Token::OpenParen => "(",
			Token::CloseParen => ")",

			Token::OpenBrace => "{",
			Token::CloseBrace => "",

			Token::OpenBracket => "[",
			Token::CloseBracket => "]",

			Token::Ident(name) => name,
			Token::Int(inner) => hoist_up(&mut tmp, inner.to_string()),
			Token::Float(lhs, rhs) => hoist_up(&mut tmp, format!("{lhs}.{rhs}")),
			Token::Char(inner) => hoist_up(&mut tmp, inner.to_string()),
			Token::String(inner) => inner,
			Token::Eof => "<eof>",
		};
		f.write_str(str)
	}
}

// TODO implement and use ConstMap<&'static str, Token, 22>
const KEYWORDS_LEN: usize = 22;
pub const KEYWORDS_MAP: ([&str; KEYWORDS_LEN], [Token; KEYWORDS_LEN]) = (
	[
		// a
		"break", // b
		"const", "continue", // c
		"do",       // d
		"else",     // e
		"enum",     // e
		"for", "func", // f
		// g
		// h
		"if", "impl", "in", // i
		// j
		// k
		"let", "loop", // l
		"mod", "mut", // m
		// n
		// o
		// p
		// q
		"return", // r
		"struct", // s
		"then", "trait", "type", // t
		"use",  // u
		// v
		"while", // w
		         // x
		         // y
		         // z
	],
	[
		Token::Break,
		Token::Const,
		Token::Continue,
		Token::Do,
		Token::Else,
		Token::Enum,
		Token::For,
		Token::Func,
		Token::If,
		Token::Impl,
		Token::In,
		Token::Let,
		Token::Loop,
		Token::Mod,
		Token::Mut,
		Token::Return,
		Token::Struct,
		Token::Then,
		Token::Trait,
		Token::Type,
		Token::Use,
		Token::While,
	],
);
