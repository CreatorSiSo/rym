#![feature(let_chains)]

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
    let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src, src_id);

    let (module, diagnostics) = parse::parse_file(&tokens, src, src_id);
    for diagnostic in diagnostics {
        emitter.send(diagnostic).unwrap();
    }
    Diagnostic::new(Level::Debug, "Finished parsing")
        .with_child(vec![], Level::Debug, format!("{module:#?}\n"))
        .emit(emitter);

    // TODO Name resolution
    // TODO Typechecking
    // TODO Const evaluation
    // TODO Generate intermediate representation

    module
}

// TODO take a module (for name lookup and so on) as input
pub fn compile_stmt(emitter: Sender<Diagnostic>, src: &str, src_id: SourceId) -> Option<ast::Stmt> {
    let tokens: Vec<(Token, Span)> = tokenize(emitter.clone(), src, src_id);

    let (expr, diagnostics) = parse::parse_stmt(&tokens, src, src_id);
    for diagnostic in diagnostics {
        emitter.send(diagnostic).unwrap();
    }
    Diagnostic::new(Level::Debug, "Finished parsing")
        .with_child(vec![], Level::Debug, format!("{expr:#?}\n"))
        .emit(emitter);

    expr
}

fn tokenize(emitter: Sender<Diagnostic>, src: &str, src_id: SourceId) -> Vec<(Token, Span)> {
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
                write!(accum, " [{}]\n", span.src(src).escape_debug()).unwrap();
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
