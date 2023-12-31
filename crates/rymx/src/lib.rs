use std::sync::mpsc::Sender;

mod ast;
mod compile;
mod error;
mod interpret;
mod parse;
mod span;
pub mod std_lib;
mod tokenize;

pub use error::AriadneEmitter;
pub use interpret::Env;
pub use tokenize::tokenizer;

use error::{Diagnostic, Level, SourceId};
use interpret::{Interpret, Value};
use span::Span;
use tokenize::Token;

pub fn interpret(env: &mut Env, ast: impl Interpret) -> Value {
    // TODO does this make sense
    // Ignoring control flow
    let result = ast.eval(env).inner();

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

    result
}

pub fn compile_module(
    emitter: Sender<Diagnostic>,
    src: &str,
    src_id: SourceId,
) -> Option<ast::Module> {
    let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src)?
        .into_iter()
        .map(|(token, span)| (token, span.with_id(src_id)))
        .collect();

    let module = match parse::parse_file(&tokens, src, src_id) {
        Ok(module) => module,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                emitter.send(diagnostic).unwrap();
            }
            return None;
        }
    };
    emitter
        .send(Diagnostic::new(Level::Debug, format!("{module:#?}\n")))
        .unwrap();

    // TODO Name resolution
    // TODO Typechecking
    // TODO Const evaluation
    // TODO Generate intermediate representation

    Some(module)
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_stmt(emitter: Sender<Diagnostic>, src: &str, src_id: SourceId) -> Option<ast::Stmt> {
    let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src)?
        .into_iter()
        .map(|(token, span)| (token, span.with_id(src_id)))
        .collect();

    let expr = match parse::parse_stmt(&tokens, src, src_id) {
        Ok(expr) => expr,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                emitter.send(diagnostic).unwrap();
            }
            return None;
        }
    };
    emitter
        .send(Diagnostic::new(Level::Debug, format!("{expr:#?}\n")))
        .unwrap();

    Some(expr)
}

fn tokenize(emitter: Sender<Diagnostic>, src: &str) -> Option<Vec<(Token, Span)>> {
    let results: Vec<_> = tokenizer(src).collect();

    let tokens_string = results.iter().fold(String::new(), |accum, (result, span)| {
        let (token, span) = match result {
            Ok(token) => (format!("{token:?}"), span),
            Err(_) => ("Error".into(), span),
        };
        accum + &format!("{token} [{}]\n", span.src(src).escape_debug())
    });
    emitter
        .send(
            Diagnostic::new(Level::Debug, "Finished tokenizing").with_child(
                vec![],
                Level::Debug,
                tokens_string,
            ),
        )
        .unwrap();

    let mut tokens = vec![];
    for (result, span) in results {
        let Ok(token) = result else {
            emitter
                .send(Diagnostic::spanned(
                    span,
                    Level::Error,
                    format!("Invalid character [{}]", span.src(src)),
                ))
                .unwrap();
            return None;
        };
        match token {
            Token::DocComment | Token::Comment | Token::VSpace | Token::HSpace => continue,
            _ => tokens.push((token, span)),
        }
    }
    Some(tokens)
}
