use super::{common::*, stmt::stmt_parser};
use crate::ast::*;
use chumsky::prelude::*;

pub fn file_parser(src: &str) -> impl Parser<TokenStream, Module, Extra> {
	// file ::= (definition)*
	stmt_parser(src)
		.repeated()
		.collect()
		.validate(|stmts: Vec<Stmt>, extra, emitter| {
			let mut constants = vec![];
			let mut types = vec![];

			for stmt in stmts {
				match stmt {
					Stmt::Expr(..) => emitter.emit(Rich::custom(extra.span(), "todo")),
					Stmt::Variable(VariableKind::Const, name, typ, rhs) => constants.push((name, typ, rhs)),
					Stmt::Variable(..) => emitter.emit(Rich::custom(extra.span(), "todo")),
					Stmt::TypeDef(name, rhs) => types.push((name, rhs)),
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
