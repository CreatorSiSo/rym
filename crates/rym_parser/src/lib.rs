//! # Rym Grammar
//!
//! ```ignore
//! Module ⮕ Stmt*
//! Stmt ⮕ Expr | Decl
//!
//! Decl ⮕ VarDecl | TypeDecl | FnDecl | StructDecl | EnumDecl
//!
//! VarDecl ⮕ ("const" | "mut") Ident "=" Expr StmtEnd
//!
//! TypeDecl ⮕ Ident "=" __TODO__ StmtEnd
//!
//! FnDecl ⮕ "fn" Ident "(" ArgsList ")" Block
//!
//! StructDecl ⮕ "struct" Ident (StmtEnd | "{" StructFields? "}" | "(" TupleFields? ")")
//! StructFields ⮕ StructField ("," StructField)* ","?
//! TupleFields ⮕ TupleField ("," TupleField)* ","?
//! StructField ⮕ Ident ":" Type ("=" Expr)?
//! TupleField ⮕ (Ident ":")? Type
//!
//! EnumDecl ⮕ "enum" Ident __TODO__
//!
//! Expr ⮕ ExprWithBlock | ExprWithoutBlock
//! ExprWithoutBlock ⮕ LiteralExpr | PathExpr | OperatorExpr
//! LiteralExpr ⮕ FloatLit | IntLit | StringLit | CharLit
//! PathExpr ⮕ Path
//! OperatorExpr ⮕ UnaryExpr | BinaryExpr
//! UnaryExpr ⮕ ("!" | "-") Expr
//! BinaryExpr ⮕ Expr ("+" | "-" | "*" | "/" | "%") Expr
//! ExprWithBlock ⮕ IfExpr |
//!
//! Type ⮕ Path
//! Path ⮕ Ident ("::" Ident)*
//! StmtEnd ⮕ ";" | "\n" | EOF
//! Ident ⮕ ('_' | UnicodeIdentStart) UnicodeIdentContinue
//! ```

use rym_errors::{Handler, RymResult};
use rym_span::Span;
use smol_str::SmolStr;

pub mod lexer;
use lexer::{LinearLexer, LitKind, Tk, Token, TokenKind, TokenStream};

pub fn parse(src: &str, handler: &Handler) -> Vec<Stmt> {
	let mut token_stream: TokenStream = LinearLexer::new(src, handler).collect();
	let mut parser = Parser { handler };
	parser.parse_module(&mut token_stream)
}

pub struct Parser<'a> {
	handler: &'a Handler,
}

impl<'a> Parser<'a> {
	pub const fn new(handler: &'a Handler) -> Self {
		Self { handler }
	}

	/// ```ignore
	/// Module ⮕ Stmt*
	/// Stmt ⮕ (Decl | Expr) StmtEnd
	/// ```
	fn parse_module(&mut self, token_stream: &mut TokenStream) -> Vec<Stmt> {
		let mut stmts = vec![];
		loop {
			let Some(token) = token_stream.peek(false) else {
				break stmts;
			};
			let result = match token.kind {
				TokenKind::Const => self.parse_var_decl(token_stream, false),
				TokenKind::Mut => self.parse_var_decl(token_stream, true),
				_ => todo!(), /* self.parse_expr(token_stream).map(Stmt::Expr) */
			};
			match result {
				Ok(stmt) => {
					stmts.push(stmt);
					if let Err(err) = token_stream.expect(&[Tk::Semi, Tk::Newline]) {
						self.handler.emit(err)
					}
				}
				Err(err) => self.handler.emit(err),
			}
		}
	}

	/// ```ignore
	/// VarDecl ⮕ ("const" | "mut") Ident "=" Expr
	/// ```
	fn parse_var_decl(&mut self, token_stream: &mut TokenStream, mutable: bool) -> RymResult<Stmt> {
		token_stream.expect(&[Tk::Const, Tk::Mut])?;
		let Token { kind: TokenKind::Ident(name), span } = token_stream.expect(Tk::Ident)? else {
			unreachable!();
		};
		token_stream.expect(Tk::Eq)?;
		Ok(Stmt::VarDecl { name, mutable, ident_span: span })
	}

