use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Stmt {
	/// A const or mut binding
	Local(Local),

	/// Temporary statement for logging until functions are implemented
	Print(Expr),

	/// Expr without trailing semi-colon.
	Expr(Expr),

	/// Just a trailing semi-colon.
	Empty,
}

#[derive(Debug, PartialEq)]
pub enum Local {
	/// A constant binding `const a = 0`
	Const(String, Expr),

	/// A mutable binding `mut b = "hi"`
	Mut(String, Expr),
}

#[derive(Debug, PartialEq)]
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

	/// A logical operation `true && false`, `a || b`
	Logical(Box<Expr>, LogicalOp, Box<Expr>),

	/// A binary operation `a + b`, `a * b`
	Binary(Box<Expr>, BinaryOp, Box<Expr>),

	/// A unary operation `!x`, `*x`
	Unary(UnaryOp, Box<Expr>),

	/// A function call `test_fn(0, "hello")`
	Call(Box<Expr>, Vec<Expr>),

	/// `(9 - 2) * 4`
	Group(Box<Expr>),

	/// A literal `true`, `2`, `"Hello"`
	Literal(Literal),

	/// A literal `true`, `2`, `"Hello"`
	Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
	pub name: String,
	pub line: usize,
	pub col: usize,
}

impl Identifier {
	pub fn new(name: String, line: usize, col: usize) -> Self {
		Self { name, line, col }
	}
}

#[derive(Debug, PartialEq)]
pub struct Block {
	pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq)]
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum LogicalOp {
	/// The `&&` operator (logical and)
	And,
	/// The `||` operator (logical or)
	Or,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
	/// The `!` operator (not)
	Not,
	/// The `-` operator (negate)
	Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Unit,
	Bool(bool),
	Number(f64),
	String(String),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::Unit => write!(f, "()"),
			Literal::Bool(value) => write!(f, "{value}"),
			Literal::Number(value) => write!(f, "{value}"),
			Literal::String(value) => write!(f, "{value}"),
		}
	}
}

impl Literal {
	pub fn to_type_string(&self) -> String {
		match self {
			Literal::Unit => "()".into(),
			Literal::Bool(_) => "bool".into(),
			Literal::Number(_) => "number".into(),
			Literal::String(_) => "string".into(),
		}
	}
}
