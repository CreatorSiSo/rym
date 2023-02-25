use crate::Spanned;

#[derive(Debug, Clone)]
pub enum Item {
	Module {
		name: Spanned<String>,
		items: Vec<Spanned<Item>>,
	},
	Func {
		name: Spanned<String>,
		params: Vec<Spanned<String>>,
		rhs: Spanned<Expr>,
	},
	Var {
		mutable: bool,
		name: Spanned<String>,
		rhs: Spanned<Expr>,
	},
}

#[derive(Debug, Clone)]
pub enum Stmt {
	Item(Spanned<Item>),
	Expr(Spanned<Expr>),
	Error,
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
	Greater,
	Less,
}

#[derive(Clone)]
pub enum Expr {
	Ident(String),

	Int(u64),
	Float(u64, u64),
	Char(char),
	String(String),
	Record {
		name: Option<Spanned<String>>,
		fields: Vec<(Spanned<String>, Spanned<Expr>)>,
	},

	/// Unary expressions (`!`, `-`)
	Unary(UnaryOp, Box<Spanned<Expr>>),
	/// Binary expressions (`+`, `-`, `*`, `/`, `<`, `>`, ..)
	Binary(Box<Spanned<Expr>>, BinaryOp, Box<Spanned<Expr>>),
	Assign {
		name: Spanned<String>,
		rhs: Box<Spanned<Expr>>,
	},
	/// Function call `path(args)`, `returns_fn()()`
	Call(Box<Spanned<Expr>>, Vec<Spanned<Expr>>),

	Block(Vec<Stmt>),
	If {
		condition: Box<Spanned<Expr>>,
		then_branch: Box<Spanned<Expr>>,
		else_branch: Box<Option<Spanned<Expr>>>,
	},
	Loop(Box<Spanned<Expr>>),
	Break(Box<Option<Spanned<Expr>>>),
	NoNewline(Box<Spanned<Expr>>),
	Continue,
	Return(Box<Option<Spanned<Expr>>>),

	Error,
}

impl std::fmt::Debug for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Ident(val) => f.write_fmt(format_args!("Ident(\"{val}\")")),

			Self::Int(val) => f.write_fmt(format_args!("Int({val})")),
			Self::Float(arg0, val1) => f.write_fmt(format_args!("Float({arg0}, {val1})")),
			Self::Char(val) => f.write_fmt(format_args!("Char('{}')", val.escape_default())),
			Self::String(val) => f.write_fmt(format_args!("String(\"{}\")", val.escape_default())),
			Self::Record { name, fields } => f
				.debug_struct("Record")
				.field("name", name)
				.field("fields", fields)
				.finish(),

			Self::Unary(op, rhs) => f.debug_tuple("Unary").field(op).field(rhs).finish(),
			Self::Binary(lhs, op, rhs) => f
				.debug_tuple("Binary")
				.field(lhs)
				.field(op)
				.field(rhs)
				.finish(),
			Self::Assign { name, rhs } => f
				.debug_struct("Assign")
				.field("name", name)
				.field("rhs", rhs)
				.finish(),
			Self::Call(val0, val1) => f.debug_tuple("Call").field(val0).field(val1).finish(),

			Self::Block(val0) => f.debug_tuple("Block").field(val0).finish(),
			Self::If {
				condition,
				then_branch,
				else_branch,
			} => f
				.debug_struct("If")
				.field("condition", condition)
				.field("then_branch", then_branch)
				.field("else_branch", else_branch)
				.finish(),
			Self::Loop(val0) => f.debug_tuple("Loop").field(val0).finish(),
			Self::Break(val0) => f.debug_tuple("Break").field(val0).finish(),
			Self::NoNewline(val0) => f.debug_tuple("NoNewline").field(val0).finish(),
			Self::Continue => f.write_str("Continue"),
			Self::Return(val0) => f.debug_tuple("Return").field(val0).finish(),

			Self::Error => write!(f, "Error"),
		}
	}
}
