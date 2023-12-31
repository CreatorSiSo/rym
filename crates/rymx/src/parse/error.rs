use super::common::TokenStream;
use crate::{tokenize::Token, Span};
use chumsky::util::MaybeRef;
use core::fmt;

type Label = &'static str;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pattern {
    /// A specific token was expected.
    Token(Token),
    /// A labelled pattern was expected.
    Label(Label),
    /// The end of input was expected.
    EndOfInput,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Token(token) => write!(f, "{}", token.display()),
            Self::Label(label) => f.write_str(label),
            Self::EndOfInput => f.write_str("EndOfInput"),
        }
    }
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Token(token) => write!(f, "{token:?}"),
            Self::Label(label) => f.write_str(label),
            Self::EndOfInput => f.write_str("EndOfInput"),
        }
    }
}

// TODO: Maybe should make ExpectedFound encapsulated a bit more
/// The reason for a [`Rich`] error.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reason {
    /// An unexpected input was found
    ExpectedFound {
        /// The tokens expected
        expected: Vec<Pattern>,
        /// The tokens found
        found: Option<Token>,
    },
    /// An error with a custom message
    Custom(String),
    /// Multiple unrelated reasons were merged
    // TODO: Should we really do this? Possibly better to just unify the unrelated reasons. It's not like consumers
    // probably care about reporting 5 different errors for the same location anyway!
    Many(Vec<Self>),
}

impl Reason {
    /// Return the token that was found by this error reason. `None` implies that the end of input was expected.
    pub fn found(&self) -> Option<Token> {
        match self {
            Self::ExpectedFound { found, .. } => *found,
            Self::Custom(_) => None,
            Self::Many(many) => many.iter().find_map(|r| r.found()),
        }
    }

    fn take_found(&mut self) -> Option<Token> {
        match self {
            Reason::ExpectedFound { found, .. } => found.take(),
            Reason::Custom(_) => None,
            Reason::Many(many) => many.iter_mut().find_map(|r| r.take_found()),
        }
    }
}

impl Reason {
    #[inline]
    fn flat_merge(self, other: Self) -> Self {
        match (self, other) {
            (
                Reason::ExpectedFound {
                    expected: mut this_expected,
                    found,
                },
                Reason::ExpectedFound {
                    expected: mut other_expected,
                    ..
                },
            ) => {
                // Try to avoid allocations if we possibly can by using the longer vector
                if other_expected.len() > this_expected.len() {
                    core::mem::swap(&mut this_expected, &mut other_expected);
                }
                for expected in other_expected {
                    if !this_expected[..].contains(&expected) {
                        this_expected.push(expected);
                    }
                }
                Reason::ExpectedFound {
                    expected: this_expected,
                    found,
                }
            }
            (Reason::Many(mut m1), Reason::Many(m2)) => {
                m1.extend(m2);
                Reason::Many(m1)
            }
            (Reason::Many(mut m), other) => {
                m.push(other);
                Reason::Many(m)
            }
            (this, Reason::Many(mut m)) => {
                m.push(this);
                Reason::Many(m)
            }
            (this, other) => Reason::Many(vec![this, other]),
        }
    }
}

/// A rich default error type that tracks error spans, expected inputs, and the actual input found at an error site.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParseError {
    span: Span,
    reason: Reason,
    context: Vec<(Label, Span)>,
}

impl ParseError {
    /// Create an error with a custom message and span
    #[inline]
    pub fn custom<M: ToString>(span: Span, msg: M) -> Self {
        ParseError {
            span,
            reason: Reason::Custom(msg.to_string()),
            context: Vec::new(),
        }
    }

    /// Get the span associated with this error.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Get the reason for this error.
    pub fn reason(&self) -> &Reason {
        &self.reason
    }

    /// Get the token found by this error when parsing. `None` implies that the error expected the end of input.
    pub fn found(&self) -> Option<Token> {
        self.reason.found()
    }

    /// Return an iterator over the labelled contexts of this error, from least general to most.
    ///
    /// 'Context' here means parser patterns that the parser was in the process of parsing when the error occurred. To
    /// add labelled contexts, see [`Parser::labelled`].
    pub fn contexts(&self) -> impl Iterator<Item = (&Label, &Span)> {
        self.context.iter().map(|(l, s)| (l, s))
    }

    /// Get an iterator over the expected items associated with this error
    pub fn expected(&self) -> impl ExactSizeIterator<Item = &Pattern> {
        fn push_expected<'a>(reason: &'a Reason, v: &mut Vec<&'a Pattern>) {
            match reason {
                Reason::ExpectedFound { expected, .. } => v.extend(expected.iter()),
                Reason::Custom(_) => {}
                Reason::Many(many) => many.iter().for_each(|r| push_expected(r, v)),
            }
        }
        let mut v = Vec::new();
        push_expected(&self.reason, &mut v);
        v.into_iter()
    }
}

