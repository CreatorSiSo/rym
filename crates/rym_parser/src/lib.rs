//! # Rym Grammar
//!
//! ```ignore
//! File ⮕ Item*
//! Item ⮕ Module | Use | Function | Enum | Struct | Trait | Implementation
//!
//! Module ⮕ "module" Ident "{" Item* "}"
//!
//! Use ⮕ "use" UseTree ItemEnd
//! UseTree ⮕ Path | Path "::" "*" | Path "::" "{" (UseTree ("," UseTree)*)? "}"
//!
//! Function ⮕ "fn" Ident "(" FunctionParams? ")" ("->" Type)? BlockExpr
//! FunctionParams ⮕ Ident ("," Ident)* ","?
//!
//! Enum ⮕ "enum" Ident "{" EnumItem ("," EnumItem)* ","? "}"
//! EnumItem ⮕ Ident ("(" TupleFields? ")" | "{" StructFields? "}")?
//!
//! Struct ⮕ "struct" Ident ("(" TupleFields? ")" | "{" StructFields? "}" | ItemEnd)
//! TupleFields ⮕ TupleField ("," TupleField)* ","?
//! TupleField ⮕ (Ident ":")? Type ("=" Expr)?
//! StructFields ⮕ StructField ("," StructField)* ","?
//! StructField ⮕ Ident ":" Type ("=" Expr)?
//!
//! Trait ⮕ "trait" Ident "{" __TODO__ "}"
//!
//! Implementation ⮕ "impl" Path ("for" Path) "{" __TODO__ "}"
//!
//! Stmt ⮕ Item | VarStmt | ExprStmt
//!
//! VarStmt ⮕ ("const" | "mut") Ident "=" Expr ItemEnd
//!
//! ExprStmt ⮕ Expr ItemEnd
//! Expr ⮕ ExprWithoutBlock | ExprWithBlock
//!
//! ExprWithoutBlock ⮕ LiteralExpr | PathExpr | OperatorExpr | GroupedExpr | ArrayExpr
//!                    | IndexExpr | Tuple | MethodCallExpr | CallExpr | RangeExpr
//!                    | ClosureExpr | ContinueExpr | BreakExpr | ReturnExpr
//! LiteralExpr ⮕ FloatLit | IntLit | StringLit | CharLit
//! PathExpr ⮕ Path
//! OperatorExpr ⮕ UnaryExpr | BinaryExpr
//! UnaryExpr ⮕ ("!" | "-") Expr
//! BinaryExpr ⮕ Expr ("+" | "-" | "*" | "/" | "%") Expr
//! GroupedExpr ⮕ "(" Expr ")"
//! ArrayExpr ⮕ "[" (Expr ("," Expr)* ","? | Expr ";" Expr) "]"
//! IndexExpr ⮕ Expr "[" Expr "]"
//! Tuple ⮕ "(" ((Expr ",")+ Expr)? ")"
//! MethodCallExpr ⮕ Expr "." Ident "(" CallArgs? ")"
//! CallExpr ⮕ Expr "(" CallArgs? ")"
//! CallArgs ⮕ Expr ("," Expr)* ","
//! RangeExpr ⮕ Expr? ".." "="? Expr?
//! ClosureExpr ⮕ "|" ClosureParams "|" (Expr | "->" Type BlockExpr)
//! ClosureParams ⮕ __TODO__
//! ContinueExpr ⮕ "continue"
//! BreakExpr ⮕ "break" Expr?
//! ReturnExpr ⮕ "return" Expr
//!
//! ExprWithBlock ⮕ BlockExpr | LoopExpr | IfExpr | IfVarExpr | MatchExpr
//! BlockExpr ⮕ "{" Stmt* "}"
//! LoopExpr ⮕ "loop" BlockExpr
//! IfExpr ⮕ "if" Expr BlockExpr ("else" (BlockExpr | IfExpr | IfVarExpr))?
//! IfVarExpr ⮕ "if" ("const" | "mut") __TODO__ "=" Expr BlockExpr ("else" (BlockExpr | IfExpr | IfVarExpr))?
//! MatchExpr ⮕ __TODO__
//!
//! Type ⮕ Path
//! Path ⮕ Ident ("::" Ident)*
//! Ident ⮕ ('_' | UnicodeIdentStart) UnicodeIdentContinue
//! ItemEnd ⮕ ";" | "\n" | EOF
//! ```

