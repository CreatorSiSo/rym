use indoc::indoc;
use rym_errors::RymResult;
use rym_errors::{Diagnostic, DiagnosticHandler};
use rym_parser::{lexer::LitKind, parse_file_from_src, BinaryOp, Block, Expr, Item, Stmt};
use rym_parser::{FunctionParam, UnaryOp};
use rym_span::DelimSpan;
use smol_str::SmolStr;
use std::ops::Range;

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
	return_type: Option<(&str, Range<usize>)>,
	body: Block,
) -> Item {
	Item::Function {
		name: (SmolStr::new(name.0), name.1.into()),
		params,
		return_type: return_type.map(|typ| (SmolStr::new(typ.0), typ.1.into())),
		body,
	}
}

fn fn_param(name: (&str, Range<usize>), typ: Option<(&str, Range<usize>)>) -> FunctionParam {
	fn_rest_param(false, name, typ)
}

fn fn_rest_param(
	rest_param: bool,
	name: (&str, Range<usize>),
	typ: Option<(&str, Range<usize>)>,
) -> FunctionParam {
	fn_rest_param_default(rest_param, name, typ, None)
}

fn fn_param_default(
	name: (&str, Range<usize>),
	typ: Option<(&str, Range<usize>)>,
	default: Option<Expr>,
) -> FunctionParam {
	fn_rest_param_default(false, name, typ, default)
}

fn fn_rest_param_default(
	rest_param: bool,
	name: (&str, Range<usize>),
	typ: Option<(&str, Range<usize>)>,
	default: Option<Expr>,
) -> FunctionParam {
	FunctionParam {
		rest_param,
		name: (name.0.into(), name.1.into()),
		typ: typ.map(|typ| (typ.0.into(), typ.1.into())),
		default,
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

#[test]
fn pratt() {
	enum Token {
		Number(f64),
		Plus,
		Minus,
		Star,
		Slash,
		LParen,
		RParen,
		EOF,
	}

	struct Parser {
		tokens: Vec<Token>,
		current: usize,
	}

	#[derive(Debug, PartialEq)]
	enum Expr {
		Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
		Unary { op: UnaryOp, right: Box<Expr> },
		Number(f64),
	}

	impl Parser {
		fn parse_expression(&mut self, precedence: i32) -> Expr {
			let mut left = self.parse_prefix();

			while precedence < self.get_token_precedence() {
				left = self.parse_infix(left);
			}

			left
		}

		fn parse_prefix(&mut self) -> Expr {
			match self.tokens[self.current] {
				Token::Number(num) => {
					self.current += 1;
					Expr::Number(num)
				}
				Token::Minus => {
					self.current += 1;
					Expr::Unary { op: UnaryOp::Neg, right: Box::new(self.parse_expression(100)) }
				}
				Token::LParen => {
					self.current += 1;
					let node = self.parse_expression(0);
					self.current += 1;
					node
				}
				_ => {
					panic!()
				}
			}
		}

		fn parse_infix(&mut self, left: Expr) -> Expr {
			let precedence = self.get_token_precedence();
			let token = &self.tokens[self.current];
			self.current += 1;

			match token {
				Token::Plus => Expr::Binary {
					op: BinaryOp::Add,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Minus => Expr::Binary {
					op: BinaryOp::Sub,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Star => Expr::Binary {
					op: BinaryOp::Mul,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Slash => Expr::Binary {
					op: BinaryOp::Div,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				_ => {
					panic!()
				}
			}
		}

		fn get_token_precedence(&mut self) -> i32 {
			let token = &self.tokens[self.current];

			match token {
				Token::Plus | Token::Minus => 2,
				Token::Star | Token::Slash => 3,
				Token::LParen | Token::RParen => 1,
				_ => -1,
			}
		}
	}

	let tokens = vec![
		Token::Number(2.0),
		Token::Plus,
		Token::Minus,
		Token::Number(3.0),
		Token::Star,
		Token::Number(4.0),
		Token::EOF,
	];

	let mut parser = Parser { tokens, current: 0 };
	let ast = parser.parse_expression(0);

	assert_eq!(
		ast,
		Expr::Binary {
			op: BinaryOp::Add,
			left: Box::new(Expr::Number(2.0)),
			right: Box::new(Expr::Binary {
				op: BinaryOp::Mul,
				left: Box::new(Expr::Unary { op: UnaryOp::Neg, right: Box::new(Expr::Number(3.0)) }),
				right: Box::new(Expr::Number(4.0))
			})
		}
	);

	let tokens = vec![Token::Number(2.0), Token::EOF];

	let mut parser = Parser { tokens, current: 0 };
	let ast = parser.parse_expression(0);

	assert_eq!(ast, Expr::Number(2.0));
}
