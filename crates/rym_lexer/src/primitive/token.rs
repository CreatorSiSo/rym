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
	BlockComment {
		terminated: bool,
	},

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
	Pipe,
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
	/// `~`
	Tilde,
	/// `?`
	Question,
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

	/// `783256`, `100_000_000`
	Integer,
	/// `1.2358`, `999_999.999`
	Float,
	/// `"Hello World!"`
	String {
		terminated: bool,
	},
	/// `'c'`, `'\\'`, `'\n'`
	Char {
		terminated: bool,
	},

	Unkown,
}
