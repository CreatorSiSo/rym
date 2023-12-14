use super::{common::*, expr::expr_parser};
use crate::ast::{Expr, Module};
use crate::tokenize::Token;
use chumsky::prelude::*;

pub fn file_parser(src: &str) -> impl Parser<TokenStream, Module, Extra> {
	let function_stmt = recursive(|function_stmt| {
		// function_stmt ::= "fn" ident "(" parameters ")" type? block
		just(Token::Fn)
			.ignore_then(ident_parser(src))
			.then(parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
			.then(ident_parser(src).or_not())
			.then(block_parser(expr_parser(src), function_stmt))
			.map(|(((name, params), return_type), body)| {
				Expr::Function(make_function(
					FnKind::Stmt,
					Some(name),
					params,
					return_type,
					body,
				))
			})
	})
	.map(|expr| match expr {
		Expr::Function(ref func) => (func.name.clone().unwrap(), expr),
		_ => unreachable!(),
	});

	// constant ::= "const" ident (":" type) "=" expr ";"
	let constant = just(Token::Const)
		.ignore_then(ident_parser(src))
		.then(just(Token::Colon).ignore_then(ident_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr_parser(src))
		.then_ignore(just(Token::Semi))
		.map(|((name, _typ), expr)| (name.to_string(), expr));

	// global ::= constnt | function_stmt
	let global = constant.or(function_stmt);

	// module ::= (global)*
	global.repeated().collect().map(|constants| Module {
		// TODO
		name: "".into(),
		constants,
		sub_modules: vec![],
	})
}
