mod error;
mod interpret;
mod parse;
mod span;
pub mod std_lib;
mod tokenize;
mod typecheck;

pub use error::AriadneEmitter;
pub use interpret::Env;
use std::sync::mpsc;
pub use tokenize::tokenizer;

use error::{Diagnostic, Level, SourceId};
use interpret::{Interpret, Value};
use span::Span;
use tokenize::Token;

use crate::typecheck::typecheck_function;

pub fn interpret(env: &mut Env, ast: impl Interpret) -> Option<Value> {
    // let env_state: String = env
    //     .variables()
    //     .into_iter()
    //     .fold((0, String::new()), |(ident, mut accum), scope| {
    //         for (name, (kind, value)) in scope {
    //             writeln!(accum, "{}{kind} {name} = {}", "  ".repeat(ident), value)
    //                 .expect("Internal Error: Unable to write into String");
    //         }
    //         (ident + 1, accum)
    //     })
    //     .1;
    // emitter.push_result(&env_state);
    // emitter.send(t);

    // TODO does this make sense
    // Ignoring control flow
    // match ast.eval(env) {
    //     ControlFlow::Exit => None,
    //     ControlFlow::None(inner) => Some(inner),
    //     ControlFlow::Break(inner) => Some(inner),
    //     ControlFlow::Return(inner) => Some(inner),
    // }
    None
}

pub fn compile_module(
    emitter: mpsc::Sender<Diagnostic>,
    src: &str,
    src_id: SourceId,
) -> Option<ast::Module> {
    let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src, src_id);

    let module = parse::parse_file(&tokens, src, src_id);

    // for diagnostic in diagnostics {
    //     emitter.send(diagnostic).unwrap();
    // }
    // Diagnostic::new(Level::Debug, "Finished parsing")
    //     .with_child(vec![], Level::Debug, format!("{module:#?}\n"))
    //     .emit(emitter);
    println!("{:?}\n", &module);

    let mut ctx = typecheck::Context {
        symbol_table: typecheck::SymbolTable::new(),
    };
    for func in &module {
        ctx.symbol_table.insert(
            func.name.clone().unwrap(),
            ast::Type::Function {
                params: func
                    .params
                    .iter()
                    .map(|param| (param.name.clone(), param.typ.clone()))
                    .collect(),
                result: Box::new(func.return_type.clone()),
            },
        );
    }

    for func in module {
        typecheck_function(&mut ctx, &func);
    }

    // TODO Name resolution
    // TODO Typechecking
    // TODO Const evaluation
    // TODO Generate intermediate representation

    None
}

// pub fn compile_stmt(
//     emitter: mpsc::Sender<Diagnostic>,
//     src: &str,
//     src_id: SourceId,
// ) -> Option<ast::Stmt<'_>> {
//     let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src, src_id);

//     let (expr, diagnostics) = parse::parse_stmt(&tokens, src, src_id);
//     for diagnostic in diagnostics {
//         emitter.send(diagnostic).unwrap();
//     }
//     Diagnostic::new(Level::Debug, "Finished parsing")
//         .with_child(vec![], Level::Debug, format!("{expr:#?}\n"))
//         .emit(emitter);

//     expr
// }

fn tokenize(emitter: mpsc::Sender<Diagnostic>, src: &str, src_id: SourceId) -> Vec<(Token, Span)> {
    let results: Vec<(Option<Token>, Span)> = tokenizer(src)
        .map(|(token, span)| (token, span.with_id(src_id)))
        .collect();

    // Debugging stuff
    {
        use std::fmt::Write;
        let tokens_string = results
            .iter()
            .fold(String::new(), |mut accum, (result, span)| {
                match result {
                    Some(token) => write!(accum, "{token:?}"),
                    None => write!(accum, "Error"),
                }
                .unwrap();
                // TODO Add option to display spans as well
                writeln!(accum, " [{}]", span.src(src).escape_debug()).unwrap();
                accum
            });
        Diagnostic::new(Level::Debug, "Finished tokenizing")
            .with_child(vec![], Level::Debug, tokens_string)
            .emit(emitter.clone());
    }

    // Report and ignore invalid characters
    results
        .into_iter()
        .flat_map(|(maybe_token, span)| match maybe_token {
            Some(Token::DocComment | Token::Comment | Token::VSpace | Token::HSpace) => None,
            Some(token) => Some((token, span)),
            None => {
                let message = format!("Invalid character `{}`", span.src(src));
                Diagnostic::spanned(span, Level::Error, message).emit(emitter.clone());
                None
            }
        })
        .collect()
}
