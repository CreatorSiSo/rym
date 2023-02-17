use crate::Spanned;

// #[derive(Debug, Clone)]
// pub enum Item {
// 	Func {
// 		name: Spanned<String>,
// 		params: Vec<Spanned<String>>,
// 		body: Option<Spanned<Expr>>,
// 	},
// 	Type {
// 		name: Spanned<String>,
// 	},
// 	Var {
// 		name: Spanned<String>,
// 		init: Option<Spanned<Expr>>,
// 	},
// }

#[derive(Debug, Clone)]
pub enum Stmt {
	Var {
		mutable: bool,
		name: Spanned<String>,
		init: Spanned<Expr>,
	},
	Expr(Spanned<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
	Not,
	Neg,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
	// Sum
	Add,
	Sub,

	// Product
	Mul,
	Div,
	// Rem,

	// Comparison
	Eq,
	Neq,
}

#[derive(Clone)]
pub enum Expr {
	Int(u64),
	Float(u64, u64),
	Char(char),
	String(String),

	Ident(String),

	// Unary expressions (!, -)
	Unary(UnaryOp, Box<Spanned<Expr>>),

	// Binary expressions (+, -, *, /, <, >)
	Binary(Box<Spanned<Expr>>, BinaryOp, Box<Spanned<Expr>>),

	Call(Box<Spanned<Expr>>, Vec<Spanned<Expr>>),
	Block(Vec<Stmt>),
}

impl std::fmt::Debug for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Int(val) => f.write_fmt(format_args!("Int({val})")),
			Self::Float(arg0, val1) => f.write_fmt(format_args!("Float({arg0}, {val1})")),
			Self::Char(val) => f.write_fmt(format_args!("Char('{}')", val.escape_default())),
			Self::String(val) => f.write_fmt(format_args!("String(\"{}\")", val.escape_default())),

			Self::Ident(val) => f.write_fmt(format_args!("Ident(\"{val}\")")),

			Self::Unary(op, rhs) => f.debug_tuple("Unary").field(op).field(rhs).finish(),
			Self::Binary(lhs, op, rhs) => f
				.debug_tuple("Binary")
				.field(lhs)
				.field(op)
				.field(rhs)
				.finish(),

			Self::Call(val0, val1) => f.debug_tuple("Call").field(val0).field(val1).finish(),
			Self::Block(val0) => f.debug_tuple("Block").field(val0).finish(),
		}
	}
}
