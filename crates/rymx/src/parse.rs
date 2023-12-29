use self::error::{ParseError, Pattern};
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

type ReportBuilder<'a> = ariadne::ReportBuilder<'a, SourceSpan>;

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
        let builder = Report::build(ariadne::ReportKind::Error, src_id.clone(), err.span().start);
        let span = SourceSpan(src_id.clone(), err.span());
        dbg!(err.contexts().collect::<Vec<_>>());

        match err.reason() {
            Reason::ExpectedFound { expected, found } => match (expected.is_empty(), found) {
                (true, _) => report_unexpected(builder, span, found),
                (false, None) => report_expected(builder, span, expected),
                (false, Some(found)) => report_expected_found(builder, span, expected, found),
            },
            Reason::Custom(msg) => builder
                .with_message("Syntax Error")
                .with_label(Label::new(SourceSpan(src_id.clone(), err.span())).with_message(msg)),
            Reason::Many(_) => todo!(),
        }
        .finish()
    };

    parse_result
        .into_result()
        .map_err(|errs| errs.into_iter().map(err_to_report).collect())
}

fn report_expected_found<'a>(
    builder: ReportBuilder<'a>,
    span: SourceSpan,
    expected: &Vec<Pattern>,
    found: &Token,
) -> ReportBuilder<'a> {
    let patterns = patterns_to_string(expected);
    builder
        .with_message(format!("Expected {patterns}, found {}", found.display()))
        .with_label(Label::new(span).with_message(format!("Expected {patterns}")))
}

fn report_expected<'a>(
    builder: ReportBuilder<'a>,
    span: SourceSpan,
    expected: &Vec<Pattern>,
) -> ReportBuilder<'a> {
    let msg = format!("Expected {}", patterns_to_string(expected));
    builder
        .with_message(msg.clone())
        .with_label(Label::new(span).with_message(msg))
}

fn report_unexpected<'a>(
    builder: ReportBuilder<'a>,
    span: SourceSpan,
    found: &Option<Token>,
) -> ReportBuilder<'a> {
    let msg = format!(
        "Unexpected {}",
        found
            .map(|token| token.display())
            .unwrap_or("end of input".into())
    );
    builder
        .with_message(msg.clone())
        .with_label(Label::new(span).with_message(msg))
}

fn patterns_to_string(patterns: &Vec<Pattern>) -> String {
    if patterns.is_empty() {
        return "nothing".into();
    };

    use std::collections::HashSet;
    let mut patterns: HashSet<&Pattern> = HashSet::from_iter(patterns.iter());

    fn replace_subset<'a>(
        super_set: &mut HashSet<&'a Pattern>,
        search: &'static [Pattern],
        replacement: &'static Pattern,
    ) {
        let search_set = HashSet::from_iter(search);
        if search_set.is_subset(&super_set) {
            for pattern in search_set {
                super_set.remove(pattern);
            }
            super_set.insert(&replacement);
        }
    }

    fn replace_element(
        haystack: &mut HashSet<&Pattern>,
        needle: &'static Pattern,
        replacement: &'static Pattern,
    ) {
        if haystack.remove(needle) {
            haystack.insert(replacement);
        }
    }

    replace_element(
        &mut patterns,
        &Pattern::Token(Token::Ident),
        &Pattern::Label("identifier"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Token(Token::Int),
            Pattern::Token(Token::Float),
            Pattern::Token(Token::String),
        ],
        &Pattern::Label("literal"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Token(Token::Eq),
            Pattern::Token(Token::NotEq),
            Pattern::Token(Token::LessThan),
            Pattern::Token(Token::LessThanEq),
            Pattern::Token(Token::GreaterThan),
            Pattern::Token(Token::GreaterThanEq),
        ],
        &Pattern::Label("a comparison operator"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Token(Token::Slash),
            Pattern::Token(Token::Star),
            Pattern::Token(Token::Plus),
            Pattern::Token(Token::Minus),
        ],
        &Pattern::Label("an arithmetic operator"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Token(Token::Assign),
            // TODO Add others
        ],
        &Pattern::Label("an assignment operator"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Label("a comparison operator"),
            Pattern::Label("an arithmetic operator"),
            Pattern::Label("an assignment operator"),
        ],
        &Pattern::Label("an operator"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Label("literal"),
            Pattern::Token(Token::Ident),
            Pattern::Token(Token::ParenOpen),
            Pattern::Token(Token::BraceOpen),
            Pattern::Token(Token::BracketOpen),
            Pattern::Token(Token::Break),
            Pattern::Token(Token::Return),
        ],
        &Pattern::Label("expression"),
    );

    replace_subset(
        &mut patterns,
        &[
            Pattern::Label("expression"),
            Pattern::Token(Token::Not),
            Pattern::Token(Token::Minus),
        ],
        &Pattern::Label("expression"),
    );

    let mut patterns = patterns.into_iter().collect_vec();
    patterns.sort();
    let (last, start) = patterns.split_last().unwrap();
    format!(
        "{}{}{last}",
        start.iter().join(", "),
        if start.is_empty() { "" } else { " or " }
    )
}
