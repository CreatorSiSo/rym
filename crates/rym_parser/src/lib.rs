//! # Rym Grammar
//!
//! ```ignore
//! Module ⮕ Stmt*
//! Stmt ⮕ Expr | Decl
//!
//! Decl ⮕ VarDecl | TypeDecl | FnDecl | StructDecl | EnumDecl
//!
//! VarDecl ⮕ ("const" | "mut") Ident "=" Expr StmtEnd
//! FnDecl ⮕ "fn" Ident "(" FnParams? ")" ("->" Type)? Block
//! FnParams ⮕ Ident ("," Ident)* ","?
//!
//! TypeDecl ⮕ "type" Ident "=" __TODO__ StmtEnd
//! StructDecl ⮕ "struct" Ident (StmtEnd | "{" StructFields? "}" | "(" TupleFields? ")")
//! StructFields ⮕ StructField ("," StructField)* ","?
//! TupleFields ⮕ TupleField ("," TupleField)* ","?
//! StructField ⮕ Ident ":" Type ("=" Expr)?
//! TupleField ⮕ (Ident ":")? Type
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
use lexer::{Delimiter, LinearLexer, LitKind, Tk, TokenKind, TokenStream};

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
	/// ```
	fn parse_module(&mut self, token_stream: &mut TokenStream) -> Vec<Stmt> {
		let mut stmts = vec![];
		loop {
			match self.parse_stmt(token_stream) {
				Ok(maybe_stmt) => match maybe_stmt {
					Some(stmt) => stmts.push(stmt),
					None => return stmts,
				},
				Err(err) => self.handler.emit(err),
			}
		}
	}

	/// ```ignore
	/// Stmt ⮕ (Decl | Expr) StmtEnd
	/// ```
	fn parse_stmt(&self, token_stream: &mut TokenStream) -> RymResult<Option<Stmt>> {
		let Some(token) = token_stream.peek(false) else {
			return Ok(None);
		};
		let stmt = match token.kind {
			TokenKind::Const => self.parse_var_decl(token_stream, false),
			TokenKind::Mut => self.parse_var_decl(token_stream, true),
			TokenKind::Fn => self.parse_fn_decl(token_stream),
			_ => return Ok(None), /* self.parse_expr(token_stream).map(Stmt::Expr) */
		}?;
		token_stream.expect(&[Tk::Eof, Tk::Newline, Tk::Semi, Tk::CloseDelim(Delimiter::Brace)])?;
		Ok(Some(stmt))
	}

	/// ```ignore
	/// VarDecl ⮕ ("const" | "mut") Ident "=" Expr
	/// ```
	fn parse_var_decl(&self, token_stream: &mut TokenStream, mutable: bool) -> RymResult<Stmt> {
		token_stream.expect(&[Tk::Const, Tk::Mut])?;
		let name = token_stream.expect_ident()?;
		token_stream.expect(Tk::Eq)?;
		Ok(Stmt::VarDecl { name, mutable })
	}

	/// ```ignore
	/// FnDecl ⮕ "fn" Ident FnParams ("->" Type)? Block
	/// ```
	fn parse_fn_decl(&self, token_stream: &mut TokenStream) -> RymResult<Stmt> {
		token_stream.expect(Tk::Fn)?;
		let name = token_stream.expect_ident()?;
		let params = self.parse_fn_params(token_stream)?;
		let return_type = if token_stream.matches(Tk::Minus).is_some()
			&& token_stream.expect(Tk::GreaterThan).is_ok()
		{
			Some(token_stream.expect_ident()?)
		} else {
			None
		};
		let body = self.parse_block(token_stream)?;
		Ok(Stmt::FnDecl { name, params, return_type, body })
	}

	/// ```ignore
	/// FnParams ⮕ "(" Inner? ")"
	/// Inner ⮕ Ident ("," Ident)* ","?
	/// ```
	fn parse_fn_params(&self, token_stream: &mut TokenStream) -> RymResult<Vec<(SmolStr, Span)>> {
		token_stream.expect(Tk::OpenDelim(Delimiter::Paren))?;
		if token_stream.matches(Tk::CloseDelim(Delimiter::Paren)).is_some() {
			return Ok(vec![]);
		}

		let mut params = vec![token_stream.expect_ident()?];
		while let Some(..) = token_stream.matches(Tk::Comma) {
			params.push(token_stream.expect_ident()?);
		}
		token_stream.matches(Tk::CloseDelim(Delimiter::Paren));
		Ok(params)
	}

	/// Expr => UnaryExpr
	fn parse_expr(&self, token_stream: &mut TokenStream) -> RymResult<Expr> {
		let Some(_) = token_stream.matches(&[Tk::Literal, Tk::Ident]) else {
			todo!()
		};
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

		// todo!()
		Ok(Expr::Empty)
	}

	fn parse_block(&self, token_stream: &mut TokenStream) -> RymResult<Block> {
		let start = token_stream.expect(Tk::OpenDelim(Delimiter::Brace))?.span.start;
		let stmts = vec![];
		let end = token_stream.expect(Tk::CloseDelim(Delimiter::Brace))?.span.end;
		Ok(Block { stmts, span: Span::new(start, end) })
	}
}

// fn binding_power(tt: Token, is_prefix: bool) -> (Token, (u8, u8)) {
// 	(todo!(), (0, 0))
// }

#[derive(Debug, PartialEq)]
pub enum Stmt {
	Empty,
	Expr(Expr),
	VarDecl {
		name: (SmolStr, Span),
		mutable: bool,
	},
	FnDecl {
		name: (SmolStr, Span),
		params: Vec<(SmolStr, Span)>,
		return_type: Option<(SmolStr, Span)>,
		body: Block,
	},
}

#[derive(Debug, PartialEq)]
pub struct Block {
	pub stmts: Vec<Stmt>,
	pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
	Binary { lhs: Box<Expr>, op: BinaryOp, rhs: Box<Expr>, span: Span },
	Unary { op: UnaryOp, rhs: Box<Expr>, span: Span },
	Ident { name: SmolStr, span: Span },
	Literal { lit: LitKind, span: Span },
	Empty,
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