use std::fmt::Debug;

use rym_errors::{Diagnostic, Handler, Level, RymResult};
use rym_span::{DelimSpan, Span};
use smol_str::SmolStr;

pub mod lexer;
use lexer::{Delimiter, LinearLexer, LitKind, Token, TokenKind, TokenStream};

pub fn parse(src: &str, handler: &Handler) -> Vec<Item> {
	let mut token_stream: TokenStream = LinearLexer::new(src, handler).collect();
	let mut parser = Parser { handler };
	parser.parse_file(&mut token_stream)
}

pub fn parse_single_expr(src: &str, handler: &Handler) -> RymResult<Expr> {
	let mut token_stream: TokenStream = LinearLexer::new(src, handler).collect();
	let parser = Parser { handler };
	parser.parse_expr(&mut token_stream)
}

pub struct Parser<'a> {
	handler: &'a Handler,
}

impl<'a> Parser<'a> {
	pub const fn new(handler: &'a Handler) -> Self {
		Self { handler }
	}

	/// ```ignore
	/// File ⮕ Item*
	///```
	pub fn parse_file(&mut self, token_stream: &mut TokenStream) -> Vec<Item> {
		let mut items = vec![];
		while !token_stream.is_empty() {
			match self.parse_item(token_stream) {
				Ok(item) => items.push(item),
				Err(err) => {
					self.handler.emit(err);
					token_stream.consume_while(&[
						TokenKind::Module,
						TokenKind::Use,
						TokenKind::Fn,
						TokenKind::Enum,
						TokenKind::Struct,
						TokenKind::Trait,
						TokenKind::Impl,
					]);
				}
			}
		}
		items
	}

	/// ```ignore
	/// Item ⮕ Module | Use | Function | Enum | Struct | Trait | Implementation
	///```
	pub fn parse_item(&mut self, token_stream: &mut TokenStream) -> RymResult<Item> {
		let Token { kind: item_kind, .. } = token_stream.expect(&[
			TokenKind::Module,
			TokenKind::Use,
			TokenKind::Fn,
			TokenKind::Enum,
			TokenKind::Struct,
			TokenKind::Trait,
			TokenKind::Impl,
		])?;
		match item_kind {
			TokenKind::Module => self.parse_module(token_stream),
			TokenKind::Use => todo!(),
			TokenKind::Fn => self.parse_function(token_stream),
			TokenKind::Enum => todo!(),
			TokenKind::Struct => todo!(),
			TokenKind::Trait => todo!(),
			TokenKind::Impl => todo!(),
			_ => unreachable!(),
		}
	}

	/// Assumes that the `module` keyword has already been consumed
	/// ```ignore
	/// Module ⮕ "module" Ident "{" Item* "}"
	/// ```
	fn parse_module(&mut self, token_stream: &mut TokenStream) -> RymResult<Item> {
		let name = token_stream.expect_ident()?;

		let (items, delim_span) =
			self.parse_delimited(token_stream, Delimiter::Brace, |parser, token_stream| {
				parser.parse_item(token_stream)
			})?;

		Ok(Item::Module { name, items, delim_span })
	}

	/// Assumes that the `fn` keyword has already been consumed
	/// ```ignore
	/// Function ⮕ "fn" Ident "(" FunctionParams? ")" ("->" Type)? BlockExpr
	/// FunctionParams ⮕ FunctionParam ("," FunctionParam)* ","?
	/// FunctionParam ⮕ ".."? Ident (":" Path)? ("=" Expr)?
	/// ```
	fn parse_function(&mut self, token_stream: &mut TokenStream) -> RymResult<Item> {
		let name = token_stream.expect_ident()?;

		let (params, _) = self.parse_delimited(token_stream, Delimiter::Paren, |_, token_stream| {
			let rest_param = token_stream.matches(TokenKind::DotDot).is_some();
			let name = token_stream.expect_ident()?;

			let typ = if token_stream.matches(TokenKind::Colon).is_some() {
				// TODO Use path insted of ident
				Some(token_stream.expect_ident()?)
			} else {
				None
			};

			// TODO Parse default values
			token_stream.matches(TokenKind::Comma);
			Ok(FunctionParam { rest_param, name, typ, default: None })
		})?;

		let return_type = if token_stream.matches(TokenKind::ThinArrow).is_some() {
			// TODO Use path insted of ident
			Some(token_stream.expect_ident()?)
		} else {
			None
		};

		let body = self.parse_block(token_stream)?;

		Ok(Item::Function { name, params, return_type, body })
	}

