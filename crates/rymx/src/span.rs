use crate::error::SourceId;
use core::ops::Range;
use std::fmt::{Debug, Display};

type Index = usize;

#[derive(Debug)]
pub struct Spanned<T> {
    val: T,
    span: Span,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    /// Inclusive first index
    pub start: Index,
    /// Exclusive last index
    pub end: Index,
    /// Id to corresponding source code
    pub id: SourceId,
}

impl Span {
    pub fn new(start: Index, end: Index) -> Self {
        Self {
            start,
            end,
            id: SourceId::INVALID,
        }
    }

    pub fn src(self, src: &str) -> &str {
        &src[self.start..self.end]
    }

    pub fn with_id(mut self, id: SourceId) -> Self {
        self.id = id;
        self
    }
}

impl From<Span> for Range<Index> {
    fn from(value: Span) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<Range<Index>> for Span {
    fn from(value: Range<Index>) -> Self {
        Self {
            start: value.start,
            end: value.end,
            id: SourceId::INVALID,
        }
    }
}

// impl TryFrom<Range<usize>> for Span {
//     type Error = std::num::TryFromIntError;

//     fn try_from(value: Range<usize>) -> Result<Self, Self::Error> {
//         Ok(Self {
//             start: value.start.try_into()?,
//             end: value.end.try_into()?,
//             ..Self::default()
//         })
//     }
// }

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{} {}", self.start, self.end, self.id))
    }
}

impl chumsky::span::Span for Span {
    type Context = SourceId;
    type Offset = Index;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Span::from(range).with_id(context)
    }

    fn context(&self) -> Self::Context {
        self.id
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

impl ariadne::Span for Span {
    type SourceId = SourceId;

    fn source(&self) -> &Self::SourceId {
        &self.id
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}
