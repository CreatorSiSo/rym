use chumsky::ParseResult;
use chumsky::error::{Rich, RichReason};
use itertools::Itertools;
use span::Span;

use crate::error::{Diagnostic, Level};
use crate::tokenize::Token;

pub fn map_parse_result<T>(
    result: ParseResult<T, Rich<'_, Token, Span>>,
) -> Result<T, Vec<Diagnostic>> {
    if result.has_output() {
        return Ok(result.into_output().unwrap());
    }

    Err(result
        .errors()
        .map(|error| match error.reason() {
            RichReason::ExpectedFound { expected, found } => {
                let pattern = expected
                    .iter()
                    .map(|pattern| pattern.to_string())
                    .join(" | ");

                let message = if let Some(found) = found {
                    format!("expected {pattern}, found '{}'", found.to_string())
                } else {
                    format!("expected {pattern}")
                };

                Diagnostic::new(Level::Error, message).with_child(
                    [*error.span()].as_slice(),
                    Level::Error,
                    "unexpected token",
                )
            }
            RichReason::Custom(_) => todo!(),
        })
        .collect())
}
