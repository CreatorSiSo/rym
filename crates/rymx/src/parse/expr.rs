use super::{common::*, type_parser};
use crate::{ast::*, interpret::Function, tokenize::Token};
use chumsky::prelude::*;

pub fn expr_parser(src: &str) -> impl Parser<TokenStream, Expr, Extra> + Clone {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(Expr::Literal);

		// var ::= ("const" | "let" | "let mut") ident (":" type)? "=" expr ";"
		let var = choice((
			just(Token::Const).to(VariableKind::Const),
			just(Token::Let)
				.then(just(Token::Mut))
				.to(VariableKind::LetMut),
			just(Token::Let).to(VariableKind::Let),
		))
		.then(ident_parser(src))
		.then_ignore(just(Token::Colon).ignore_then(type_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr.clone())
		.then_ignore(just(Token::Semi))
		.map(|((kind, name), rhs)| Stmt::Variable(kind, name.into(), rhs))
		.labelled("variable")
		.boxed();

		// function ::= fn "(" parameters ")" type? "=>" expr
		let function = just(Token::Fn)
			.ignore_then(
				parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			)
			.then(type_parser(src).or_not())
			.then(just(Token::ThickArrow).ignore_then(expr.clone()))
			.map(|((params, return_type), body)| {
				Expr::Function(Function {
					name: None,
					params: params.into_iter().map(Into::into).collect(),
					return_type: return_type.unwrap_or(Type::Unkown),
					body: Box::new(body),
				})
			})
			.labelled("function");

		// array ::= "[" (expr ";" expr | (expr ",")* expr?) "]"
		let array = choice((
			expr
				.clone()
				.then_ignore(just(Token::Semi))
				.then(expr.clone())
				.map(|(value, length)| Expr::ArrayWithRepeat(value.into(), length.into())),
			expr
				.clone()
				.separated_by(just(Token::Comma))
				.allow_trailing()
				.collect::<Vec<Expr>>()
				.map(Expr::Array),
		))
		.delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
		.labelled("array")
		.boxed();

		// atom ::= literal | ident | "(" expr ")" | array | block
		let atom = choice((
			literal,
			ident_parser(src).map(String::from).map(Expr::Ident),
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			array,
			block_parser(expr.clone(), var),
		))
		.labelled("atom");

		// call ::= atom ("(" (expr ("," expr)*)? ")")?
		let call = atom
			.then(
				expr
					.clone()
					.separated_by(just(Token::Comma))
					.collect::<Vec<Expr>>()
					.delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
					.or_not(),
			)
			.map(|(lhs, args)| match args {
				Some(args) => Expr::Call(Box::new(lhs), args),
				None => lhs,
			})
			.boxed();

		// chain ::= call ("." call)*
		let chain = call
			.clone()
			.foldl(just(Token::Dot).ignore_then(call).repeated(), |lhs, rhs| {
				Expr::Chain(Box::new(lhs), Box::new(rhs))
			});

		// unary ::= ("-" | "not")* chain
		let unary = select! {
			Token::Minus => UnaryOp::Neg,
			Token::Not => UnaryOp::Not,
		}
		.repeated()
		.at_least(1)
		.collect::<Vec<UnaryOp>>()
		.or_not()
		.then(chain)
		.map(|(maybe_op, expr)| {
			let Some(op) = maybe_op else {
				return expr;
			};
			op.into_iter()
				.fold(expr, |accum, op| Expr::Unary(op, Box::new(accum)))
		});

		// sum ::= unary (("*" | "/") unary)*
		let product = unary
			.clone()
			.foldl_with(
				select! {
					Token::Star => BinaryOp::Mul,
					Token::Slash => BinaryOp::Div,
				}
				.then(unary)
				.repeated(),
				|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
			)
			.boxed();

		// sum ::= product (("+" | "-") product)*
		let sum = product.clone().foldl_with(
			select! {
				Token::Plus => BinaryOp::Add,
				Token::Minus => BinaryOp::Sub,
			}
			.then(product)
			.repeated(),
			|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
		);

		// compare ::= sum (("==" | "!=" | "<" | "<=" | ">" | ">=") sum)*
		let compare = sum
			.clone()
			.foldl_with(
				select! {
					Token::Eq => BinaryOp::Eq,
					Token::NotEq => BinaryOp::NotEq,
					Token::LessThan => BinaryOp::LessThan,
					Token::LessThanEq => BinaryOp::LessThanEq,
					Token::GreaterThan => BinaryOp::GreaterThan,
					Token::GreaterThanEq => BinaryOp::GreaterThanEq,
				}
				.then(sum)
				.repeated(),
				|a, (op, b), _| Expr::Binary(op, Box::new(a), Box::new(b)),
			)
			.boxed();

		let if_else = just(Token::If)
			.ignore_then(expr.clone())
			.then_ignore(just(Token::Then))
			.then(expr.clone())
			.then(just(Token::Else).ignore_then(expr.clone()).or_not())
			.map(|((cond, then_branch), else_branch)| {
				Expr::IfElse(
					Box::new(cond),
					Box::new(then_branch),
					Box::new(else_branch.unwrap_or(Expr::Unit)),
				)
			});

		// return ::= "return" expr?
		let r#return = just(Token::Return)
			.ignore_then(expr.or_not())
			.map(|inner| Expr::Return(Box::new(inner.unwrap_or(Expr::Unit))));

		// expr ::= function | if_else | compare | return
		choice((function, if_else, compare, r#return))
			.labelled("expression")
			.boxed()
	})
}
