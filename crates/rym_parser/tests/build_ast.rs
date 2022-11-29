use indoc::indoc;
use rym_errors::{Diagnostic, Handler};
use rym_parser::{lexer::LitKind, parse, BinaryOp, Block, Expr, Stmt};
use smol_str::SmolStr;
use std::ops::Range;

mod decl;
mod expr;

#[track_caller]
fn assert_ast_errs(src: &str, expected: &[Stmt], errs: &[Diagnostic]) {
	let handler = Handler::default();
	let got_ast = parse(src, &handler);

	let got_errs = handler.collect();
	println!("{:?}", got_errs);
	assert_eq!(&got_errs, errs);

	assert_eq!(&got_ast, expected);
}

fn const_decl(name: &str, span: Range<usize>) -> Stmt {
	Stmt::VarDecl { name: (SmolStr::new(name), span.into()), mutable: false }
}

fn mut_decl(name: &str, span: Range<usize>) -> Stmt {
	Stmt::VarDecl { name: (SmolStr::new(name), span.into()), mutable: true }
}

fn fn_decl(
	name: (&str, Range<usize>),
	params: Vec<(&str, Range<usize>)>,
	return_type: Option<(&str, Range<usize>)>,
	body: Block,
) -> Stmt {
	Stmt::FnDecl {
		name: (SmolStr::new(name.0), name.1.into()),
		params: params.into_iter().map(|name| (SmolStr::new(name.0), name.1.into())).collect(),
		return_type: return_type.map(|typ| (SmolStr::new(typ.0), typ.1.into())),
		body,
	}
}

fn expr_binary(op: BinaryOp, lhs: Expr, rhs: Expr, span: Range<usize>) -> Expr {
	Expr::Binary { lhs: Box::new(lhs), op, rhs: Box::new(rhs), span: span.into() }
}

fn expr_ident(name: &str, span: Range<usize>) -> Expr {
	Expr::Ident { name: SmolStr::new(name), span: span.into() }
}

fn expr_lit(lit: impl Into<LitKind>, span: Range<usize>) -> Expr {
	Expr::Literal { lit: lit.into(), span: span.into() }
}

fn block(stmts: Vec<Stmt>, span: Range<usize>) -> Block {
	Block { stmts, span: span.into() }
}
