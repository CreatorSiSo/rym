use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Stmt<'src> {
	/// A const or mut binding
	Local(Local<'src>),

	/// Temporary statement for logging until functions are implemented
	Print(Expr<'src>),

	/// Expr without trailing semi-colon.
	Expr(Expr<'src>),

	/// Just a trailing semi-colon.
	Empty,
}

#[derive(Debug, PartialEq)]
pub enum Local<'src> {
	/// A constant binding `const a = 0`
	Const(&'src str, Expr<'src>),

	/// A mutable binding `mut b = "hi"`
	Mut(&'src str, Expr<'src>),
}

#[derive(Debug, PartialEq)]
pub enum Expr<'src> {
	/// An array `[a, b, c, d]`
	Array(Vec<Expr<'src>>),

	/// A tuple `(a, b, c, d)`
	Tuple(Vec<Expr<'src>>),

	/// An `if` block, with an optional `else` block.
	///
	/// `if expr { block } else { block }`
	If(Box<Expr<'src>>, Block<'src>, Option<Block<'src>>),

	/// A while loop `while expr { block }`
	While(Box<Expr<'src>>, Block<'src>),

	/// Conditionless loop (can be exited with `break`, `continue`, or `return`)
	///
	/// `loop { block }`
	Loop(Block<'src>),

	/// A block `{ .. }`
	Block(Block<'src>),

	/// A `break`, with an optional expression
	Break(Option<Box<Expr<'src>>>),

	/// A `continue`
	Continue,

	/// A `return`, with an optional value to be returned
	Return(Option<Box<Expr<'src>>>),

	/// An assignment `a = 20`
	Assign(Box<Expr<'src>>, Box<Expr<'src>>),

	/// A logical operation `true && false`, `a || b`
	Logical(Box<Expr<'src>>, LogicalOp, Box<Expr<'src>>),

	/// A binary operation `a + b`, `a * b`
	Binary(Box<Expr<'src>>, BinaryOp, Box<Expr<'src>>),

	/// A unary operation `!x`, `*x`
	Unary(UnaryOp, Box<Expr<'src>>),

	/// A function call `test_fn(0, "hello")`
	Call(Box<Expr<'src>>, Vec<Expr<'src>>),

	/// `(9 - 2) * 4`
	Group(Box<Expr<'src>>),

	/// A literal `true`, `2`, `"Hello"`
	Literal(Literal<'src>),
}

#[derive(Debug, PartialEq)]
pub struct Block<'src> {
	pub stmts: Vec<Stmt<'src>>,
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
pub enum Literal<'src> {
	Unit,
	Bool(bool),
	Number(f64),
	String(String),
	Identifier(&'src str),
}

impl<'src> Display for Literal<'src> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::Unit => write!(f, "()"),
			Literal::Bool(value) => write!(f, "{value}"),
			Literal::Number(value) => write!(f, "{value}"),
			Literal::String(value) => write!(f, "{value}"),
			Literal::Identifier(name) => write!(f, "{name}"),
		}
	}
}

impl<'src> Literal<'src> {
	pub fn to_type_string(&self) -> String {
		match self {
			Literal::Unit => "()".into(),
			Literal::Bool(_) => "bool".into(),
			Literal::Number(_) => "number".into(),
			Literal::String(_) => "string".into(),
			Literal::Identifier(_) => "identifier".into(),
		}
	}
}
