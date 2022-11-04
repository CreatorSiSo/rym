use std::fmt::{Debug, Display};
use std::ops::Range;

mod token;
mod visitor;

pub type Span = Range<usize>;
pub type SpannedToken = Spanned<Token>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T>(pub T, pub Span);

pub use token::{Token, TokenType, KEYWORDS};
pub use visitor::AstVisitor;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
	/// Variable or function declaration
	Decl(Decl),

	/// Expr with trailing semicolon or newline
	Expr(Expr),

	/// Just a trailing semicolon
	Empty,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
	/// A function declaration `fn name(param_1, param_2) { .. }`
	Fn(String, Vec<String>, Expr),

	/// A constant binding `const a = 0`
	Const(String, Expr),

	/// A mutable binding `mut b = "hi"`
	Mut(String, Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	/// An array `[a, b, c, d]`
	Array(Vec<Expr>),

	/// A tuple `(a, b, c, d)`
	Tuple(Vec<Expr>),

	/// A `if` expression, an optional `else` block.
	///
	/// `if expr { then_block } else { else_block }`
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
	Break(Box<Option<Expr>>),

	/// A `continue`
	Continue,

	/// A `return`, with an optional value to be returned
	Return(Box<Expr>),

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

impl Expr {
	pub const fn variants() -> [&'static str; 17] {
		[
			"Array",
			"Tuple",
			"If",
			"While",
			"Loop",
			"Block",
			"Break",
			"Continue",
			"Return",
			"Assign",
			"Logical",
			"Binary",
			"Unary",
			"Call",
			"Group",
			"Literal",
			"Identifier",
		]
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Identifier {
	pub name: String,
	// TODO Replace line, col with Spanned<T>
	pub line: usize,
	pub col: usize,
}

impl Identifier {
	pub fn new(name: &str, line: usize, col: usize) -> Self {
		Self {
			name: name.into(),
			line,
			col,
		}
	}
}

impl Debug for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Identifier").field(&self.name).finish()
	}
}

#[derive(Clone, PartialEq)]
pub struct Block {
	pub stmts: Vec<Stmt>,
}

impl Block {
	pub fn new(stmts: Vec<Stmt>) -> Self {
		Self { stmts }
	}
}

impl Debug for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Block")?;
		f.debug_list().entries(self.stmts.iter()).finish()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalOp {
	/// The `&&` operator (logical and)
	And,
	/// The `||` operator (logical or)
	Or,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
