mod ast;
mod error;
mod tests;

use ast::{BinaryOp, Expr, Item, Stmt, UnaryOp};
use error::ParseError;

use chumsky::input::{Input, SpannedInput};
use chumsky::prelude::*;
use rym_lexer::rich::{Lexer, Token};

pub(crate) type Span = std::ops::Range<usize>;
#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T>(T, Span);

pub(crate) type ParserInput<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
pub(crate) type Extra = extra::Err<Rich<Token, Span>>;

pub struct ParseResult<T>(pub Option<T>, pub Vec<ParseError>);

impl<T> From<chumsky::ParseResult<T, Rich<Token, Span>>> for ParseResult<T> {
	fn from(value: chumsky::ParseResult<T, Rich<Token, Span>>) -> Self {
		let (output, errors) = value.into_output_errors();
		let errors: Vec<ParseError> = errors.into_iter().map(|err| err.into()).collect();
		Self(output, errors)
	}
}

pub fn parse_module_file<'a>(src: &'a str) -> ParseResult<Vec<Spanned<Item>>> {
	parse_str(|tokens| module_file_parser().parse(tokens).into(), src)
}

pub fn parse_expr<'a>(src: &'a str) -> ParseResult<Spanned<Expr>> {
	parse_str(|tokens| expr_parser().parse(tokens).into(), src)
}

pub(crate) fn parse_str<'a, 'b, T>(
	parse_fn: fn(ParserInput<'_>) -> ParseResult<T>,
	src: &'a str,
) -> ParseResult<T>
where
	ParseResult<T>: From<chumsky::prelude::ParseResult<T, Rich<Token, Span>>>,
{
	let tokens: Vec<(Token, Span)> = Lexer::new(src).collect();
	parse_fn(tokens.as_slice().spanned(tokens.len()..tokens.len())).into()
}

fn module_file_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Vec<Spanned<Item>>, Extra> {
	item_parser(expr_parser()).repeated().collect()
}

fn item_parser<'a>(
	expr_parser: impl Parser<'a, ParserInput<'a>, Spanned<Expr>, Extra> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, Spanned<Item>, Extra> + Clone {
	/// Helper function to recover a parser that expects a block or a semicolon
	fn recover_block_or_semi<'a, O>(
		parser: impl Parser<'a, ParserInput<'a>, O, Extra> + Clone,
		fallback: fn(Span) -> O,
	) -> impl Parser<'a, ParserInput<'a>, O, Extra> + Clone {
		parser
			.recover_with(via_parser(nested_delimiters(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenParen, Token::CloseParen),
					(Token::OpenBracket, Token::CloseBracket),
				],
				fallback,
			)))
			.recover_with(via_parser(
				empty().map_with_span(move |_, span| fallback(span)),
			))
	}

	recursive(|item| {
		let module = just(Token::Mod)
			.ignore_then(ident_parser().or_not())
			.then(recover_block_or_semi(
				choice((
					just(Token::Semi).to(vec![]),
					item.repeated()
						.collect()
						.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)),
				)),
				|_| vec![],
			))
			.map(|(name, items)| Item::Module { name, items });

		let function = just(Token::Func)
			.ignore_then(ident_parser())
			.then(
				ident_parser()
					.separated_by(just(Token::Comma))
					.allow_trailing()
					.collect()
					.delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
			)
			.then(recover_block_or_semi(
				choice((
					expr_parser.clone().map(|expr| Some(expr)),
					just(Token::Semi).to(None),
				)),
				|_| None,
			))
			.map(|((name, params), rhs)| Item::Func { name, params, rhs });

		let binding = just(Token::Let)
			.ignore_then(ident_parser())
			.then_ignore(just(Token::Eq))
			.then(expr_parser)
			.then_ignore(just(Token::Semi))
			.map(|(name, rhs)| Item::Binding { name, rhs });

		choice((
			module,   /* .labelled(Label::Module) */
			function, /* .labelled(Label::Function) */
			binding,  /* .labelled(Label::Binding) */
		))
		.map_with_span(|item, span| Spanned(item, span))
	})
}

