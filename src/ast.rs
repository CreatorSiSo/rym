use crate::Spanned;

#[derive(Debug)]
pub enum Item {
	Func {
		name: Spanned<String>,
		params: Vec<Spanned<String>>,
		body: Option<Expr>,
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
	Int(i64),
	Ident(String),

	// Unary expressions (!, -)
	Not(Box<Expr>),
	Neg(Box<Expr>),

	// Binary expressions (+, -, *, /)
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),

	Block(Vec<Stmt>),
}
