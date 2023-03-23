use std::fmt::Debug;

mod visitor;
pub use visitor::MutVisitor;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
	pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Spanned<R> {
		Spanned(f(self.0), self.1)
	}

	pub fn as_ref(&self) -> Spanned<&T> {
		Spanned(&self.0, self.1.clone())
	}
}

#[derive(Clone, PartialEq, Eq)]
pub enum Item {
	Module(Spanned<Module>),
	Func(Spanned<Func>),
	Var(Spanned<Var>),
}

impl Debug for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Module(arg0) => arg0.0.fmt(f),
			Self::Func(arg0) => arg0.0.fmt(f),
			Self::Var(arg0) => arg0.0.fmt(f),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
	pub name: Spanned<String>,
	pub items: Vec<Spanned<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
	pub name: Spanned<String>,
	pub params: Vec<Spanned<String>>,
	pub rhs: Option<Spanned<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
	pub name: Spanned<String>,
	pub rhs: Spanned<Expr>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Stmt {
	Item(Spanned<Item>),
	Expr(Spanned<Expr>),
	Error,
}

impl Debug for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Stmt::Item(item) => item.0.fmt(f),
			Stmt::Expr(expr) => expr.0.fmt(f),
			Stmt::Error => f.write_str("Error"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	/// An identifier `name`, `True`, `Iterator`
	Ident(Spanned<String>),

	/// A literal `2`, `1.0`, `'c'`, `"Hello"`
	Literal(Spanned<Literal>),

	/// A record
	/// `
	/// Record {
	/// 	name: Spanned[String]?,
	/// 	fields: List[(Spanned[String], Spanned[Expr])]
	/// }
	/// `
	Record {
		name: Option<Spanned<String>>,
		fields: Vec<(Spanned<String>, Spanned<Expr>)>,
	},

	/// A group `(9 - 2) * 4`
	Group(Box<Spanned<Expr>>),

	/// A block `{ .. }`
	Block(Spanned<Vec<Spanned<Stmt>>>),

	/// A `if` expression, with optional `else` branch
	If {
		condition: Box<Spanned<Expr>>,
		then_branch: Box<Spanned<Expr>>,
		else_branch: Box<Option<Spanned<Expr>>>,
	},

	/// Conditionless loop
	Loop(Box<Spanned<Expr>>),

	Continue,
	Break(Box<Option<Spanned<Expr>>>),
	Return(Box<Option<Spanned<Expr>>>),

	/// A unary operation `!x`, `*x`
	Unary(UnaryOp, Box<Spanned<Expr>>),

	/// A binary operation `a + b`, `a * b`
	Binary(Box<Spanned<Expr>>, BinaryOp, Box<Spanned<Expr>>),

	/// A logical operation `True and False`, `a or b`
	Logical(Box<Spanned<Expr>>, LogicalOp, Box<Spanned<Expr>>),

	/// An assignment `a = 20`
	Assign(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

	/// A function call `test_fn(0, "hello")`
	Call {
		func: Box<Spanned<Expr>>,
		args: Spanned<Vec<Spanned<Expr>>>,
	},

	Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
	/// The `+` operator (addition)
	Add,
	/// The `-` operator (subtraction)
	Sub,
	/// The `*` operator (multiplication)
	Mul,
	/// The `/` operator (division)
	Div,
	/// The `%` operator (remainder)
	Rem,

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogicalOp {
	/// The `and` operator (logical and)
	And,
	/// The `or` operator (logical or)
	Or,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
	/// The `not` operator (not)
	Not,
	/// The `-` operator (negate)
	Neg,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Literal {
	Int(u64),
	Float(u64, u64),
	Char(char),
	String(String),
}

impl Debug for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Int(arg0) => f.write_fmt(format_args!("Int({arg0})")),
			Self::Float(arg0, arg1) => f.write_fmt(format_args!("Float({arg0}, {arg1})")),
			Self::Char(arg0) => f.write_fmt(format_args!("Char({arg0:?})")),
			Self::String(arg0) => f.write_fmt(format_args!("String({arg0:?})")),
		}
	}
}
