mod ast;
mod error;
mod tests;

use ast::{BinaryOp, Expr, Item, Stmt, UnaryOp};
use error::{Label, ParseResult};

use chumsky::input::SpannedInput;
use chumsky::prelude::*;
use chumsky::util::MaybeRef;
use rym_lexer::rich::{Lexer, Token};

pub(crate) type Span = std::ops::Range<usize>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T>(T, Span);

pub(crate) type InputAlias<'a> = SpannedInput<Token, Span, &'a [(Token, Span)]>;
pub(crate) type ExtraAlias<'a> = extra::Err<ErrorAlias<'a>>;
pub(crate) type ErrorAlias<'a> = Rich<'a, Token, Span, Label>;

pub fn parse_module_file<'a>(src: &'a str) -> ParseResult<Vec<Spanned<Item>>> {
	parse_str(|tokens| module_file_parser().parse(tokens).into(), src)
}

pub fn parse_expr<'a>(src: &'a str) -> ParseResult<Spanned<Expr>> {
	parse_str(|tokens| expr_parser().parse(tokens).into(), src)
}

pub(crate) fn parse_str<'a, T>(
	parse_fn: fn(InputAlias) -> ParseResult<T>,
	src: &'a str,
) -> ParseResult<T>
where
	ParseResult<'a, T>: From<chumsky::prelude::ParseResult<T, ErrorAlias<'a>>>,
{
	// TODO Fix the leak
	let tokens = Lexer::new(src).collect::<Vec<(Token, Span)>>().leak();
	parse_fn(tokens.spanned(tokens.len()..tokens.len())).into()
}

fn module_file_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Vec<Spanned<Item>>, ExtraAlias<'a>> {
	item_parser(expr_parser()).repeated().collect()
}

fn item_parser<'a>(
	expr_parser: impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone + 'a,
) -> impl Parser<'a, InputAlias<'a>, Spanned<Item>, ExtraAlias<'a>> + Clone {
	/// Helper function to recover a parser that expects a block or a semicolon
	fn recover_block_or_semi<'a, O>(
		parser: impl Parser<'a, InputAlias<'a>, O, ExtraAlias<'a>> + Clone,
		fallback: fn(Span) -> O,
	) -> impl Parser<'a, InputAlias<'a>, O, ExtraAlias<'a>> + Clone {
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
			module.labelled(Label::Module),
			function.labelled(Label::Function),
			binding.labelled(Label::Binding),
		))
		.map_with_span(|item, span| Spanned(item, span))
	})
}

pub fn expr_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Spanned<Expr>, ExtraAlias<'a>> + Clone {
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
			.map_with_span(|expr, span| Spanned(expr, span))
			.labelled(Label::Literal);

			// group => "(" expr ")"
			let group = expr
				.clone()
				.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
				// Attempt to recover anything that looks like a group but contains errors
				.recover_with(via_parser(custom_nested_delims(
					Token::OpenParen,
					Token::CloseParen,
					[
						(Token::OpenBrace, Token::CloseBrace),
						(Token::OpenBracket, Token::CloseBracket),
					],
					|span| Spanned(Expr::Error, span),
				)))
				.labelled(Label::Group);

			group.or(literal
				.or(ident_parser().map(|Spanned(ident, span)| Spanned(Expr::Ident(ident), span))))
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
				.recover_with(via_parser(custom_nested_delims(
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

		let raw_expr = comp;

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
			.recover_with(via_parser(custom_nested_delims(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenBracket, Token::CloseBracket),
					(Token::OpenParen, Token::CloseParen),
				],
				|span| Spanned(Expr::Block(vec![Stmt::Error]), span),
			)))
			.labelled(Label::Block);

		// control_flow => break | continue | return | loop | if
		let control_flow = {
			// break => "break" expr
			let break_ = just(Token::Break)
				.ignore_then(expr.clone().or_not())
				.map(|expr| Expr::Break(Box::new(expr)))
				.labelled(Label::Break);

			// continue => "continue" expr
			let continue_ = just(Token::Continue).map(|_| Expr::Continue); // .labelled(Label::Continue)

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
					.recover_with(via_parser(custom_nested_delims(
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
			.map_with_span(|(name, fields), span| Spanned(Expr::Record { name, fields }, span))
			.labelled(Label::Record);

		choice((record, assign, raw_expr, block, control_flow)).labelled(Label::Expression)
	})
}

pub(crate) fn custom_nested_delims<'a, O, const N: usize>(
	start: Token,
	end: Token,
	others: [(Token, Token); N],
	fallback: impl Fn(Span) -> O + Clone,
) -> impl Parser<'a, InputAlias<'a>, O, ExtraAlias<'a>> + Clone {
	custom(move |input_ref| {
		let err_unexpected = chumsky::error::Error::<InputAlias<'a>>::expected_found;

		let all_delims_map: Vec<(&Token, &Token)> = [(&start, &end)]
			.into_iter()
			.chain(others.iter().map(|(s, e)| (s, e)))
			.collect();

		let start_delims: Vec<&Token> = all_delims_map.iter().map(|(s, _)| *s).collect();
		let end_delims: Vec<&Token> = all_delims_map.iter().map(|(_, e)| *e).collect();

		let all_delim_tokens: Vec<&Token> =
			all_delims_map.iter().flat_map(|(s, e)| [*s, *e]).collect();

		let first_offset = input_ref.offset();
		// SAFETY: first_offset was obtained via input_ref.offset() => is a valid offset of input_ref
		let first_span = unsafe { input_ref.span_since(first_offset) };

		let mut delim_stack = vec![{
			let first_token = input_ref.next_token();
			if first_token != Some(start.clone()) {
				return Err(err_unexpected(
					[Some(MaybeRef::Val(start.clone()))],
					first_token.map(|tok| MaybeRef::Val(tok)),
					first_span,
				));
			}
			first_token.unwrap()
		}];

		// consumes input until delimiter or eof is found
		let mut consume_input = || {
			while let Some(token) = input_ref.next_token() {
				if all_delim_tokens.contains(&&token) {
					return Some(token);
				}
			}
			None
		};

		while !delim_stack.is_empty() {
			let Some(delim) = consume_input() else {
				return Err(err_unexpected(
					[Some(MaybeRef::Val(end.clone()))],
					None,
					unsafe { input_ref.span_since(first_offset) },
				));
			};
			delim_stack.push(delim);

			let [.., l, r] = &delim_stack[..] else {
				continue;
			};
			let (l, r) = (l.clone(), r.clone());

			// remove last two delimiters from the stack if
			//   - they belong to a pair eg. ( + ), { + }, [ + ], ..
			//   - the right one is a closing delimiter eg. ( + }, ( + ], { + ), ..
			if all_delims_map.iter().any(|(s, e)| s == &&l && e == &&r) || end_delims.contains(&&r)
			{
				delim_stack.pop();
				delim_stack.pop();
			}
		}

		// SAFETY: see first_span
		Ok(fallback(unsafe { input_ref.span_since(first_offset) }))
	})
}

fn ident_parser<'a>() -> impl Parser<'a, InputAlias<'a>, Spanned<String>, ExtraAlias<'a>> + Clone {
	select! { Token::Ident(ident) => ident }
		.map_with_span(|ident, span: std::ops::Range<usize>| Spanned(ident, span.into()))
		.labelled(Label::Identifier)
}
