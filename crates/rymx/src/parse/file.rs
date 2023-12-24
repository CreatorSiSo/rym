use super::{common::*, expr::expr_parser, r#type::type_parser};
use crate::ast::*;
use crate::parse::r#type::typedef_parser;
use crate::tokenize::Token;
use chumsky::prelude::*;

pub fn file_parser(src: &str) -> impl Parser<TokenStream, Module, Extra> {
	enum Definition {
		Constant((String, Option<Type>, Expr)),
		TypeDef((String, Type)),
	}

	// constant ::= "const" ident (":" type) "=" expr ";"
	let constant = just(Token::Const)
		.ignore_then(ident_parser(src))
		.then(just(Token::Colon).ignore_then(type_parser(src)).or_not())
		.then_ignore(just(Token::Assign))
		.then(expr_parser(src))
		.then_ignore(just(Token::Semi))
		.map(|((name, typ), expr)| Definition::Constant((name.into(), typ, expr)));

	let typedef = typedef_parser(src).map(|(name, value)| Definition::TypeDef((name.into(), value)));

	let definition = constant.or(typedef);

	// module ::= (definition)*
	definition
		.repeated()
		.collect()
		.map(|definitions: Vec<Definition>| {
			let mut constants = vec![];
			let mut types = vec![];

			for definition in definitions {
				match definition {
					Definition::Constant(inner) => constants.push(inner),
					Definition::TypeDef(inner) => types.push(inner),
				}
			}

			Module {
				// TODO
				name: "".into(),
				constants,
				types,
				sub_modules: vec![],
			}
		})
}
