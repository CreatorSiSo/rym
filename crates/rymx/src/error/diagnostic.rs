use std::{fmt::Display, sync::mpsc::Sender};

use crate::Span;

/// An enum representing a diagnostic level.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Error => write!(f, "Error"),
            Level::Warning => write!(f, "Warning"),
            Level::Note => write!(f, "Note"),
            Level::Help => write!(f, "Help"),
            Level::Debug => write!(f, "Debug"),
        }
    }
}

/// A structure representing a diagnostic message and associated children messages.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub span: Option<Span>,
    pub children: Vec<SubDiagnostic>,
}

impl Diagnostic {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            span: None,
            children: vec![],
        }
    }

    pub fn spanned(span: Span, level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            span: Some(span),
            children: vec![],
        }
    }

    pub fn with_child(
        mut self,
        spans: impl MultiSpan,
        level: Level,
        message: impl Into<String>,
    ) -> Self {
        self.children.push(SubDiagnostic {
            level,
            message: message.into(),
            spans: spans.into_vec(),
        });
        self
    }

    pub fn emit(self, emitter: Sender<Diagnostic>) {
        emitter
            .send(self)
            .expect("Internal error: Could not emit diagnostic")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubDiagnostic {
    pub level: Level,
    pub message: String,
    pub spans: Vec<Span>,
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
