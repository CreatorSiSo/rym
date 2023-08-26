use crate::Span;

struct Node {
	pub span: Span<u32>,
	pub kind: NodeKind,
}

enum NodeKind {
	Branch { kind: BranchKind },
	Leaf { leaf: Leaf },
}

enum BranchKind {
	Block,
	Call,
	Def,
	Fn,
	None,
	Params,
	Typed,
}

enum Leaf {
	Int(i64),
	Float(f64),
	String(String),
	Char(String),

	Ident,
	DocComment,
	Comment,
	VSpace,
	HSpace,

	As,
	Const,
	Enum,
	For,
	Let,
	Mut,
	Not,
	LowerSelf,
	UpperSelf,
	Struct,
	Use,

	BraceOpen,
	BraceClose,
	BracketOpen,
	BracketClose,
	ParenOpen,
	ParenClose,

	Ampersand,
	Assign,
	Comma,
	Dot,
	DotDot,
	Plus,
	Pipe,
	Minus,
	Star,
	Slash,
	Percent,
	Pound,
	Semi,
	Colon,

	Eq,
	NotEq,
	LessThan,
	LessThanEq,
	GreaterThan,
	GreaterThanEq,
}
