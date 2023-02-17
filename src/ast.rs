use crate::Spanned;

#[derive(Debug)]
pub enum Item {
	Func {
		name: Spanned<String>,
		params: Vec<Spanned<String>>,
		body: Option<Spanned<Expr>>,
	},
	Type {
		name: Spanned<String>,
	},
	Var {
		name: Spanned<String>,
		init: Option<Expr>,
	},
}

#[derive(Debug)]
pub enum Stmt {
	Item(Item),
	Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
	Int(u64),
	Float(u64, u64),
	Char(char),
	String(String),

	Ident(String),

	// Unary expressions (!, -)
	Not(Box<Expr>),
	Neg(Box<Expr>),

	// Binary expressions (+, -, *, /)
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),

	Call(Box<Spanned<Expr>>, Vec<Spanned<Expr>>),
	Block(Vec<Stmt>),
}
