use crate::{Span, Token};
use chumsky::error::RichReason;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let data = <&'static str>::from(self);
		f.write_str(data)
	}
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

impl From<Label> for &'static str {
	fn from(value: Label) -> Self {
		(&value).into()
	}
}

impl From<&'static str> for Label {
	fn from(value: &'static str) -> Self {
		match value {
			"block" => Label::Block,
			"break" => Label::Break,
			"continue" => Label::Continue,
			"expression" => Label::Expression,
			"function" => Label::Function,
			"group" => Label::Group,
			"identifier" => Label::Identifier,
			"loop" => Label::Loop,
			"module" => Label::Module,
			"record" => Label::Record,
			"return" => Label::Return,
			"binding" => Label::Binding,
			_ => unreachable!(),
		}
	}
}

pub struct ParseResult<T>(pub Option<T>, pub Vec<ParseError>);

impl<T> From<chumsky::ParseResult<T, crate::Error<'_>>> for ParseResult<T> {
	fn from(value: chumsky::ParseResult<T, crate::Error<'_>>) -> Self {
		let (output, errors) = value.into_output_errors();
		let errors: Vec<ParseError> = errors.into_iter().map(|err| err.into()).collect();
		Self(output, errors)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
	pub label: Option<Label>,
	pub span: Span,
	pub reason: ErrorReason,
	pub found: Token,
	pub expected: Vec<Token>,
}

impl<'a> From<chumsky::prelude::Rich<'a, chumsky::input::Spanned<Token, Span, &'a [(Token, Span)]>>>
	for ParseError
{
	fn from(
		value: chumsky::prelude::Rich<
			'a,
			chumsky::input::Spanned<Token, Span, &'a [(Token, Span)]>,
		>,
	) -> Self {
		let mut expected: Vec<Token> = value
			.expected()
			.map(|o| o.unwrap_or(&Token::Eof))
			.cloned()
			.collect();
		expected.sort();

		Self {
			label: /* value.label().map(Into::into) */ None,
			span: value.span(),
			// TODO just use clone once it is implemented
			reason: <&RichReason<'_, chumsky::input::Spanned<Token, Span, &'a [(Token, Span)]>> as Into<ErrorReason>>::into(value.reason()),
			found: value.found().cloned().unwrap_or(Token::Eof),
			expected,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorReason {
	/// An unexpected input was found
	Unexpected,
	/// An error with a custom message
	Custom(String),
	/// Multiple unrelated reasons were merged
	Many(Vec<ErrorReason>),
}

impl<'a> From<&RichReason<'a, chumsky::input::Spanned<Token, Span, &'a [(Token, Span)]>>>
	for ErrorReason
{
	fn from(
		value: &RichReason<'a, chumsky::input::Spanned<Token, Span, &'a [(Token, Span)]>>,
	) -> Self {
		match value {
			RichReason::ExpectedFound { .. } => Self::Unexpected,
			RichReason::Custom(msg) => Self::Custom(msg.clone()),
			RichReason::Many(reasons) => Self::Many(reasons.into_iter().map(Into::into).collect()),
		}
	}
}