	// /// ```ignore
	// /// Stmt ⮕ (Decl | Expr) ItemEnd
	// /// ```
	// fn parse_stmt(&self, token_stream: &mut TokenStream) -> RymResult<Option<Stmt>> {
	// 	let Some(token) = token_stream.peek(false) else {
	// 		return Ok(None);
	// 	};
	// 	let stmt = match token.kind {
	// 		TokenKind::Const => self.parse_var_decl(token_stream, false),
	// 		TokenKind::Mut => self.parse_var_decl(token_stream, true),
	// 		_ => todo!(), /* self.parse_expr(token_stream).map(Stmt::Expr) */
	// 	}?;
	// 	token_stream.expect(&[Tk::Eof, Tk::Newline, Tk::Semi, Tk::CloseDelim(Delimiter::Brace)])?;
	// 	Ok(Some(stmt))
	// }

	// /// ```ignore
	// /// VarDecl ⮕ ("const" | "mut") Ident "=" Expr
	// /// ```
	// fn parse_var_decl(&self, token_stream: &mut TokenStream, mutable: bool) -> RymResult<Stmt> {
	// 	token_stream.expect(&[Tk::Const, Tk::Mut])?;
	// 	let name = token_stream.expect_ident()?;
	// 	token_stream.expect(Tk::Eq)?;
	// 	Ok(Stmt::VarDecl { name, mutable })
	// }

	/// Expr => UnaryExpr
	fn parse_expr(&self, _token_stream: &mut TokenStream) -> RymResult<Expr> {
		todo!()
	}

	fn parse_block(&mut self, token_stream: &mut TokenStream) -> RymResult<Block> {
		let (stmts, span) = self.parse_delimited(token_stream, Delimiter::Brace, |_, _| todo!())?;
		Ok(Block { stmts, span })
	}

	/// Expectes the next token to be the opening variant of the delimiter
	/// Executes the provided function until the closing variant of the delimiter is seen
	/// Produces diagnostics is it is unclosed
	fn parse_delimited<T: Debug>(
		&mut self,
		token_stream: &mut TokenStream,
		delim: Delimiter,
		f: impl Fn(&mut Self, &mut TokenStream) -> RymResult<T>,
	) -> RymResult<(Vec<T>, DelimSpan)> {
		println!("Inside delimted {delim:?}");
		let Token { span: open, .. } = token_stream.expect(TokenKind::OpenDelim(delim))?;
		let mut items = vec![];

		let close = loop {
			println!("Inside delimted {delim:?}");
			if token_stream.is_empty() {
				let last = token_stream.previous_span();
				self.handler.emit(Diagnostic::new_spanned(
					Level::Error,
					"Unclosed module block",
					Span::new(open.start, last.end),
				));
				break last;
			}

			match token_stream.matches(TokenKind::CloseDelim(delim)) {
				Some(Token { span, .. }) => break span,
				None => self.handler.handle(f(self, token_stream)).map(|item| items.push(item)),
			};
		};

		Ok((items, DelimSpan { open, close, entire: Span::new(open.start, close.end) }))
	}
}

// fn binding_power(tt: Token, is_prefix: bool) -> (Token, (u8, u8)) {
// 	(todo!(), (0, 0))
// }

#[derive(Debug, PartialEq)]
pub enum Item {
	Module {
		name: (SmolStr, Span),
		items: Vec<Item>,
		delim_span: DelimSpan,
	},
	Use,
	Function {
		name: (SmolStr, Span),
		params: Vec<FunctionParam>,
		return_type: Option<(SmolStr, Span)>,
		body: Block,
	},
	Enum,
	Struct,
	Trait,
	Impl,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam {
	pub rest_param: bool,
	pub name: (SmolStr, Span),
	pub typ: Option<(SmolStr, Span)>, // TODO: Make this a path
	pub default: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
	Empty,
	Expr(Expr),
	VarDecl { name: (SmolStr, Span), mutable: bool },
}

#[derive(Debug, PartialEq)]
pub struct Block {
	pub stmts: Vec<Stmt>,
	pub span: DelimSpan,
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
