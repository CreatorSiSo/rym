use crate::{
	tokenize::{Token, TokenKind},
	Span,
};

struct Node {
	pub span: Span<u32>,
	pub kind: NodeKind,
}

enum NodeKind {
	BranchStart(BranchKind),
	BranchEnd(BranchKind),
	Leaf(Leaf),
}

enum BranchKind {
	Block,
	Call,
	Def,
	Fn,
	None,
	Params,
	Typed,
}

pub enum Leaf {
	Int(i64),
	Float(f64),
	String(String),
	Char(String),

	Ident,
	DocComment,
	Comment,
	VSpace,
	HSpace,

	As,
	Const,
	Enum,
	For,
	Impl,
	Let,
	Mut,
	Not,
	LowerSelf,
	UpperSelf,
	Struct,
	Use,

	BraceOpen,
	BraceClose,
	BracketOpen,
	BracketClose,
	ParenOpen,
	ParenClose,

	Ampersand,
	Assign,
	Comma,
	Dot,
	DotDot,
	Plus,
	Pipe,
	Minus,
	Star,
	Slash,
	Percent,
	Pound,
	Semi,
	Colon,

	Eq,
	NotEq,
	LessThan,
	LessThanEq,
	GreaterThan,
	GreaterThanEq,
}

impl Leaf {
	pub fn from_token(token: Token, src: &str) -> Self {
		match token.kind {
			TokenKind::Int => Self::Int(0),
			TokenKind::Float => Self::Float(0.0),
			TokenKind::String => Self::String("".into()),
			TokenKind::Char => Self::Char("".into()),
			TokenKind::Ident => Self::Ident,

			TokenKind::DocComment => Self::DocComment,
			TokenKind::Comment => Self::Comment,
			TokenKind::VSpace => Self::VSpace,
			TokenKind::HSpace => Self::HSpace,

			TokenKind::As => Self::As,
			TokenKind::Const => Self::Const,
			TokenKind::Enum => Self::Enum,
			TokenKind::For => Self::For,
			TokenKind::Impl => Self::Impl,
			TokenKind::Let => Self::Let,
			TokenKind::Mut => Self::Mut,
			TokenKind::Not => Self::Not,
			TokenKind::LowerSelf => Self::LowerSelf,
			TokenKind::UpperSelf => Self::UpperSelf,
			TokenKind::Struct => Self::Struct,
			TokenKind::Use => Self::Use,

			TokenKind::BraceOpen => Self::BraceOpen,
			TokenKind::BraceClose => Self::BraceClose,
			TokenKind::BracketOpen => Self::BracketOpen,
			TokenKind::BracketClose => Self::BracketClose,
			TokenKind::ParenOpen => Self::ParenOpen,
			TokenKind::ParenClose => Self::ParenClose,

			TokenKind::Ampersand => Self::Ampersand,
			TokenKind::Assign => Self::Assign,
			TokenKind::Comma => Self::Comma,
			TokenKind::Dot => Self::Dot,
			TokenKind::DotDot => Self::DotDot,
			TokenKind::Plus => Self::Plus,
			TokenKind::Pipe => Self::Pipe,
			TokenKind::Minus => Self::Minus,
			TokenKind::Star => Self::Star,
			TokenKind::Slash => Self::Slash,
			TokenKind::Percent => Self::Percent,
			TokenKind::Pound => Self::Pound,
			TokenKind::Semi => Self::Semi,
			TokenKind::Colon => Self::Colon,

			TokenKind::Eq => Self::Eq,
			TokenKind::NotEq => Self::NotEq,
			TokenKind::LessThan => Self::LessThan,
			TokenKind::LessThanEq => Self::LessThanEq,
			TokenKind::GreaterThan => Self::GreaterThan,
			TokenKind::GreaterThanEq => Self::GreaterThanEq,
		}
	}
}
