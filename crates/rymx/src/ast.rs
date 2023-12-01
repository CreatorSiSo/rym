use crate::interpret::Function;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Module {
	pub name: String,
	pub constants: Vec<(String, Expr)>,
	pub children: Vec<Module>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableKind {
	Const,
	Let,
	LetMut,
}

impl Display for VariableKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			VariableKind::Const => "const",
			VariableKind::Let => "let",
			VariableKind::LetMut => "let mut",
		})
	}
}

#[derive(Clone, PartialEq)]
pub enum Expr {
	Unit,
	Literal(Literal),
	Ident(String),
	Function(Function),

	Unary(UnaryOp, Box<Expr>),
	Binary(BinaryOp, Box<Expr>, Box<Expr>),
	Call(Box<Expr>, Vec<Expr>),

	IfElse(
		/// Condition
		Box<Expr>,
		/// Then branch
		Box<Expr>,
		/// Else branch
		Box<Expr>,
	),
	Block(Vec<Expr>),
	Break(Box<Expr>),
	Return(Box<Expr>),

	Var(VariableKind, String, Box<Expr>),
}

impl std::fmt::Debug for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Unit => f.write_str("Unit"),
			Self::Literal(arg0) => f.write_fmt(format_args!("Literal({arg0:?})")),
			Self::Ident(arg0) => f.write_fmt(format_args!("Ident({arg0:?})")),
			Self::Function(arg0) => f.write_fmt(format_args!("{arg0:#?}")),

			Self::Unary(arg0, arg1) => f.debug_tuple(&arg0.to_string()).field(arg1).finish(),
			Self::Binary(arg0, arg1, arg2) => f
				.debug_tuple(&arg0.to_string())
				.field(arg1)
				.field(arg2)
				.finish(),
			Self::Call(arg0, arg1) => f.debug_tuple("Call").field(arg0).field(arg1).finish(),

			Self::IfElse(arg0, arg1, arg2) => f
				.debug_tuple("IfElse")
				.field(arg0)
				.field(arg1)
				.field(arg2)
				.finish(),
			Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
			Self::Break(arg0) => f.debug_tuple("Break").field(arg0).finish(),
			Self::Return(arg0) => f.debug_tuple("Return").field(arg0).finish(),

			Self::Var(arg0, arg1, arg2) => f
				.debug_tuple(&format!("{arg0:?}"))
				.field(arg1)
				.field(arg2)
				.finish(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
	/// Negation `-1`
	Neg,
	/// Not `not true`
	Not,
}

impl Display for UnaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self:?}"))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
	/// Addition `1 + 2`
	Add,
	/// Subtraction `1 - 2`
	Sub,
	/// Multiplication `1 * 2`
	Mul,
	/// Division `1 / 2`
	Div,
	/// Equality `1 == 2`
	Eq,
	/// Inequality `1 != 2`
	NotEq,
}

impl Display for BinaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self:?}"))
	}
}

#[derive(Clone, PartialEq)]
pub enum Literal {
	Bool(bool),
	Int(i64),
	Float(f64),
	String(String),
}

impl std::fmt::Debug for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(arg0) => f.write_fmt(format_args!("Bool {arg0}")),
			Self::Int(arg0) => f.write_fmt(format_args!("Int {arg0}")),
			Self::Float(arg0) => f.write_fmt(format_args!("Float {arg0}")),
			Self::String(arg0) => f.write_fmt(format_args!("String {arg0}")),
		}
	}
}
