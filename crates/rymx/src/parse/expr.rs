use super::common::*;
use crate::{ast::*, tokenize::Token, Span};
use chumsky::prelude::*;

pub fn expr_parser(src: &str) -> impl Parser<TokenStream, Expr, Extra> + Clone {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(Expr::Literal);

		// function_stmt ::= fn "ident" "(" parameters ")" ident? block
		let function_stmt = recursive(|function_stmt| {
			just(Token::Fn)
				.ignore_then(ident_parser(src))
				.then(parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
				.then(ident_parser(src).or_not())
				.then(block_parser(expr.clone(), function_stmt).map(|expr| expr))
				.map(|(((name, params), return_type), body)| {
					Expr::Function(make_function(
						FnKind::Stmt,
						Some(name),
						params,
						return_type,
						body,
					))
				})
				.labelled("function statement")
		});

		// function_expr ::= fn "(" parameters ")" ident? "=>" expr
		let function_expr = just(Token::Fn)
			.ignore_then(
				parameters_parser(src).delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			)
			.then(ident_parser(src).or_not())
			.then(just(Token::ThickArrow).ignore_then(expr.clone()))
			.map(|((params, return_type), body)| {
				Expr::Function(make_function(FnKind::Expr, None, params, return_type, body))
			})
			.labelled("function expression");

		// atom ::= literal | ident | "(" expr ")" | block
		let atom = choice((
			literal,
			ident_parser(src).map(String::from).map(Expr::Ident),
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			block_parser(expr.clone(), function_stmt),
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

		// var ::= ("const" | "let" | "let mut") indent (":" expr)? "=" expr
		let var = choice((
			just(Token::Const).to(VariableKind::Const),
			just(Token::Let)
				.then(just(Token::Mut))
				.to(VariableKind::LetMut),
			just(Token::Let).to(VariableKind::Let),
		))
		.then(ident_parser(src))
		.then_ignore(just(Token::Colon).ignore_then(expr.clone()).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr.clone())
		.map(
			|((kind, name), rhs)| Expr::Var(kind, name.into(), Box::new(rhs)),
			// TODO typ: Type::Unknown,
		)
		.boxed()
		.labelled("variable");

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

		// expr ::= function | var | if_else | compare | return
		choice((function_expr, var, if_else, compare, r#return))
			.labelled("expression")
			.boxed()
	})
}

fn literal_parser(src: &str) -> impl Parser<TokenStream, Literal, Extra> + Clone {
	let integer = just(Token::Int)
		.map_with(|_, extra| {
			Literal::Int(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse u64"),
			)
		})
		.labelled("integer");

	let float = just(Token::Float)
		.map_with(|_, extra| {
			Literal::Float(
				current_src(extra, src)
					.parse()
					.expect("Internal Error: Failed to parse f64"),
			)
		})
		.labelled("float");

	let string = just(Token::String)
		.map_with(|_, extra| {
			Literal::String({
				let mut span: Span = extra.span();
				span.start += 1;
				span.end -= 1;
				span.src(src).into()
			})
		})
		.labelled("string");

	choice((integer, float, string)).labelled("literal").boxed()
}
