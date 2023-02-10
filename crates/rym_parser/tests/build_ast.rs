// use indoc::indoc;
use rym_errors::Level;
use rym_errors::{Diagnostic, DiagnosticHandler, RymResult};
use rym_parser::lexer::LitKind;
use rym_parser::{parse_file_from_src, BinaryOp, Block, Expr, FunctionParam, Item, Path, Stmt};
use rym_span::{DelimSpan, Span};
use smol_str::SmolStr;
use std::ops::{Deref, Range};

mod expr;
mod item;

#[track_caller]
fn assert_ast_errs(src: &str, expected: &[Item], errs: &[Diagnostic]) {
	let handler = DiagnosticHandler::default();
	let got_ast = parse_file_from_src(src, &handler);

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
	params: Vec<FunctionParam>,
	return_type: Option<Path>,
	body: Block,
) -> Item {
	Item::Function { name: ident(name.0, name.1), params, return_type, body }
}

fn fn_param(name: (&str, Range<usize>), typ: Option<Path>) -> FunctionParam {
	fn_rest_param(false, name, typ)
}

fn fn_rest_param(rest_param: bool, name: (&str, Range<usize>), typ: Option<Path>) -> FunctionParam {
	fn_rest_param_default(rest_param, name, typ, None)
}

fn fn_param_default(
	name: (&str, Range<usize>),
	typ: Option<Path>,
	default: Option<Expr>,
) -> FunctionParam {
	fn_rest_param_default(false, name, typ, default)
}

fn fn_rest_param_default(
	rest_param: bool,
	name: (&str, Range<usize>),
	typ: Option<Path>,
	default: Option<Expr>,
) -> FunctionParam {
	FunctionParam { rest_param, name: ident(name.0, name.1), typ, default }
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

fn expr_path(path: Path) -> Expr {
	Expr::Path { path }
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

// #[macro_export]
// macro_rules! path {
// 	($first_segment:ident $(:: $rest_segments:ident)*; $span:expr) => {
// 		Path { segments: vec![(SmolStr::new(stringify!($first_segment)), $span.into()), (SmolStr::new($(stringify!($rest_segments),)*), $span.into())] }
// 	};
// }

#[macro_export]
macro_rules! path {
	($path:path, $start:literal..$end:literal) => {
		build_path(stringify!($path), $start, $end)
	};
}

fn build_path(segments: impl Deref<Target = str>, start: usize, end: usize) -> Path {
	let mut pos = start;
	let span_len = end - start;
	let path_len = segments.len();
	if path_len > span_len {
		panic!("End value `{end}` does not match up with path length. Path length: {path_len}, Span length: {span_len}");
	}
	let segments: Vec<&str> = segments.split("::").collect();
	let mut first = true;
	let segments = segments
		.into_iter()
		.map(|segment| {
			ident(
				segment,
				{
					pos += if first { 0 } else { 2 };
					first = false;
					pos
				}..{
					pos += segment.len();
					pos
				},
			)
		})
		.collect();
	Path { segments }
}

#[test]
fn path() {
	let path = path!(crate::testing::Item, 0..20);
	assert_eq!(
		path,
		Path { segments: vec![ident("crate", 0..5), ident("testing", 7..14), ident("Item", 16..20)] }
	);
}

fn ident(name: &str, Range { start, end }: Range<usize>) -> (SmolStr, Span) {
	(SmolStr::new(name), Span::new(start, end))
}
