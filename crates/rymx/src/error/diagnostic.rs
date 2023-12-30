use std::sync::mpsc::SyncSender;

use crate::Span;

/// An enum representing a diagnostic level.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Level {
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
    /// Some debug information.
    Debug,
}

/// A structure representing a diagnostic message and associated children messages.
#[must_use]
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            spans: vec![],
            children: vec![],
        }
    }

    pub fn spanned(spans: impl MultiSpan, level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            spans: spans.into_vec(),
            children: vec![],
        }
    }

    pub fn with_child(
        mut self,
        spans: impl MultiSpan,
        level: Level,
        message: impl Into<String>,
    ) -> Self {
        self.children.push(Self::spanned(spans, level, message));
        self
    }

    pub fn emit(self, emitter: SyncSender<Diagnostic>) {
        emitter
            .send(self)
            .expect("Internal error: Could not emit diagnostic")
    }
}

/// Trait implemented by types that can be converted into a set of `Span`s.
pub trait MultiSpan {
    /// Converts `self` into a `Vec<Span>`.
    fn into_vec(self) -> Vec<Span>;
}

impl MultiSpan for Span {
    fn into_vec(self) -> Vec<Span> {
        vec![self]
    }
}

impl MultiSpan for Vec<Span> {
    fn into_vec(self) -> Vec<Span> {
        self
    }
}

impl MultiSpan for &[Span] {
    fn into_vec(self) -> Vec<Span> {
        self.to_vec()
    }
}
