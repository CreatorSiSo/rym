#[derive(Debug, PartialEq, Eq)]
pub struct PrimitiveToken {
	pub kind: PrimitiveTokenKind,
	pub len: u32,
}

impl PrimitiveToken {
	pub const fn new(kind: PrimitiveTokenKind, len: u32) -> Self {
		Self { kind, len }
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrimitiveTokenKind {
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

	Literal {
		kind: PrimitiveLitKind,
	},

	Unkown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrimitiveLitKind {
	/// `783256`, `100_000_000`
	Integer,
	/// `1.2358`, `999_999.999`
	Float,
	/// `"Hello World!"`
	String { terminated: bool },
	/// `'c'`, `'\\'`, `'\n'`
	Char { terminated: bool },
}

impl PrimitiveLitKind {
	pub const fn is_terminated(&self) -> bool {
		match self {
			PrimitiveLitKind::Integer => true,
			PrimitiveLitKind::Float => true,
			PrimitiveLitKind::String { terminated } => *terminated,
			PrimitiveLitKind::Char { terminated } => *terminated,
		}
	}
}
