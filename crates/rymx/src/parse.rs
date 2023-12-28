use crate::{
    ast::{Module, Stmt},
    span::{SourceSpan, Span},
    tokenize::Token,
    SourceId,
};
use ariadne::{Label, Report};
use chumsky::{input::Input, prelude::*};
use itertools::Itertools;

mod common;
mod error;
mod file;
mod stmt;
mod r#type;

pub(self) use file::file_parser;
pub(self) use r#type::type_parser;
pub(self) use stmt::stmt_parser;

use self::error::{ParseError, Pattern};

pub fn parse_file<'a>(
    tokens: &'a [(Token, Span)],
    src: &'a str,
    src_id: SourceId,
) -> Result<Module, Vec<Report<'a, SourceSpan>>> {
    let parse_result = file_parser(src).parse(tokens.spanned(Span {
        start: src.len(),
        end: src.len(),
    }));

    map_parse_result(parse_result, src_id)
}

pub fn parse_stmt<'a>(
    tokens: &'a [(Token, Span)],
    src: &'a str,
    src_id: SourceId,
) -> Result<Stmt, Vec<Report<'a, SourceSpan>>> {
    let parse_result = stmt_parser(src).parse(tokens.spanned(Span {
        start: src.len(),
        end: src.len(),
    }));

    map_parse_result(parse_result, src_id)
}

fn map_parse_result<'a, T>(
    parse_result: ParseResult<T, ParseError>,
    src_id: SourceId,
) -> Result<T, Vec<Report<'a, SourceSpan>>> {
    use self::error::Reason;

    let err_to_report = |err: ParseError| {
        let report_base =
            Report::build(ariadne::ReportKind::Error, src_id.clone(), err.span().start);

        match err.reason() {
            Reason::ExpectedFound { expected, found } => report_base
                .with_message(format!("Syntax Error"))
                .with_label(
                    Label::new(SourceSpan(src_id.clone(), err.span())).with_message(format!(
                        "Expected {}, found {}.",
                        if expected.is_empty() {
                            "nothing".into()
                        } else {
                            expected.into_iter().map(display_pattern).join(" | ")
                        },
                        found
                            .map(|token| token.display())
                            .unwrap_or("nothing".into())
                    )),
                )
                .finish(),
            Reason::Custom(msg) => {
                let builder = report_base.with_message("Syntax Error").with_label(
                    Label::new(SourceSpan(src_id.clone(), err.span())).with_message(msg),
                );

                // let notes = HashMap::from([]);

                // if let Some(note) = notes.get(msg.as_str()) {
                // 	builder.with_note(note).finish()
                // } else {
                builder.finish()
                // }
            }
            Reason::Many(_) => todo!(),
        }
    };

    parse_result
        .into_result()
        .map_err(|errs| errs.into_iter().map(err_to_report).collect())
}

fn display_pattern(pattern: &Pattern) -> String {
    match pattern {
        Pattern::Token(token) => token.display(),
        Pattern::Label(label) => (*label).into(),
        Pattern::EndOfInput => "end of input".into(),
    }
}