impl<'a> chumsky::error::Error<'a, TokenStream<'a>> for ParseError {
    #[inline]
    fn expected_found<E: IntoIterator<Item = Option<MaybeRef<'a, Token>>>>(
        expected: E,
        found: Option<MaybeRef<'a, Token>>,
        span: Span,
    ) -> Self {
        Self {
            span,
            reason: Reason::ExpectedFound {
                expected: expected
                    .into_iter()
                    .map(|tok| {
                        tok.map(|inner| Pattern::Token(*inner))
                            .unwrap_or(Pattern::EndOfInput)
                    })
                    .collect(),
                found: found.map(|inner| *inner),
            },
            context: Vec::new(),
        }
    }

    #[inline]
    fn merge(mut self, mut other: Self) -> Self {
        let new_reason = self.reason.flat_merge(other.reason);
        Self {
            span: self.span,
            reason: new_reason,
            // TODO Merging contexts correctly?
            context: {
                self.context.append(&mut other.context);
                self.context.dedup_by_key(|(label, _)| *label);
                self.context
            },
        }
    }

    #[inline]
    fn merge_expected_found<E: IntoIterator<Item = Option<MaybeRef<'a, Token>>>>(
        mut self,
        new_expected: E,
        found: Option<MaybeRef<'a, Token>>,
        _span: Span,
    ) -> Self {
        match &mut self.reason {
            Reason::ExpectedFound { expected, found: _ } => {
                for new_expected in new_expected {
                    let new_expected = new_expected
                        .map(|inner| Pattern::Token(*inner))
                        .unwrap_or(Pattern::EndOfInput);
                    if !expected[..].contains(&new_expected) {
                        expected.push(new_expected);
                    }
                }
            }
            Reason::Many(m) => m.push(Reason::ExpectedFound {
                expected: new_expected
                    .into_iter()
                    .map(|tok| {
                        tok.map(|inner| Pattern::Token(*inner))
                            .unwrap_or(Pattern::EndOfInput)
                    })
                    .collect(),
                found: found.map(|inner| *inner),
            }),
            Reason::Custom(_) => {
                let old = core::mem::replace(&mut self.reason, Reason::Many(Vec::new()));
                self.reason = Reason::Many(vec![
                    old,
                    Reason::ExpectedFound {
                        expected: new_expected
                            .into_iter()
                            .map(|tok| {
                                tok.map(|inner| Pattern::Token(*inner))
                                    .unwrap_or(Pattern::EndOfInput)
                            })
                            .collect(),
                        found: found.map(|inner| *inner),
                    },
                ]);
            }
        }
        // TOOD: Merge contexts
        self
    }

    #[inline]
    fn replace_expected_found<E: IntoIterator<Item = Option<MaybeRef<'a, Token>>>>(
        mut self,
        new_expected: E,
        new_found: Option<MaybeRef<'a, Token>>,
        span: Span,
    ) -> Self {
        self.span = span;
        match &mut self.reason {
            Reason::ExpectedFound { expected, found } => {
                expected.clear();
                expected.extend(new_expected.into_iter().map(|tok| {
                    tok.map(|inner| Pattern::Token(*inner))
                        .unwrap_or(Pattern::EndOfInput)
                }));
                *found = new_found.map(|inner| *inner);
            }
            _ => {
                self.reason = Reason::ExpectedFound {
                    expected: new_expected
                        .into_iter()
                        .map(|tok| {
                            tok.map(|inner| Pattern::Token(*inner))
                                .unwrap_or(Pattern::EndOfInput)
                        })
                        .collect(),
                    found: new_found.map(|inner| *inner),
                };
            }
        }
        self.context.clear();
        self
    }
}

impl<'a> chumsky::label::LabelError<'a, TokenStream<'a>, Label> for ParseError {
    #[inline]
    fn label_with(&mut self, label: Label) {
        // Opportunistically attempt to reuse allocations if we can
        match &mut self.reason {
            Reason::ExpectedFound { expected, found: _ } => {
                expected.clear();
                expected.push(Pattern::Label(label));
            }
            _ => {
                self.reason = Reason::ExpectedFound {
                    expected: vec![Pattern::Label(label)],
                    found: self.reason.take_found(),
                };
            }
        }
    }

    #[inline]
    fn in_context(&mut self, label: Label, span: Span) {
        if self.context.iter().all(|(l, _)| l != &label) {
            self.context.push((label, span));
        }
    }
}
