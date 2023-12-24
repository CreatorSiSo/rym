use crate::{
	ast::{Module, Stmt},
	span::{SourceSpan, Span},
	tokenize::Token,
	SourceId,
};
use ariadne::{Label, Report};
use chumsky::{error::RichPattern, input::Input, prelude::*};
use itertools::Itertools;

mod common;
mod file;
mod stmt;
mod r#type;

pub(self) use file::file_parser;
pub(self) use stmt::stmt_parser;
pub(self) use r#type::type_parser;

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

fn map_parse_result<T>(
	parse_result: ParseResult<T, Rich<'_, Token, Span>>,
	src_id: SourceId,
) -> Result<T, Vec<Report<'_, SourceSpan>>> {
	use chumsky::error::RichReason;

	let err_to_report = |err: Rich<'_, Token, Span>| {
		let report_base = Report::build(ariadne::ReportKind::Error, src_id.clone(), err.span().start);

		match err.reason() {
			RichReason::ExpectedFound { expected, found } => report_base
				.with_message(format!("Syntax Error"))
				.with_label(
					Label::new(SourceSpan(src_id.clone(), *err.span())).with_message(format!(
						"Expected {}, found {}.",
						if expected.is_empty() {
							"nothing".into()
						} else {
							expected.into_iter().map(display_pattern).join(" or ")
						},
						found
							.map(|token| token.display())
							.unwrap_or("nothing".into())
					)),
				)
				.finish(),
			RichReason::Custom(msg) => {
				let builder = report_base
					.with_message("Syntax Error")
					.with_label(Label::new(SourceSpan(src_id.clone(), *err.span())).with_message(msg));

				// let notes = HashMap::from([]);

				// if let Some(note) = notes.get(msg.as_str()) {
				// 	builder.with_note(note).finish()
				// } else {
				builder.finish()
				// }
			}
			RichReason::Many(_) => todo!(),
		}
	};

	parse_result
		.into_result()
		.map_err(|errs| errs.into_iter().map(err_to_report).collect())
}

fn display_pattern(pattern: &RichPattern<'_, Token>) -> String {
	match pattern {
		RichPattern::Token(token) => token.display(),
		RichPattern::Label(label) => (*label).into(),
		RichPattern::EndOfInput => "end of input".into(),
	}
}
