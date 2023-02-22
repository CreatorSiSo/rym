use chumsky::{error::SimpleReason, prelude::Simple};
use core::cmp::Ordering;
use std::fmt::Display;

use crate::{Span, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
	Block,
	Break,
	Continue,
	Expression,
	Group,
	Identifier,
	Loop,
	Record,
	Return,
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
			Label::Block => "block",
			Label::Break => "break",
			Label::Continue => "continue",
			Label::Expression => "expression",
			Label::Group => "group",
			Label::Identifier => "identifier",
			Label::Loop => "loop",
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
			"group" => Label::Group,
			"identifier" => Label::Identifier,
			"loop" => Label::Loop,
			"record" => Label::Record,
			"return" => Label::Return,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
	pub label: Option<Label>,
	pub span: Span,
	pub reason: SimpleReason<Token, Span>,
	pub found: Token,
	pub expected: Vec<Token>,
}

impl PartialOrd for Error {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Error {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.span == other.span {
			Ordering::Equal
		} else if self.span.start < other.span.start {
			Ordering::Less
		} else {
			Ordering::Greater
		}
	}
}

impl From<Simple<Token>> for Error {
	fn from(value: Simple<Token>) -> Self {
		let mut expected: Vec<Token> = value
			.expected()
			.cloned()
			.map(|o| o.unwrap_or(Token::Eof))
			.collect();
		expected.sort();

		Self {
			label: value.label().map(Into::into),
			span: value.span(),
			reason: value.reason().clone(),
			found: value.found().cloned().unwrap_or(Token::Eof),
			expected,
		}
	}
}
