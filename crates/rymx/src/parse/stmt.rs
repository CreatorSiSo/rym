use super::{common::*, type_parser};
use crate::{ast::*, interpret::Function, tokenize::Token};
use chumsky::prelude::*;

pub fn stmt_parser(src: &str) -> impl Parser<TokenStream, Stmt, Extra> + Clone {
	recursive(|stmt| {
		let expr = expr_parser(src, stmt);

		// typedef ::=  "type" ident "=" type ";"
		let typedef = just(Token::Type)
			.ignore_then(ident_parser(src))
			.then_ignore(just(Token::Assign))
			.then(type_parser(src))
			.then_ignore(just(Token::Semi))
			.map(|(name, rhs)| Stmt::TypeDef(name.into(), rhs))
			.labelled("type definition")
			.boxed();

		// variable ::= ("const" | "let" | "let mut") ident (":" type)? "=" expr ";"
		let variable = choice((
			just(Token::Const).to(VariableKind::Const),
			just(Token::Let)
				.then(just(Token::Mut))
				.to(VariableKind::LetMut),
			just(Token::Let).to(VariableKind::Let),
		))
		.then(ident_parser(src))
		.then(just(Token::Colon).ignore_then(type_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr.clone())
		.then_ignore(just(Token::Semi))
		.map(|(((kind, name), typ), rhs)| {
			Stmt::Variable(kind, name.into(), typ.unwrap_or(Type::Unkown), rhs)
		})
		.labelled("variable")
		.boxed();

		choice((
			typedef,
			variable,
			expr.then_ignore(just(Token::Semi)).map(Stmt::Expr),
		))
	})
}

/// Only works when called from stmt_parser!
fn expr_parser<'src>(
	src: &'src str,
	stmt: impl Parser<'src, TokenStream<'src>, Stmt, Extra<'src>> + Clone + 'src,
) -> impl Parser<TokenStream, Expr, Extra> + Clone {
	recursive(|expr| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(Expr::Literal);

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

		// block ::= "{" statement* expr? "}"
		let block = stmt
			.clone()
			.repeated()
			.collect::<Vec<Stmt>>()
			.then(
				expr.clone().or_not(), // FIXME Not working
			)
			.delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
			.map(|(mut statements, final_expr)| {
				if let Some(expr) = final_expr {
					if !matches!(expr, Expr::Return(..) | Expr::Break(..)) {
						statements.push(Stmt::Expr(Expr::Break(Box::new(expr))));
					}
				}
				Expr::Block(statements)
			})
			.boxed();

		// atom ::= literal | ident | "(" expr ")" | array | block
		let atom = choice((
			literal,
			ident_parser(src).map(String::from).map(Expr::Ident),
			expr
				.clone()
				.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
			array,
			block,
		))
		.labelled("atom");

		let basic = {
			use chumsky::pratt::{infix, left, postfix, prefix};

			let binary = |associativity, token, op| {
				infix(associativity, just(token), move |l, r| {
					Expr::Binary(op, Box::new(l), Box::new(r))
				})
			};

			atom.clone().pratt((
				// field ::= atom "." ident
				postfix(
					6,
					just(Token::Dot).ignore_then(ident_parser(src).map(String::from)),
					|l, field| Expr::FieldAccess(Box::new(l), field),
				),
				// call ::= field "(" (expr ("," expr)*)? ")"
				postfix(
					5,
					expr
						.clone()
						.separated_by(just(Token::Comma))
						.collect::<Vec<Expr>>()
						.delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
					|l, args| Expr::Call(Box::new(l), args),
				),
				// unary ::= ("-" | "not") call
				prefix(4, just(Token::Not), |rhs| {
					Expr::Unary(UnaryOp::Not, Box::new(rhs))
				}),
				prefix(4, just(Token::Minus), |rhs| {
					Expr::Unary(UnaryOp::Neg, Box::new(rhs))
				}),
				// mul_div ::= unary ("*" | "/") unary
				binary(left(3), Token::Star, BinaryOp::Mul),
				binary(left(3), Token::Slash, BinaryOp::Div),
				// add_sub ::= mul_div ("*" | "/") mul_div
				binary(left(2), Token::Plus, BinaryOp::Add),
				binary(left(2), Token::Minus, BinaryOp::Sub),
				// compare ::= add_sub ("==" | "!=" | "<" | "<=" | ">" | ">=") add_sub
				binary(left(1), Token::Eq, BinaryOp::Eq),
				binary(left(1), Token::NotEq, BinaryOp::NotEq),
				binary(left(1), Token::LessThan, BinaryOp::LessThan),
				binary(left(1), Token::LessThanEq, BinaryOp::LessThanEq),
				binary(left(1), Token::GreaterThan, BinaryOp::GreaterThan),
				binary(left(1), Token::GreaterThanEq, BinaryOp::GreaterThanEq),
			))
		};

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

		// expr ::= function | if_else | simple | return
		choice((function, if_else, /* compare, */ basic, atom, r#return))
			.labelled("expression")
			.boxed()
	})
}
