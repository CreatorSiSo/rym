use crate::{ErrorAlias, Span, Token};
use chumsky::error::RichReason;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Label {
	Block,
	Break,
	Continue,
	Expression,
	Function,
	Group,
	Identifier,
	Loop,
	Module,
	Record,
	Return,
	Binding,
}

impl From<&Label> for &'static str {
	fn from(value: &Label) -> Self {
		match value {
			Label::Binding => "binding",
			Label::Block => "block",
			Label::Break => "break",
			Label::Continue => "continue",
			Label::Expression => "expression",
			Label::Function => "function",
			Label::Group => "group",
			Label::Identifier => "identifier",
			Label::Loop => "loop",
			Label::Module => "module",
			Label::Record => "record",
			Label::Return => "return",
		}
	}
}

impl Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let data = <&'static str>::from(self);
		f.write_str(data)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError<'a> {
	pub span: Span,
	pub reason: RichReason<'a, Token, Label>,
}

impl<'a> From<ErrorAlias<'a>> for ParseError<'a> {
	fn from(value: ErrorAlias<'a>) -> ParseError<'a> {
		fn deep_clone_sort_reason(reason: RichReason<Token, Label>) -> RichReason<Token, Label> {
			match reason {
				RichReason::ExpectedFound {
					mut expected,
					found,
				} => {
					expected.sort();
					RichReason::ExpectedFound { expected, found }
				}
				RichReason::Many(mut reasons) => {
					reasons.sort();
					RichReason::Many(reasons.into_iter().map(deep_clone_sort_reason).collect())
				}
				reason => reason,
			}
		}

		Self {
			span: value.span().clone(),
			reason: deep_clone_sort_reason(value.reason().clone()),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseResult<'a, T>(pub Option<T>, pub Vec<ParseError<'a>>);

impl<'a, T> From<chumsky::ParseResult<T, ErrorAlias<'a>>> for ParseResult<'a, T> {
	fn from(value: chumsky::ParseResult<T, ErrorAlias<'a>>) -> Self {
		let (output, errors) = value.into_output_errors();
		let errors: Vec<_> = errors.into_iter().map(|err| err.into()).collect();
		Self(output, errors)
	}
}
