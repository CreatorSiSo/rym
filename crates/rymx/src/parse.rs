use rymx::Span;

struct Node {
	pub span: Span<u32>,
	pub value: Value,
}

enum Value {
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
	VSpace,
	HSpace,
	Comment,
	DocComment,

	// keywords
	/// `not`
	Not,
	/// `self`
	LowerSelf,
	/// `Self`
	UpperSelf,

	/// `&`
	Ampersand,
	/// `+`
	Plus,
	/// `-`
	Minus,
	/// `*`
	Star,
	/// `/`
	Slash,
	/// `#`
	Pound,
}
