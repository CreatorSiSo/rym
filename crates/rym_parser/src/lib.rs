//! # Rym Grammar
//!
//! ```ignore
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

use rym_errors::{Diagnostic, DiagnosticHandler, HandleDiagnostic, Level, RymResult};
use rym_span::{DelimSpan, Span, DUMMY_SPAN};
use smol_str::SmolStr;

pub mod lexer;
use lexer::{Delimiter, LinearLexer, LitKind, Token, TokenKind, TokenStream};

pub fn parse_file_from_src(src: &str, handler: &DiagnosticHandler) -> Vec<Item> {
	let mut token_stream: TokenStream = LinearLexer::new(src, handler).collect();
	parse_file(handler, &mut token_stream)
}

pub fn parse_expr_from_src(src: &str, handler: &DiagnosticHandler) -> RymResult<Expr> {
	let mut token_stream: TokenStream = LinearLexer::new(src, handler).collect();
	parse_expr(handler, &mut token_stream)
}

/// ```ignore
/// File ⮕ Item*
///```
pub fn parse_file(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> Vec<Item> {
	let mut items = vec![];
	while !token_stream.is_empty() {
		match parse_item(handler, token_stream) {
			Ok(item) => items.push(item),
			Err(err) => {
				handler.emit(err);
				token_stream.consume_while([
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
pub fn parse_item(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	let Token { kind: item_kind, .. } = token_stream.expect([
		TokenKind::Module,
		TokenKind::Use,
		TokenKind::Fn,
		TokenKind::Enum,
		TokenKind::Struct,
		TokenKind::Trait,
		TokenKind::Impl,
	])?;
	match item_kind {
		TokenKind::Module => parse_module(handler, token_stream),
		TokenKind::Use => parse_use(handler, token_stream),
		TokenKind::Fn => parse_function(handler, token_stream),
		TokenKind::Enum => parse_enum(handler, token_stream),
		TokenKind::Struct => parse_struct(handler, token_stream),
		TokenKind::Trait => parse_trait(handler, token_stream),
		TokenKind::Impl => parse_impl(handler, token_stream),
		_ => unreachable!(),
	}
}

/// Assumes that the `module` keyword has already been consumed
/// ```ignore
/// Module ⮕ "module" Ident "{" Item* "}"
/// ```
fn parse_module(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	let name = token_stream.expect_ident()?;

	let (items, delim_span) =
		parse_delimited(handler, token_stream, Delimiter::Brace, |handler, token_stream| {
			parse_item(handler, token_stream)
		})?;

	Ok(Item::Module { name, items, delim_span })
}

/// Assumes that the `use` keyword has already been consumed
/// ```ignore
/// Use ⮕ "use" UseTree ItemEnd
/// UseTree ⮕ Path | Path "::" "*" | Path "::" "{" (UseTree ("," UseTree)*)? "}"
/// ```
fn parse_use(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	todo!()
}

/// Assumes that the `fn` keyword has already been consumed
/// ```ignore
/// Function ⮕ "fn" Ident "(" FunctionParams? ")" ("->" Type)? BlockExpr
/// FunctionParams ⮕ FunctionParam ("," FunctionParam)* ","?
/// FunctionParam ⮕ ".."? Ident (":" Path)? ("=" Expr)?
/// ```
fn parse_function(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	let name = parse_ident_and_handle_err(handler, token_stream);

	let (params, _) = parse_delimited(handler, token_stream, Delimiter::Paren, |_, token_stream| {
		let rest_param = token_stream.matches(TokenKind::DotDot).is_some();
		let name = parse_ident_and_handle_err(handler, token_stream);

		// TODO Use path insted of ident
		let typ = token_stream
			.matches(TokenKind::Colon)
			.and_then(|_| token_stream.expect_ident().ok_or_emit(handler));

		let default = token_stream.matches(TokenKind::Eq).and_then(|_| {
			match token_stream.expect([TokenKind::AnyLiteral]) {
				Ok(Token { kind, span }) => {
					let lit = match kind {
						TokenKind::Literal(lit) => lit,
						_ => unreachable!(),
					};
					Some(Expr::Literal { lit, span })
				}
				Err(err) => {
					handler.emit(err);
					token_stream.consume_until([TokenKind::Comma, TokenKind::CloseDelim(Delimiter::Paren)]);
					None
				}
			}
		});

		token_stream.matches(TokenKind::Comma);
		Ok(FunctionParam { rest_param, name, typ, default })
	})?;

	// TODO Use path insted of ident
	let return_type = token_stream
		.matches(TokenKind::ThinArrow)
		.and_then(|_| token_stream.expect_ident().ok_or_emit(handler));

	let body = parse_block(handler, token_stream)?;

	Ok(Item::Function { name, params, return_type, body })
}

/// Assumes that the `enum` keyword has already been consumed
/// ```ignore
/// Enum ⮕ "enum" Ident "{" EnumItem ("," EnumItem)* ","? "}"
/// EnumItem ⮕ Ident ("(" TupleFields? ")" | "{" StructFields? "}")?
/// ```
fn parse_enum(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	todo!()
}

/// Assumes that the `struct` keyword has already been consumed
/// ```ignore
/// Struct ⮕ "struct" Ident ("(" TupleFields? ")" | "{" StructFields? "}" | ItemEnd)
/// TupleFields ⮕ TupleField ("," TupleField)* ","?
/// TupleField ⮕ (Ident ":")? Type ("=" Expr)?
/// StructFields ⮕ StructField ("," StructField)* ","?
/// StructField ⮕ Ident ":" Type ("=" Expr)?
/// ```
fn parse_struct(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	todo!()
}

/// Assumes that the `trait` keyword has already been consumed
/// ```ignore
/// Trait ⮕ "trait" Ident "{" __TODO__ "}"
/// ```
fn parse_trait(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	todo!()
}

/// Assumes that the `impl` keyword has already been consumed
/// ```ignore
/// Implementation ⮕ "impl" Path ("for" Path) "{" __TODO__ "}"
/// ```
fn parse_impl(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Item> {
	todo!()
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
pub fn parse_expr(
	_handler: &DiagnosticHandler,
	_token_stream: &mut TokenStream,
) -> RymResult<Expr> {
	todo!()
}

fn parse_block(handler: &DiagnosticHandler, token_stream: &mut TokenStream) -> RymResult<Block> {
	let (stmts, span) = parse_delimited(handler, token_stream, Delimiter::Brace, |_, _| todo!())?;
	Ok(Block { stmts, span })
}

/// The next token has to be the opening variant of the delimiter
/// Executes parse_inner_f until the closing variant of the delimiter is seen
/// Returns an error with diagnostics if it is unclosed
fn parse_delimited<T: Debug>(
	handler: &DiagnosticHandler,
	token_stream: &mut TokenStream,
	delim: Delimiter,
	parse_inner_f: impl Fn(&DiagnosticHandler, &mut TokenStream) -> RymResult<T>,
) -> RymResult<(Vec<T>, DelimSpan)> {
	dbg!(delim);
	let Token { span: open, .. } = token_stream.expect(TokenKind::OpenDelim(delim))?;
	let mut items = vec![];

	let close = loop {
		dbg!(delim);
		if token_stream.is_empty() {
			let last = token_stream.previous_span();
			handler.emit(Diagnostic::new_spanned(
				Level::Error,
				"Unclosed module block",
				Span::new(open.start, last.end),
			));
			break last;
		}

		match token_stream.matches(TokenKind::CloseDelim(delim)) {
			Some(Token { span, .. }) => break span,
			None => parse_inner_f(handler, token_stream).ok_or_emit(handler).map(|item| items.push(item)),
		};
	};

	Ok((items, DelimSpan { open, close, entire: Span::new(open.start, close.end) }))
}

fn parse_ident_and_handle_err(
	handler: &DiagnosticHandler,
	token_stream: &mut TokenStream,
) -> (SmolStr, Span) {
	token_stream
		.expect_ident()
		.map_err(|err| handler.emit(err))
		.unwrap_or((SmolStr::new("<empty>"), DUMMY_SPAN))
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
