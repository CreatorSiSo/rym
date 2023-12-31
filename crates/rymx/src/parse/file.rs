use super::{common::*, error::ParseError, stmt::stmt_parser};
use crate::ast::*;
use chumsky::prelude::*;

pub fn file_parser(src: &str) -> impl Parser<TokenStream, Module, Extra> {
    let definition = stmt_parser(src).validate(|stmt, extra, emitter| {
        match stmt {
            Stmt::Expr(..) => emitter.emit(ParseError::custom(
                current_span(extra),
                "Top-level expressions are not allowed.",
            )),
            Stmt::Variable(VariableKind::Let | VariableKind::LetMut, ..) => {
                emitter.emit(ParseError::custom(current_span(extra), "todo"))
            }
            _ => {}
        }
        stmt
    });

    // file ::= (definition)*
    definition
        .repeated()
        .collect()
        .map(|stmts: Vec<Stmt>| {
            let mut constants = vec![];
            let mut types = vec![];

            for stmt in stmts {
                match stmt {
                    Stmt::Variable(VariableKind::Const, name, typ, rhs) => {
                        constants.push((name, typ, rhs))
                    }
                    Stmt::Type(name, rhs) => types.push((name, rhs)),

                    // Already emitted an error for these
                    _ => {}
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
        .with_ctx(src)
}
