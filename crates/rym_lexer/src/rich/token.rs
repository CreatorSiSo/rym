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
	/// `then`
	Then,
	/// `trait`
	Trait,
	/// `type`
	Type,
	/// `use`
	Use,
	/// `while`
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
		fn hoist_up<'a>(tmp: &'a mut String, data: String) -> &'a str {
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

			Token::Const => "const",
			Token::Else => "else",
			Token::Enum => "enum",
			Token::For => "for",
			Token::Func => "func",
			Token::If => "if",
			Token::Impl => "impl",
			Token::In => "in",
			Token::Loop => "loop",
			Token::Mod => "mod",
			Token::Mut => "mut",
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

const KEYWORDS_LEN: usize = 17;
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
		"then", "trait", "type", // t
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
		Token::Then,
		Token::Trait,
		Token::Type,
		Token::Use,
		Token::While,
	],
);