pub fn expr_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Spanned<Expr>, Extra> + Clone {
	recursive(|expr| {
		let stmt = {
			choice((
				item_parser(expr.clone()).map(|item| Stmt::Item(item)),
				expr.clone()
					.then_ignore(just(Token::Semi))
					.map(|expr| Stmt::Expr(expr)),
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
			.map_with_span(|expr, span| Spanned(expr, span));

			// group => "(" expr ")"
			let group = expr
				.clone()
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				// Attempt to recover anything that looks like a group but contains errors
				.recover_with(via_parser(nested_delimiters(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| Spanned(Expr::Error, span),
				)));
			// .labelled(Label::Group)

			choice((
				group,
				literal,
				ident_parser().map(|Spanned(ident, span)| Spanned(Expr::Ident(ident), span)),
			))
		};

		// items => expr (expr ",")* ","?
		let items = expr
			.clone()
			.separated_by(just(Token::Comma))
			.allow_trailing()
			.collect::<Vec<_>>();

		// call => atom "(" items? ")"
		let call = atom.foldl(
			items
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				.recover_with(via_parser(nested_delimiters(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| vec![Spanned(Expr::Error, span)],
				)))
				.map_with_span(|args, span| Spanned(args, span))
				.repeated(),
			|spanned_func, Spanned(args, args_span)| {
				let span = spanned_func.1.start..args_span.end;
				Spanned(Expr::Call(Box::new(spanned_func), args), span)
			},
		);

		// Unary operators (not and negate) have equal precedence
		// unary => ("!" | "-") call
		let unary = {
			let op = choice((
				just(Token::Bang).to(UnaryOp::Not),
				just(Token::Minus).to(UnaryOp::Neg),
			))
			.map_with_span(|op, span| Spanned(op, span));

			op.repeated()
				.foldr(call, |Spanned(op, op_span), rhs: Spanned<Expr>| {
					let span = op_span.start..rhs.1.end;
					Spanned(Expr::Unary(op, Box::new(rhs)), span)
				})
		};

		// Product operators (multiply and divide) have equal precedence
		// product => ("*" | "")
		let product = {
			let op = choice((
				just(Token::Star).to(BinaryOp::Mul),
				just(Token::Slash).to(BinaryOp::Div),
			));
			unary.clone().foldl(
				op.then(unary).repeated(),
				|lhs: Spanned<Expr>, (op, rhs): (_, Spanned<Expr>)| {
					let span = lhs.1.start..rhs.1.end;
					Spanned(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				},
			)
		};

		// Sum operators (add and subtract) have equal precedence
		// sum => product ("+" | "-") product
		let sum = {
			let op = choice((
				just(Token::Plus).to(BinaryOp::Add),
				just(Token::Minus).to(BinaryOp::Sub),
			));
			product.clone().foldl(
				op.then(product).repeated(),
				|lhs: Spanned<Expr>, (op, rhs): (_, Spanned<Expr>)| {
					let span = lhs.1.start..rhs.1.end;
					Spanned(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				},
			)
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
			sum.clone().foldl(
				op.then(sum).repeated(),
				|lhs: Spanned<Expr>, (op, rhs): (_, Spanned<Expr>)| {
					let span = lhs.1.start..rhs.1.end;
					Spanned(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				},
			)
		};

		let raw_expr = comp/* comp */;

		// assign => IDENT "=" expr
		let assign = ident_parser()
			.then_ignore(just(Token::Eq))
			.then(expr.clone())
			.map_with_span(|(name, rhs), span| {
				Spanned(
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
			.collect()
			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
			.map_with_span(|stmts, span| Spanned(Expr::Block(stmts), span))
			// Attempt to recover anything that looks like a block but contains errors
			.recover_with(via_parser(nested_delimiters(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenBracket, Token::CloseBracket),
					(Token::OpenParen, Token::CloseParen),
				],
				|span| Spanned(Expr::Block(vec![Stmt::Error]), span),
			)));
		// .labelled(Label::Block)

		// control_flow => break | continue | return | loop | if
		let control_flow = {
			// break => "break" expr
			let break_ = just(Token::Break)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Break(Box::new(expr)));
			// .labelled(Label::Break)

			// continue => "continue" expr
			let continue_ = just(Token::Continue).map(|_| Expr::Continue); // .labelled(Label::Continue)

			// return => "return" expr
			let return_ = just(Token::Return)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Return(Box::new(expr)));
			// .labelled(Label::Return)

			// loop => "loop" expr
			let loop_ = just(Token::Loop)
				.ignore_then(expr.clone())
				.map(|expr| Expr::Loop(Box::new(expr)));
			// .labelled(Label::Loop)

			// if => "if" expr "then" expr ("else" expr)?
			let if_ = recursive(|if_| {
				just(Token::If)
					.ignore_then(expr.clone())
					.then_ignore(just(Token::Then))
					.recover_with(via_parser(nested_delimiters(
						Token::If,
						Token::Then,
						[],
						|span| Spanned(Expr::Error, span),
					)))
					.then(expr.clone())
					.then(
						just(Token::Else)
							.ignore_then(expr.clone())
							.or(if_.map_with_span(Spanned))
							.or_not(),
					)
					.map(|((condition, then_branch), else_branch)| Expr::If {
						condition: Box::new(condition),
						then_branch: Box::new(then_branch),
						else_branch: Box::new(else_branch),
					})
			});

			choice((break_, continue_, return_, loop_, if_))
				.map_with_span(|expr, span| Spanned(expr, span))
		};

		// record => (IDENT | ".") "{" record_fields? "}"
		// record_fields => record_field ("," record_field)* ","?
		// record_field => IDENT ":" expr
		let record_field = ident_parser().then_ignore(just(Token::Colon)).then(expr);
		let record_fields = record_field
			.separated_by(just(Token::Comma))
			.allow_trailing()
			.collect::<Vec<_>>();
		let record = ident_parser()
			.map(|ident| Some(ident))
			.or(just(Token::Dot).map(|_| None))
			.then(record_fields.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
			.map_with_span(|(name, fields), span| Spanned(Expr::Record { name, fields }, span));
		// .labelled(Label::Record)

		choice((record, assign, raw_expr, block, control_flow)) /* .labelled(Label::Expression) */
	})
}

fn ident_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Spanned<String>, Extra> + Clone {
	select! { Token::Ident(ident) => ident }
		.map_with_span(|ident, span: std::ops::Range<usize>| Spanned(ident, span.into()))
	// .labelled(Label::Identifier)
}
