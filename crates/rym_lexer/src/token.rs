#[derive(Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenKind,
	pub len: u32,
}

impl Token {
	pub const fn new(kind: TokenKind, len: u32) -> Self {
		Self { kind, len }
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
	/// `// comment`
	LineComment,
	/// `/* comment */`
	BlockComment,

	/// Any whitespace character sequence.
	Whitespace,

	// Punctuation tokens.
	/// `;`
	Semi,
	/// `:`
	Colon,
	/// `,`
	Comma,
	/// `.`
	Dot,

	// Used one character tokens.
	/// `|`
	Or,
	/// `&`
	And,
	/// `+`
	Plus,
	/// `-`
	Minus,
	/// `*`
	Star,
	/// `/`
	Slash,
	/// `%`
	Percent,
	/// `=`
	Eq,
	/// `!`
	Bang,

	// Currently unused one character tokens.
	/// `@`
	At,
	/// `^`
	Caret,
	/// `$`
	Dollar,
	/// `#`
	Pound,

	// Delimiter like tokens.
	/// `<`
	LessThan,
	/// `>`
	GreaterThan,
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

	Ident,

	Literal {
		kind: LiteralKind,
	},

	Unkown,

	Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralKind {
	String { terminated: bool },
	Char { terminated: bool },
	Number,
}
