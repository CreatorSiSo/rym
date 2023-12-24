use crate::{
	ast::{Module, Stmt},
	span::{SourceSpan, Span},
	tokenize::Token,
	SourceId,
};
use ariadne::{Label, Report};
use chumsky::{input::Input, prelude::*};

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
	parse_result.into_result().map_err(|errs| {
		errs
			.into_iter()
			.map(|err| {
				Report::build(ariadne::ReportKind::Error, src_id.clone(), err.span().start)
					.with_label(
						Label::new(SourceSpan(src_id.clone(), *err.span()))
							.with_message(format!("{:?}", err.reason())),
					)
					.finish()
			})
			.collect()
	})
}
