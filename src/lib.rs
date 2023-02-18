mod ast;
mod test;

use ast::{BinaryOp, Expr, Stmt, UnaryOp};
use chumsky::prelude::*;
use rym_lexer::rich::Token;

type Span = std::ops::Range<usize>;
type Spanned<T> = (T, Span);

/// line_end => ";" | EOF
// fn line_end() -> impl TokenParser<()> {
// 	// TODO Look into
// 	// TODO   - automatic semicolon insertion eg. Js
// 	// TODO   - rules for omitting semicolons eg. Go

// 	choice((just(Token::Semi), just(Token::Eof))).ignored()
// }

/// item => func_decl | type_decl | var_decl
// pub fn item() -> impl TokenParser<Item> {
// 	recursive(|item| {
// 		let stmt = choice((
// 			item.map(|item| Stmt::Item(item)),
// 			// expr()
// 			// 	.then_ignore(line_end())
// 			// 	.map(|(expr, _)| Stmt::Expr(expr)),
// 		));

// 		let block = stmt
// 			.repeated()
// 			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
// 			.collect::<Vec<Stmt>>()
// 			.map_with_span(|stmts, span| (Expr::Block(stmts), span))
// 			.labelled("block");

// 		let params = ident()
// 			// .then_ignore(just(Token::Colon))
// 			.separated_by(just(Token::Comma))
// 			.allow_trailing()
// 			.delimited_by(just(Token::OpenParen), just(Token::CloseParen))
// 			.labelled("function params");

// 		// func_decl => "func" IDENT "(" params ")" (block | line_end)
// 		let func_decl = just(Token::Func)
// 			.ignore_then(ident())
// 			.then(params)
// 			// TODO: return type
// 			// .then_ignore(just(Token::ThinArrow))
// 			// .then(type_expr)
// 			.then(block.map(|block| Some(block)).or(line_end().map(|_| None)))
// 			.map(|((name, params), body)| Item::Func { name, params, body });

// 		// let type_decl = just(Token::Type)
// 		// 	.ignore_then(ident())
// 		// 	.map(|name| Item::Type { name });

// 		func_decl /* .or(type_decl) */
// 	})
// }

pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
	let ident = select! { Token::Ident(ident) => ident }
		.map_with_span(|ident, span| (ident, span))
		.labelled("identifier");

	recursive(|expr| {
		let stmt = {
			let var = just(Token::Const)
				.to(false)
				.or(just(Token::Mut).to(true))
				.then(ident)
				.then_ignore(just(Token::Eq))
				.then(expr.clone());

			choice((
				var.map(|((mutable, name), init)| Stmt::Var {
					mutable,
					name,
					init,
				}),
				expr.clone().map(|expr| Stmt::Expr(expr)),
			))
			.then_ignore(just(Token::Semi))
		};

		let literal = select! {
			Token::Int(val) => Expr::Int(val),
			Token::Float(r_val, l_val) => Expr::Float(r_val, l_val),
			Token::Char(val) => Expr::Char(val),
			Token::String(val) => Expr::String(val),
		}
		.map_with_span(|expr, span: Span| (expr, span));

		let group = expr
			.clone()
			.delimited_by(just(Token::OpenParen), just(Token::CloseParen));

		// atom => "(" expr ")" | literal | IDENT
		let atom = choice((
			group,
			literal,
			ident.map(|(ident, span)| (Expr::Ident(ident), span)),
		)) // Attempt to recover anything that looks like a group but contains errors
		.recover_with(nested_delimiters(
			Token::OpenParen,
			Token::CloseParen,
			[
				(Token::OpenBrace, Token::CloseBrace),
				(Token::OpenBracket, Token::CloseBracket),
			],
			|span| (Expr::Error, span),
		));

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
		let op = just(Token::Bang)
			.to(UnaryOp::Not)
			.or(just(Token::Minus).to(UnaryOp::Neg))
			.map_with_span(|op, span: Span| (op, span));
		let unary = op.repeated().then(call).foldr(|(op, op_span), rhs| {
			let span = op_span.start..rhs.1.end;
			(Expr::Unary(op, Box::new(rhs)), span)
		});

		// Product operators (multiply and divide) have equal precedence
		let op = just(Token::Star)
			.to(BinaryOp::Mul)
			.or(just(Token::Slash).to(BinaryOp::Div));
		let product = unary
			.clone()
			.then(op.then(unary).repeated())
			.foldl(|lhs, (op, rhs)| {
				let span = lhs.1.start..rhs.1.end;
				(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
			});

		// Sum operators (add and subtract) have equal precedence
		let op = just(Token::Plus)
			.to(BinaryOp::Add)
			.or(just(Token::Minus).to(BinaryOp::Sub));
		let sum = product
			.clone()
			.then(op.then(product).repeated())
			.foldl(|lhs, (op, rhs)| {
				let span = lhs.1.start..rhs.1.end;
				(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
			});

		// Comparison operators (equal and not-equal) have equal precedence
		let op = just(Token::EqEq)
			.to(BinaryOp::Eq)
			.or(just(Token::BangEq).to(BinaryOp::Neq));
		let comp = sum
			.clone()
			.then(op.then(sum).repeated())
			.foldl(|lhs, (op, rhs)| {
				let span = lhs.1.start..rhs.1.end;
				(Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
			});

		let raw_expr = comp;

		// block => "{" stmt* "}"
		let block = stmt
			.repeated()
			.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace))
			// Attempt to recover anything that looks like a block but contains errors
			.recover_with(nested_delimiters(
				Token::OpenBrace,
				Token::CloseBrace,
				[
					(Token::OpenBracket, Token::CloseBracket),
					(Token::OpenParen, Token::CloseParen),
				],
				|_| vec![Stmt::Error],
			))
			.map_with_span(|stmts, span| (Expr::Block(stmts), span));

		// if => "if" expr "then" expr "else" expr
		let if_ = recursive(|if_| {
			just(Token::If)
				.ignore_then(expr.clone())
				.then_ignore(just(Token::Then))
				.then(expr.clone())
				.then(just(Token::Else).ignore_then(expr.clone()).or(if_))
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

		// record => (IDENT | ".") "{" record_fields? "}"
		// record_fields => record_field ("," record_field)* ","?
		// record_field => IDENT ":" expr
		let record_field = ident.then_ignore(just(Token::Colon)).then(expr);
		let record_fields = record_field
			.separated_by(just(Token::Comma))
			.allow_trailing();
		let record = ident
			.map(|ident| Some(ident))
			.or(just(Token::Dot).map(|_| None))
			.then(record_fields.delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
			.map_with_span(|(name, fields), span| (Expr::Record { name, fields }, span));

		choice((record, raw_expr, block, if_))
	})
}