	/// Expr => UnaryExpr
	fn parse_expr(&mut self, outer_ts: &mut TokenStream) -> RymResult<Expr> {
		// let Some(l_tt) = outer_ts.next() else {
		// 	return Err(Diagnostic::new(Level::Error, "Unexpected end of file"));
		// };
		// let lhs = match l_tt {
		// 	Token { kind, span } => match kind {
		// 		TokenKind::Ident(name) => Expr::Ident { name, span },
		// 		TokenKind::Literal(lit) => self.make_lit_expr(lit),
		// 		other => panic!("{other:?}"),
		// 	},
		// 	// TokenTree::Delimited(_, delim, delim_ts) => match delim {
		// 	// 	Delimiter::Paren => self.parse_expr(&mut delim_ts.into_iter())?,
		// 	// 	Delimiter::Brace => self.parse_block_expr(&mut delim_ts.into_iter())?,
		// 	// 	Delimiter::Bracket => self.parse_array_expr(&mut delim_ts.into_iter())?,
		// 	// },
		// };

		// loop {
		// 	let Some(r_tt) = outer_ts.next() else {
		// 		return Err(Diagnostic::new(Level::Error, "Unexpected end of file"));
		// 	};
		// 	// let (r_tt, r_bp) = match binding_power(tt, lhs.is_none()) {};
		// }

		todo!()
	}

	fn parse_block_expr(&self, token_stream: &mut TokenStream) -> RymResult<Expr> {
		todo!()
	}

	fn parse_array_expr(&self, token_stream: &mut TokenStream) -> RymResult<Expr> {
		todo!()
	}

	fn make_lit_expr(&self, lit: LitKind) -> Expr {
		todo!()
	}
}

// fn binding_power(tt: Token, is_prefix: bool) -> (Token, (u8, u8)) {
// 	(todo!(), (0, 0))
// }

#[derive(Debug, PartialEq)]
pub enum Stmt {
	VarDecl { name: SmolStr, mutable: bool, ident_span: Span },
	Expr(Expr),
	Empty,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
	Binary { lhs: Box<Expr>, op: BinaryOp, rhs: Box<Expr>, span: Span },
	Unary { op: UnaryOp, rhs: Box<Expr>, span: Span },
	Ident { name: SmolStr, span: Span },
	Literal { lit: LitKind, span: Span },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
	/// `+`
	Add,
	/// `-`
	Sub,
	/// `*`
	Mul,
	/// `/`
	Div,
	/// `%`
	Mod,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
	/// `!`
	Not,
	/// `-`
	Neg,
}

#[cfg(test)]
mod parse {
	use super::*;
	use indoc::indoc;
	use rym_errors::Diagnostic;
	use std::ops::Range;

	#[track_caller]
	fn assert_ast_errs(src: &str, expected: &[Stmt], errs: &[Diagnostic]) {
		let handler = Handler::default();
		let got_ast = parse(src, &handler);

		let got_errs = handler.collect();
		println!("{:?}", got_errs);
		assert_eq!(&got_errs, errs);

		assert_eq!(&got_ast, expected);
	}

	fn const_decl(name: &str, Range { start, end }: Range<usize>) -> Stmt {
		Stmt::VarDecl { name: SmolStr::new(name), mutable: false, ident_span: Span { start, end } }
	}

	fn mut_decl(name: &str, Range { start, end }: Range<usize>) -> Stmt {
		Stmt::VarDecl { name: SmolStr::new(name), mutable: true, ident_span: Span { start, end } }
	}

	fn expr_binary(lhs: Expr, op: BinaryOp, rhs: Expr, span: Range<usize>) -> Expr {
		Expr::Binary {
			lhs: Box::new(lhs),
			op,
			rhs: Box::new(rhs),
			span: Span::new(span.start, span.end),
		}
	}

	fn expr_ident(name: &str, span: Range<usize>) -> Expr {
		Expr::Ident { name: SmolStr::new(name), span: Span::new(span.start, span.end) }
	}

	fn expr_lit(lit: impl Into<LitKind>, span: Range<usize>) -> Expr {
		Expr::Literal { lit: lit.into(), span: Span::new(span.start, span.end) }
	}

	// #[test]
	fn simple_addition() {
		assert_ast_errs(
			"src + 2",
			&[Stmt::Expr(expr_binary(expr_ident("src", 0..3), BinaryOp::Add, expr_lit(2, 6..7), 0..7))],
			&[],
		);
	}

	#[test]
	fn var_decl() {
		assert_ast_errs(
			"const	test1 = \n mut test2 = ;\n",
			&[const_decl("test1", 6..11), mut_decl("test2", 20..25)],
			&[],
		);
		assert_ast_errs(
			indoc!(
				r#"const
				test1	=
				mut	test2=;
				"#
			),
			&[const_decl("test1", 6..11), mut_decl("test2", 18..23)],
			&[],
		);
	}
}
