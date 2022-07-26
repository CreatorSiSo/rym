use std::fmt::{Debug, Display};
use std::ops::Range;

mod token;
mod visitor;
pub use token::{Token, TokenData, TokenType, KEYWORDS};
pub use visitor::AstVisitor;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T>(pub Span, pub T);

impl<T> Spanned<T> {
	pub fn map<F, R>(self, f: F) -> Spanned<R>
	where
		F: FnOnce(T) -> R,
	{
		Spanned(self.0, f(self.1))
	}

	pub fn as_ref(&self) -> Spanned<&T> {
		Spanned(self.0.clone(), &self.1)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
	/// Variable or function declaration
	Decl(Decl),

	/// Expr with trailing semicolon or newline
	Expr(Spanned<Expr>),

	// TODO: Is this really needed?
	/// Just a trailing semicolon
	Empty,
}

impl From<Spanned<Expr>> for Stmt {
	fn from(val: Spanned<Expr>) -> Self {
		Stmt::Expr(val)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
	/// A function declaration `fn name(param_1, param_2) { .. }`
	Fn {
		name: String,
		params: Vec<String>,
		body: Spanned<Expr>,
	},

	/// A constant binding `const a = 0`
	Const(String, Expr),

	/// A mutable binding `mut b = "hi"`
	Mut(String, Expr),
}

pub type Block = Vec<Spanned<Stmt>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	/// An array `[a, b, c, d]`
	Array(Vec<Expr>),

	/// A tuple `(a, b, c, d)`
	Tuple(Vec<Expr>),

	/// A `if` expression, an optional `else` block.
	///
	/// `if expr { then_block } else { else_block }`
	If(Box<Spanned<Expr>>, Spanned<Block>, Option<Spanned<Block>>),

	/// A while loop `while expr { block }`
	While(Box<Expr>, Block),

	/// Conditionless loop (can be exited with `break`, `continue`, or `return`)
	///
	/// `loop { block }`
	Loop(Spanned<Block>),

	/// A block `{ .. }`
	Block(Spanned<Block>),

	/// A `break`, with an optional expression
	Break(Option<Box<Spanned<Expr>>>),

	/// A `continue`
	Continue,

	/// A `return`, with an optional value to be returned
	Return(Box<Spanned<Expr>>),

	/// An assignment `a = 20`
	Assign(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

	/// A logical operation `true && false`, `a || b`
	Logical(Box<Spanned<Expr>>, LogicalOp, Box<Spanned<Expr>>),

	/// A binary operation `a + b`, `a * b`
	Binary(Box<Spanned<Expr>>, BinaryOp, Box<Spanned<Expr>>),

	/// A unary operation `!x`, `*x`
	Unary(UnaryOp, Box<Spanned<Expr>>),

	/// A function call `test_fn(0, "hello")`
	Call {
		callee: Box<Spanned<Expr>>,
		args: Vec<Spanned<Expr>>,
	},

	/// `(9 - 2) * 4`
	Group(Box<Spanned<Expr>>),

	/// A literal `true`, `2`, `"Hello"`
	Literal(Literal),

	/// A literal `true`, `2`, `"Hello"`
	Identifier(String),
}

impl From<Literal> for Expr {
	fn from(val: Literal) -> Self {
		Expr::Literal(val)
	}
}

impl From<String> for Expr {
	fn from(val: String) -> Self {
		Expr::Identifier(val)
	}
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
	/// The `!` operator (not)
	Not,
	/// The `-` operator (negate)
	Neg,
}

impl Display for UnaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnaryOp::Not => f.write_str("!"),
			UnaryOp::Neg => f.write_str("-"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Unit,
	Bool(bool),
	Number(f64),
	String(String),
}

impl From<()> for Literal {
	fn from(_: ()) -> Self {
		Literal::Unit
	}
}

impl From<bool> for Literal {
	fn from(val: bool) -> Self {
		Literal::Bool(val)
	}
}

impl From<f64> for Literal {
	fn from(val: f64) -> Self {
		Literal::Number(val)
	}
}

impl From<&str> for Literal {
	fn from(val: &str) -> Self {
		Literal::String(val.into())
	}
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
	pub const fn to_type_string(&self) -> &str {
		match self {
			Literal::Unit => "()",
			Literal::Bool(_) => "bool",
			Literal::Number(_) => "number",
			Literal::String(_) => "string",
		}
	}
}
