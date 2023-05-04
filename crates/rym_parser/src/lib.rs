mod error;
mod tests;

use error::{Label, ParseError};
use rym_ast::{BinaryOp, Expr, Func, Item, Literal, Module, Span, Spanned, Stmt, UnaryOp, Var};

use chumsky::input::SpannedInput;
use chumsky::prelude::*;
use chumsky::util::MaybeRef;
use rym_lexer::rich::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseResult<'a, T> {
	pub ast: Option<T>,
	pub errors: Vec<ParseError<'a>>,
}

impl<'a, T> From<chumsky::ParseResult<T, ErrorAlias<'a>>> for ParseResult<'a, T> {
	fn from(value: chumsky::ParseResult<T, ErrorAlias<'a>>) -> Self {
		let (ast, errors) = value.into_output_errors();
		let errors: Vec<_> = errors.into_iter().map(|err| err.into()).collect();
		Self { ast, errors }
	}
}

pub(crate) type InputAlias<'a> = SpannedInput<Token, Span, &'a [(Token, Span)]>;
pub(crate) type ExtraAlias<'a> = extra::Err<ErrorAlias<'a>>;
pub(crate) type ErrorAlias<'a> = Rich<'a, Token, Span, Label>;

pub fn parse_module_file(tokens: &[(Token, Span)]) -> ParseResult<Vec<Spanned<Item>>> {
	module_file_parser()
		.parse(tokens.spanned(tokens.len()..tokens.len()))
		.into()
}

fn module_file_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Vec<Spanned<Item>>, ExtraAlias<'a>> {
	item_parser(expr_parser()).repeated().collect()
}

pub fn parse_script_file(tokens: &[(Token, Span)]) -> ParseResult<Vec<Spanned<Stmt>>> {
	script_file_parser()
		.parse(tokens.spanned(tokens.len()..tokens.len()))
		.into()
}

fn script_file_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Vec<Spanned<Stmt>>, ExtraAlias<'a>> {
	stmt_parser(expr_parser()).repeated().collect()
}

pub fn parse_item(tokens: &[(Token, Span)]) -> ParseResult<Spanned<Item>> {
	item_parser(expr_parser())
		.parse(tokens.spanned(tokens.len()..tokens.len()))
		.into()
}

fn item_parser<'a>(
	expr: impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone + 'a,
) -> impl Parser<'a, InputAlias<'a>, Spanned<Item>, ExtraAlias<'a>> + Clone {
	recursive(|item| {
		let module = just(Token::Mod)
			.ignore_then(ident_parser())
			.then(
				item
					.repeated()
					.collect()
					.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
					.or(
						just(Token::Semi)
							.to(vec![])
							.recover_with(via_parser(empty().to(vec![]))),
					)
					.recover_with(via_parser(custom_nested_delimiters(
						Token::OpenBrace,
						Token::CloseBrace,
						[
							(Token::OpenParen, Token::CloseParen),
							(Token::OpenBracket, Token::CloseBracket),
						],
						|_| vec![],
					))),
			)
			.map_with_span(|(name, items), span| Item::Module(Spanned(Module { name, items }, span)));

		let function = just(Token::Func)
			.ignore_then(ident_parser())
			.then(
				ident_parser()
					.separated_by(just(Token::Comma))
					.allow_trailing()
					.collect()
					.delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
			)
			.then(choice((
				expr.clone().map(Some),
				just(Token::Semi)
					.to(None)
					.recover_with(via_parser(any().rewind().to(None))),
			)))
			.map_with_span(|((name, params), rhs), span| {
				Item::Func(Spanned(Func { name, params, rhs }, span))
			});

		let binding = just(Token::Let)
			.ignore_then(ident_parser())
			.then_ignore(just(Token::Colon).then(ident_parser()).or_not())
			.then_ignore(just(Token::Eq))
			.then(expr)
			.then_ignore(just(Token::Semi))
			.map_with_span(|(name, rhs), span| Item::Var(Spanned(Var { name, rhs }, span)));

		choice((
			module.labelled(Label::Module),
			function.labelled(Label::Function),
			binding.labelled(Label::Binding),
		))
		.map_with_span(Spanned)
	})
}

fn stmt_parser<'a>(
	expr: impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone + 'a,
) -> impl Parser<'a, InputAlias<'a>, Spanned<Stmt>, ExtraAlias<'a>> + Clone {
	choice((
		item_parser(expr.clone()).map(Stmt::Item),
		expr.then_ignore(just(Token::Semi)).map(Stmt::Expr),
	))
	.map_with_span(Spanned)
}

