pub struct Module {
	name: String,
	constants: Vec<Constant>,
	children: Vec<Module>,
}

pub struct Constant {
	name: String,
	data: Expr,
	typ: Expr,
}

pub enum Expr {
	Value(Value),
	Unary(UnaryOp, Box<Expr>),
	Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

pub enum UnaryOp {
	/// Negation `-1`
	Neg,
	/// Not `!true`
	Not,
}

pub enum BinaryOp {
	/// Addition `1 + 2`
	Add,
	/// Subtraction `1 - 2`
	Sub,
}

pub enum Value {
	Bool(bool),
	Int(u64),
	Float(f64),
	String(String),
	Function(Function),
	Type(Type),
}

pub struct Function {
	params: Vec<(String, Type)>,
}

type Type = ();
