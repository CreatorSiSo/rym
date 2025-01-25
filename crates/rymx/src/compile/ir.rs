use super::ty::Type;
pub use crate::ast::{BinaryOp, UnaryOp};

pub struct Function<'a> {
    pub params: &'a [Type<'a>],
    pub result: Type<'a>,
    pub body: &'a Expr<'a>,
}

pub enum Expr<'a> {
    // Value creation
    Uint(u64),
    Int(u64),
    // Array(&'a [Expr<'a>]),
    Aggregate(&'a [Expr<'a>]),
    Function(Function<'a>),

    // Value modification
    Unary(UnaryOp, &'a Expr<'a>),
    Binary(BinaryOp, &'a Expr<'a>, &'a Expr<'a>),
    Call(&'a Expr<'a>, &'a [Expr<'a>]),

    // Value access
    Store(usize, &'a Expr<'a>),
    Load(usize),
    StoreField(usize, usize, &'a Expr<'a>),
    LoadField(usize, usize),
    // Subscript(&'a Expr<'a>, &'a Expr<'a>),

    // Control flow
    IfElse(
        /// Condition
        &'a Expr<'a>,
        /// Then branch
        &'a Expr<'a>,
        /// Else branch
        &'a Expr<'a>,
    ),
    Loop(&'a [Stmt<'a>]),
    Block(&'a [Stmt<'a>]),
    Break(&'a Expr<'a>),
    Return(&'a Expr<'a>),
}

pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Def(Type<'a>, Expr<'a>),
}
