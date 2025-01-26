//! Low level intermediate representation
//!
//! - local and global variables have been converted to indices
//! - types only descibe data layout

use super::ty::Type;
pub use crate::ast::{BinaryOp, UnaryOp};

pub struct TypedExpr<'a>(pub &'a Expr<'a>, pub Type<'a>);

pub enum Expr<'a> {
    // Value creation
    Literal(u64),
    Array(&'a [TypedExpr<'a>]),
    Aggregate(&'a [TypedExpr<'a>]),

    // Value modification
    Unary(UnaryOp, TypedExpr<'a>),
    Binary(BinaryOp, TypedExpr<'a>, TypedExpr<'a>),

    // Value access
    AccessLocal(usize),
    AccessField(TypedExpr<'a>, usize),
    Assign(TypedExpr<'a>, TypedExpr<'a>),
    Subscript(TypedExpr<'a>, TypedExpr<'a>),

    // Control flow
    Call(TypedExpr<'a>, &'a [TypedExpr<'a>]),
    IfElse(
        /// Condition
        TypedExpr<'a>,
        /// Then branch
        TypedExpr<'a>,
        /// Else branch
        TypedExpr<'a>,
    ),
    Loop(&'a [TypedExpr<'a>]),
    Block(&'a [TypedExpr<'a>]),
    Break(TypedExpr<'a>),
    Return(TypedExpr<'a>),
}
