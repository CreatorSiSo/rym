#[derive(Debug)]
pub enum Stmt {
	/// A const or mut binding
	Local(Local),

	/// Expr without trailing semi-colon.
	Expr(Expr),

	/// Just a trailing semi-colon.
	Empty,
}

#[derive(Debug)]
pub enum Local {
	/// A constant binding `const a = 0`
	Const(Expr),

	/// A mutable binding `mut b = "hi"`
	Mut(Expr),
}

#[derive(Debug)]
pub enum Expr {
	/// An array `[a, b, c, d]`
	Array(Vec<Expr>),

	/// A tuple `(a, b, c, d)`
	Tuple(Vec<Expr>),

	/// An `if` block, with an optional `else` block.
	///
	/// `if expr { block } else { block }`
	If(Box<Expr>, Block, Option<Block>),

	/// A while loop `while expr { block }`
	While(Box<Expr>, Block),

	/// Conditionless loop (can be exited with `break`, `continue`, or `return`)
	///
	/// `loop { block }`
	Loop(Block),

	/// A block `{ .. }`
	Block(Block),

	/// A `break`, with an optional expression
	Break(Option<Box<Expr>>),

	/// A `continue`
	Continue,

	/// A `return`, with an optional value to be returned
	Return(Option<Box<Expr>>),

	/// An assignment `a = 20`
	Assign(Box<Expr>, Box<Expr>),

	/// A binary operation `a + b`, `a * b`
	Binary(Box<Expr>, BinaryOp, Box<Expr>),

	/// A unary operation `!x`, `*x`
	Unary(UnaryOp, Box<Expr>),

	/// `(9 - 2) * 4`
	Group(Box<Expr>),

	/// A literal `true`, `2`, `"Hello"`
	Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
	/// `true` or `false`
	Bool(bool),

	/// A 64-bit floating point number
	Number(f64),

	/// TODO
	String(String),
	/// TODO
	Identifier(String),
}

#[derive(Debug)]
pub struct Block {
	pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum BinaryOp {
	/// The `+` operator (addition)
	Add,
	/// The `-` operator (subtraction)
	Sub,
	/// The `*` operator (multiplication)
	Mul,
	/// The `/` operator (division)
	Div,
	/// The `%` operator (modulus)
	Mod,

	/// The `==` operator (equal)
	Eq,
	/// The `!=` operator (not equal)
	Ne,
	/// The `>` operator (greater than)
	Gt,
	/// The `>=` operator (greater than or equal)
	Ge,
	/// The `<` operator (less than)
	Lt,
	/// The `<=` operator (less than or equal)
	Le,

	/// The `and` operator (logical and)
	And,
	/// The `or` operator (logical or)
	Or,
}

#[derive(Debug)]
pub enum UnaryOp {
	/// The `!` operator (not)
	Not,
	/// The `-` operator (negate)
	Neg,
}
