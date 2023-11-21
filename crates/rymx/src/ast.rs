use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Module {
	name: String,
	constants: Vec<Constant>,
	children: Vec<Module>,
}

#[derive(Debug, Clone)]
pub struct Constant {
	name: String,
	data: Expr,
	typ: Expr,
}

#[derive(Clone)]
pub enum Expr {
	Value(Value),
	Unary(UnaryOp, Box<Expr>),
	Binary(BinaryOp, Box<Expr>, Box<Expr>),
	Ident(String),
}

impl std::fmt::Debug for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Value(arg0) => f.write_fmt(format_args!("Value({arg0:?})")),
			Self::Unary(arg0, arg1) => f.debug_tuple(&arg0.to_string()).field(arg1).finish(),
			Self::Binary(arg0, arg1, arg2) => f
				.debug_tuple(&arg0.to_string())
				.field(arg1)
				.field(arg2)
				.finish(),
			Self::Ident(arg0) => f.write_fmt(format_args!("Ident({arg0:?})")),
		}
	}
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone)]
pub enum Value {
	Bool(bool),
	Int(u64),
	Float(f64),
	String(String),
	Function(Function),
	Type(Type),
}

impl std::fmt::Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(arg0) => f.write_fmt(format_args!("Bool {arg0}")),
			Self::Int(arg0) => f.write_fmt(format_args!("Int {arg0}")),
			Self::Float(arg0) => f.write_fmt(format_args!("Float {arg0}")),
			Self::String(arg0) => f.write_fmt(format_args!("String {arg0}")),
			Self::Function(arg0) => f.write_fmt(format_args!("Function {arg0:?}")),
			Self::Type(_arg0) => f.write_fmt(format_args!("Type __TODO__")),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Function {
	params: Vec<(String, Type)>,
}

type Type = ();
