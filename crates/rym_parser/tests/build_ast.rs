use indoc::indoc;
use rym_errors::RymResult;
use rym_errors::{Diagnostic, Handler};
use rym_parser::{lexer::LitKind, parse, BinaryOp, Block, Expr, Item, Stmt};
use rym_span::DelimSpan;
use smol_str::SmolStr;
use std::ops::Range;

mod expr;
mod item;

#[track_caller]
fn assert_ast_errs(src: &str, expected: &[Item], errs: &[Diagnostic]) {
	let handler = Handler::default();
	let got_ast = parse(src, &handler);

	let got_errs = handler.collect();
	println!("{:?}", got_errs);
	assert_eq!(&got_errs, errs);

	assert_eq!(&got_ast, expected);
}

fn item_module(name: (&str, Range<usize>), items: Vec<Item>, span: DelimSpan) -> Item {
	Item::Module { name: (SmolStr::new(name.0), name.1.into()), items, delim_span: span }
}

fn fn_item(
	name: (&str, Range<usize>),
	params: Vec<(&str, Range<usize>)>,
	return_type: Option<(&str, Range<usize>)>,
	body: Block,
) -> Item {
	Item::Function {
		name: (SmolStr::new(name.0), name.1.into()),
		params: params.into_iter().map(|name| (SmolStr::new(name.0), name.1.into())).collect(),
		return_type: return_type.map(|typ| (SmolStr::new(typ.0), typ.1.into())),
		body,
	}
}

fn const_decl(name: &str, span: Range<usize>) -> Stmt {
	Stmt::VarDecl { name: (SmolStr::new(name), span.into()), mutable: false }
}

fn mut_decl(name: &str, span: Range<usize>) -> Stmt {
	Stmt::VarDecl { name: (SmolStr::new(name), span.into()), mutable: true }
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

fn block(stmts: Vec<Stmt>, span: DelimSpan) -> Block {
	Block { stmts, span }
}

fn delim_span(open: Range<usize>, close: Range<usize>, entire: Range<usize>) -> DelimSpan {
	DelimSpan { open: open.into(), close: close.into(), entire: entire.into() }
}
