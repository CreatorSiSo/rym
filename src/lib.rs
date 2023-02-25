mod ast;
mod error;
mod tests;

use ast::Item;
use chumsky::prelude::*;
use chumsky::Stream;
use rym_lexer::rich::Lexer;

use ast::{BinaryOp, Expr, Stmt, UnaryOp};
use error::{Error, Label};

pub type Token = rym_lexer::rich::Token;
pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

pub fn parse_module_file(src: &str) -> (Option<Vec<Spanned<Item>>>, Vec<Error>) {
	parse_recovery(item_parser(expr_parser()).repeated(), src)
}

pub fn parse_expr(src: &str) -> (Option<Spanned<Expr>>, Vec<Error>) {
	parse_recovery(expr_parser(), src)
}

pub fn parse_recovery<O>(
	parser: impl Parser<Token, O, Error = Simple<Token>>,
	src: &str,
) -> (Option<O>, Vec<Error>) {
	let token_stream = Stream::from_iter(0..0, Lexer::new(src));
	let (ast, errors) = parser.parse_recovery(token_stream);

	let mut reports: Vec<Error> = errors.into_iter().map(Error::from).collect();
	reports.sort();

	(ast, reports)
}

fn item_parser(
	expr_parser: impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, Spanned<Item>, Error = Simple<Token>> {
	recursive(|item| {
		let module = just(Token::Mod)
			.ignore_then(ident_parser())
			.then(
				item.repeated()
					.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)),
			)
			.map(|(name, items)| Item::Module { name, items })
			.labelled(Label::Module);

		let function = just(Token::Func)
			.ignore_then(ident_parser())
			.then(
				ident_parser()
					.separated_by(just(Token::Comma))
					.allow_trailing()
					.delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
			)
			.then(expr_parser.clone())
			.map(|((name, params), rhs)| Item::Func { name, params, rhs })
			.labelled(Label::Function);

		let var = choice((just(Token::Const).to(false), just(Token::Mut).to(true)))
			.then(ident_parser())
			.then_ignore(just(Token::Eq))
			.then(expr_parser)
			.map(|((mutable, name), rhs)| Item::Var { mutable, name, rhs })
			.labelled(Label::Variable);

		choice((module, function, var)).map_with_span(|item, span| (item, span))
	})
}

pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
	recursive(|expr| {
		let stmt = {
			choice((
				item_parser(expr.clone())
					.map(|item| Stmt::Item(item))
					.then_ignore(just(Token::Semi)),
				expr.clone()
					.then(just(Token::Semi).or_not())
					.map(|(expr, semi)| match semi {
						Some(_) => Stmt::Expr(expr),
						None => Stmt::Expr((Expr::NoNewline(Box::new(expr.clone())), expr.1)),
					}),
			))
		};

		// atom => group | literal | IDENT
		let atom = {
			// literal => INT | FLOAT | CHAR | STRING
			let literal = select! {
				Token::Int(val) => Expr::Int(val),
				Token::Float(r_val, l_val) => Expr::Float(r_val, l_val),
				Token::Char(val) => Expr::Char(val),
				Token::String(val) => Expr::String(val),
			}
			// .labelled("literal")
			.map_with_span(|expr, span: Span| (expr, span));

			// group => "(" expr ")"
			let group = expr
				.clone()
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				// Attempt to recover anything that looks like a group but contains errors
				.recover_with(nested_delimiters(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| (Expr::Error, span),
				))
				.labelled(Label::Group);

			choice((
				group,
				literal,
				ident_parser().map(|(ident, span)| (Expr::Ident(ident), span)),
			))
		};

		// items => expr (expr ",")* ","?
		let items = expr
			.clone()
			.separated_by(just(Token::Comma))
			.allow_trailing();

		// call => atom "(" items? ")"
		let call = atom
			.then(
				items
					.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
					// Attempt to recover anything that looks like a call but contains errors
					.recover_with(nested_delimiters(
						Token::OpenParen,
						Token::CloseParen,
						[
							(Token::OpenBrace, Token::CloseBrace),
							(Token::OpenBracket, Token::CloseBracket),
						],
						|span| vec![(Expr::Error, span)],
					))
					.map_with_span(|args, span: Span| (args, span))
					.repeated(),
			)
			.foldl(|spanned_func, (args, args_span)| {
				let span = spanned_func.1.start..args_span.end;
				(Expr::Call(Box::new(spanned_func), args), span)
			});

		// Unary operators (not and negate) have equal precedence
		// unary => ("!" | "-") call
		let unary = {
			let op = choice((
				just(Token::Bang).to(UnaryOp::Not),
				just(Token::Minus).to(UnaryOp::Neg),
			))
			.map_with_span(|op, span: Span| (op, span));

			op.repeated().then(call).foldr(|(op, op_span), rhs| {
				let span = op_span.start..rhs.1.end;
				(Expr::Unary(op, Box::new(rhs)), span)
			})
		};

		// Product operators (multiply and divide) have equal precedence
		// product => ("*" | "")
		let product = {
			let op = choice((
				just(Token::Star).to(BinaryOp::Mul),
				just(Token::Slash).to(BinaryOp::Div),
			));
			unary
				.clone()
				.then(op.then(unary).repeated())
				.foldl(|lhs, (op, rhs)| {
					let span = lhs.1.start..rhs.1.end;
					(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				})
		};

		// Sum operators (add and subtract) have equal precedence
		// sum => product ("+" | "-") product
		let sum = {
			let op = choice((
				just(Token::Plus).to(BinaryOp::Add),
				just(Token::Minus).to(BinaryOp::Sub),
			));
			product
				.clone()
				.then(op.then(product).repeated())
				.foldl(|lhs, (op, rhs)| {
					let span = lhs.1.start..rhs.1.end;
					(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				})
		};

		// Comparison operators (equal and not-equal) have equal precedence
		// comp => sum ("==" | "!=") sum
		let comp = {
			let op = choice((
				just(Token::EqEq).to(BinaryOp::Eq),
				just(Token::BangEq).to(BinaryOp::Neq),
				just(Token::GreaterThan).to(BinaryOp::Greater),
				just(Token::LessThan).to(BinaryOp::Less),
			));
			sum.clone()
				.then(op.then(sum).repeated())
				.foldl(|lhs, (op, rhs)| {
					let span = lhs.1.start..rhs.1.end;
					(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				})
		};

		let raw_expr = comp;

		// assign => IDENT "=" expr
		let assign = ident_parser()
			.then_ignore(just(Token::Eq))
			.then(expr.clone())
			.map_with_span(|(name, rhs), span| {
				(
					Expr::Assign {
						name,
						rhs: Box::new(rhs),
					},
					span,
				)
			});

		// block => "{" stmt* "}"
		let block = stmt
			.repeated()
			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
			// Attempt to recover anything that looks like a block but contains errors
			.map_with_span(|stmts, span| (Expr::Block(stmts), span))
			.recover_with(nested_delimiters(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenBracket, Token::CloseBracket),
					(Token::OpenParen, Token::CloseParen),
				],
				|span| (Expr::Block(vec![Stmt::Error]), span),
			))
			.labelled(Label::Block);

		// control_flow => break | continue | return | loop | if
		let control_flow = {
			// break => "break" expr
			let break_ = just(Token::Break)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Break(Box::new(expr)))
				.labelled(Label::Break);

			// continue => "continue" expr
			let continue_ = just(Token::Continue)
				.map(|_| Expr::Continue)
				.labelled(Label::Continue);

			// return => "return" expr
			let return_ = just(Token::Return)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Return(Box::new(expr)))
				.labelled(Label::Return);

			// loop => "loop" expr
			let loop_ = just(Token::Loop)
				.ignore_then(expr.clone())
				.map(|expr| Expr::Loop(Box::new(expr)))
				.labelled(Label::Loop);

			// if => "if" expr "then" expr ("else" expr)?
			let if_ = recursive(|if_| {
				just(Token::If)
					.ignore_then(expr.clone())
					.then_ignore(just(Token::Then))
					.recover_with(nested_delimiters(Token::If, Token::Then, [], |span| {
						(Expr::Error, span)
					}))
					.then(expr.clone())
					.then(just(Token::Else).ignore_then(expr.clone()).or(if_).or_not())
					.map_with_span(|((condition, then_branch), else_branch), span| {
						(
							Expr::If {
								condition: Box::new(condition),
								then_branch: Box::new(then_branch),
								else_branch: Box::new(else_branch),
							},
							span,
						)
					})
			});

			choice((break_, continue_, return_, loop_))
				.map_with_span(|expr, span| (expr, span))
				.or(if_)
		};

		// record => (IDENT | ".") "{" record_fields? "}"
		// record_fields => record_field ("," record_field)* ","?
		// record_field => IDENT ":" expr
		let record_field = ident_parser().then_ignore(just(Token::Colon)).then(expr);
		let record_fields = record_field
			.separated_by(just(Token::Comma))
			.allow_trailing();
		let record = ident_parser()
			.map(|ident| Some(ident))
			.or(just(Token::Dot).map(|_| None))
			.then(record_fields.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
			.map_with_span(|(name, fields), span| (Expr::Record { name, fields }, span))
			.labelled(Label::Record);

		choice((record, assign, raw_expr, block, control_flow)).labelled(Label::Expression)
	})
}

fn ident_parser() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> + Clone {
	select! { Token::Ident(ident) => ident }
		.map_with_span(|ident, span| (ident, span))
		.labelled(Label::Identifier)
}