pub fn parse_expr(tokens: &[(Token, Span)]) -> ParseResult<Spanned<Expr>> {
	expr_parser()
		.parse(tokens.spanned(tokens.len()..tokens.len()))
		.into()
}

fn expr_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone {
	recursive(|expr| {
		// atom => group | literal | IDENT
		let atom = {
			// literal => INT | FLOAT | CHAR | STRING
			let literal = select! {
				Token::Int(val) => Literal::Int(val),
				Token::Float(val) => Literal::Float(val.parse().unwrap()),
				Token::Char(val) => Literal::Char(val),
				Token::String(val) => Literal::String(val),
			}
			.map_with_span(Spanned)
			.map_with_span(|lit, span| Spanned(Expr::Literal(lit), span))
			.labelled(Label::Literal);

			// group => "(" expr ")"
			let group = expr
				.clone()
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				// Attempt to recover anything that looks like a group but contains errors
				.recover_with(via_parser(custom_nested_delimiters(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| Spanned(Expr::Error, span),
				)))
				.labelled(Label::Group);

			group.or(
				literal.or(
					ident_parser()
						.map(|ident| Expr::Ident(ident))
						.map_with_span(Spanned),
				),
			)
		};

		// items => expr (expr ",")* ","?
		let items = expr
			.clone()
			.separated_by(just(Token::Comma))
			.allow_trailing()
			.collect::<Vec<_>>();

		// call => atom "(" items? ")"
		let call = atom.clone().foldl(
			items
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				.recover_with(via_parser(custom_nested_delimiters(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| vec![Spanned(Expr::Error, span)],
				)))
				.map_with_span(Spanned)
				.repeated(),
			|func, args| {
				let span = func.1.start..args.1.end;
				Spanned(
					Expr::Call {
						func: Box::new(func),
						args,
					},
					span,
				)
			},
		);

		// Unary operators (not and negate) have equal precedence
		// unary => ("!" | "-") call
		let unary = {
			let op = choice((
				just(Token::Bang).to(UnaryOp::Not),
				just(Token::Minus).to(UnaryOp::Neg),
			))
			.map_with_span(Spanned);

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

		// Logical operators (and and or) have equal precedence
		// logical => sum ("and" | "or") sum
		let logical = {
			let op = select! {
				Token::And => BinaryOp::And,
				Token::Or => BinaryOp::Or,
			};
			sum.clone().foldl(
				op.then(sum).repeated(),
				|lhs: Spanned<Expr>, (op, rhs): (_, Spanned<Expr>)| {
					let span = lhs.1.start..rhs.1.end;
					Spanned(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				},
			)
		};

		// Comparison operators (equal and not-equal) have equal precedence
		// comp => logical ("==" | "!=") logical
		let comp = {
			let op = select! {
				Token::EqEq => BinaryOp::Eq,
				Token::BangEq => BinaryOp::Ne,
				Token::GreaterThan => BinaryOp::Gt,
				Token::LessThan => BinaryOp::Lt,
			};
			logical.clone().foldl(
				op.then(logical).repeated(),
				|lhs: Spanned<Expr>, (op, rhs): (_, Spanned<Expr>)| {
					let span = lhs.1.start..rhs.1.end;
					Spanned(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
				},
			)
		};

		let raw_expr = comp;

		// assign => atom "=" expr
		let assign = atom
			.then_ignore(just(Token::Eq))
			.then(expr.clone())
			.map_with_span(|(lhs, rhs), span| Spanned(Expr::Assign(Box::new(lhs), Box::new(rhs)), span));

		// block => "{" stmt* "}"
		let block = stmt_parser(expr.clone())
			.repeated()
			.collect()
			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
			.map_with_span(Spanned)
			.map_with_span(|stmts, span| Spanned(Expr::Block(stmts), span))
			// Attempt to recover anything that looks like a block but contains errors
			.recover_with(via_parser(custom_nested_delimiters(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenBracket, Token::CloseBracket),
					(Token::OpenParen, Token::CloseParen),
				],
				|span| Spanned(Expr::Block(Spanned(vec![], span.clone())), span),
			)))
			.labelled(Label::Block);

		// control_flow => continue | break | return | loop | if
		let control_flow = {
			// continue => "continue" expr
			let continue_ = just(Token::Continue)
				.map(|_| Expr::Continue)
				.labelled(Label::Continue);

			// break => "break" expr
			let break_ = just(Token::Break)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Break(Box::new(expr)))
				.labelled(Label::Break);

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
					.recover_with(via_parser(custom_nested_delimiters(
						Token::If,
						Token::Then,
						[
							(Token::OpenParen, Token::CloseParen),
							(Token::OpenBrace, Token::CloseBrace),
							(Token::OpenBracket, Token::CloseBracket),
						],
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

			choice((continue_, break_, return_, loop_, if_)).map_with_span(Spanned)
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
			.map(Some)
			.or(just(Token::Dot).map(|_| None))
			.then(record_fields.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
			.map_with_span(|(name, fields), span| Spanned(Expr::Record { name, fields }, span))
			.labelled(Label::Record);

		choice((record, assign, raw_expr, block, control_flow)).labelled(Label::Expression)
	})
}

pub(crate) fn custom_nested_delimiters<'a, O, const N: usize>(
	start: Token,
	end: Token,
	others: [(Token, Token); N],
	fallback: impl Fn(Span) -> O + Clone,
) -> impl Parser<'a, InputAlias<'a>, O, ExtraAlias<'a>> + Clone {
	custom(move |input_ref| {
		// TODO Make this work properly
		use std::cmp::Ordering;
		let err_unexpected = chumsky::error::Error::<InputAlias<'a>>::expected_found;

		let mut balance = 0;
		let mut balance_others = [0; N];
		let mut starts = Vec::new();
		let mut error = None;
		let start_offset = input_ref.offset();

		let recovered = loop {
			if match input_ref.next() {
				Some(t) if t == start => {
					balance += 1;
					starts.push(input_ref.offset());
					true
				}
				Some(t) if t == end => {
					balance -= 1;
					starts.pop();
					true
				}
				Some(t) => {
					for (balance_other, others) in balance_others.iter_mut().zip(others.iter()) {
						if t == others.0 {
							*balance_other += 1;
						} else if t == others.1 {
							*balance_other -= 1;

							// if *balance_other < 0 && balance == 1 {
							// input_ref.rewind(pre_state);
							// error.get_or_insert_with(|| {
							// 	Located::at(
							// 		at,
							// 		P::Error::unclosed_delimiter(
							// 			starts.pop().unwrap(),
							// 			start.clone(),
							// 			span.clone(),
							// 			end.clone(),
							// 			Some(t.clone()),
							// 		),
							// 	)
							// });
							// }
						}
					}
					false
				}
				None => {
					if balance > 0 && balance == 1 {
						error.get_or_insert_with(|| match starts.pop() {
							Some(start_offset) => err_unexpected(
								[Some(MaybeRef::Val(end.clone()))],
								None,
								input_ref.span_since(start_offset),
							),
							None => err_unexpected(
								[Some(MaybeRef::Val(end.clone()))],
								None,
								input_ref.span_since(input_ref.offset()),
							),
							// Some(start) => Located::at(
							// 	at,
							// 	P::Error::unclosed_delimiter(
							// 		start,
							// 		start.clone(),
							// 		span,
							// 		end.clone(),
							// 		None,
							// 	),
							// ),
							// None => Located::at(
							// 	at,
							// 	P::Error::expected_input_found(span, Some(Some(end.clone())), None),
							// ),
						});
					}
					break false;
				}
			} {
				match balance.cmp(&0) {
					Ordering::Equal => break true,
					// The end of a delimited section is not a valid recovery pattern
					Ordering::Less => break false,
					Ordering::Greater => (),
				}
			} else if balance == 0 {
				// A non-delimiter input before anything else is not a valid recovery pattern
				break false;
			}
		};

		let complete_span = input_ref.span_since(start_offset);

		if recovered {
			Ok(fallback(complete_span))
		} else if let Some(error) = error {
			Err(error)
		} else {
			Err(err_unexpected(
				[Some(MaybeRef::Val(end.clone()))],
				input_ref.next().map(MaybeRef::Val),
				input_ref.span_since(input_ref.offset()),
			))
		}
	})
}

fn ident_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Spanned<String>, ExtraAlias<'a>> + Clone {
	select! { Token::Ident(ident) => ident }
		.map_with_span(Spanned)
		.labelled(Label::Identifier)
}
