use crate::error::diagnostic::SubDiagnostic;
use crate::span::Span;

use super::{Diagnostic, Level};
use ariadne::{Cache, Color, Label, ReportKind, Source};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io;
use std::sync::mpsc::{self, Receiver, Sender};

// pub trait Emitter {
//     fn emit_diagnostic(&mut self, diagnostic: &Diagnostic);
// }

pub struct AriadneEmitter<W: io::Write> {
    pub source_map: SourceMap,
    receiver: Receiver<Diagnostic>,
    out: RefCell<W>,
}

impl<W: io::Write> AriadneEmitter<W> {
    pub fn new(out: W) -> (Sender<Diagnostic>, Self) {
        let (sender, receiver) = mpsc::channel();
        let emitter = Self {
            out: RefCell::new(out),
            source_map: SourceMap::new(),
            receiver,
        };
        (sender, emitter)
    }

    pub fn emit(&self, diagnostic: Diagnostic) {
        type Report<'a> = ariadne::Report<'a, crate::Span>;

        if let None = diagnostic.span {
            if diagnostic.level == Level::Debug {
                let mut out = self.out.borrow_mut();
                Report::build(
                    level_to_kind(diagnostic.level),
                    SourceId::INVALID,
                    diagnostic.span.unwrap_or(Span::new(0, 0)).start,
                )
                .with_message(&diagnostic.message)
                .finish()
                .write(&self.source_map, out.by_ref())
                .unwrap();
                for child in &diagnostic.children {
                    write!(out, "{}", child.message).unwrap();
                }
                writeln!(out).unwrap();
                return;
            }
        };

        fn level_to_kind(level: Level) -> ReportKind<'static> {
            match level {
                Level::Error => ReportKind::Error,
                Level::Warning => ReportKind::Warning,
                Level::Note => ReportKind::Advice,
                Level::Help => ReportKind::Advice,
                Level::Debug => ReportKind::Custom("Debug", Color::Cyan),
            }
        }

        fn level_to_color(level: Level) -> Color {
            match level {
                Level::Error => Color::Red,
                Level::Warning => Color::Yellow,
                Level::Note => Color::Unset,
                Level::Help => Color::Unset,
                Level::Debug => Color::Unset,
            }
        }

        fn map_children(
            children: &[SubDiagnostic],
        ) -> (Vec<Label<Span>>, Vec<&String>, Vec<&String>) {
            let mut labels = vec![];
            let mut notes = vec![];
            let mut helps = vec![];

            for child in children {
                if child.spans.is_empty() {
                    match child.level {
                        Level::Note | Level::Debug => notes.push(&child.message),
                        Level::Help => helps.push(&child.message),
                        Level::Error | Level::Warning => todo!(),
                    }
                    notes.push(&child.message)
                }
                labels.extend(child.spans.iter().map(|span| {
                    Label::new(*span)
                        .with_message(&child.message)
                        .with_color(level_to_color(child.level))
                }));
            }

            (labels, notes, helps)
        }

        let (labels, notes, helps) = map_children(&diagnostic.children);

        let mut builder = Report::build(
            level_to_kind(diagnostic.level),
            SourceId::INVALID,
            diagnostic.span.unwrap_or(Span::new(0, 0)).start,
        )
        .with_message(diagnostic.message)
        .with_labels(labels);

        // TODO Properly render multiple notes/helps
        if !notes.is_empty() {
            builder.set_note(notes.into_iter().join(", "));
        }
        if !helps.is_empty() {
            builder.set_help(helps.into_iter().join(", "));
        }

        let mut out = self.out.borrow_mut();
        builder
            .finish()
            .write(&self.source_map, out.by_ref())
            .unwrap();
        writeln!(out).unwrap();
    }

    /// Emit all received [`Diagnostic`]s without blocking
    pub fn emit_all(&self) {
        for diagnostic in self.receiver.try_iter() {
            self.emit(diagnostic);
        }
    }

    /// Emit all received [`Diagnostic`]s by blocking until every [Sender] is dropped
    pub fn emit_all_blocking(self) {
        for diagnostic in self.receiver.iter() {
            self.emit(diagnostic);
        }
    }
}

#[derive(Debug)]
pub struct SourceMap {
    map: HashMap<SourceId, (String, Source)>,
    next_id: SourceId,
}

impl Cache<SourceId> for &SourceMap {
    fn fetch(&mut self, id: &SourceId) -> Result<&Source, Box<dyn Debug + '_>> {
        match self.source(*id) {
            Some(source) => Ok(source),
            None => panic!("Internal Error: SourceId '{:?}' does not exist", id),
        }
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn Display + 'a>> {
        let name = self.name(*id)?;
        Some(Box::new(name.to_owned()))
    }
}

impl SourceMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: SourceId::FIRST,
        }
    }

    pub fn add(&mut self, name: impl Into<String>, src: impl Into<Source>) -> SourceId {
        let id = self.next_id;
        self.map.insert(id, (name.into(), src.into()));
        self.next_id.0 += 1;
        id
    }

    pub fn replace(&mut self, id: SourceId, src: impl Into<Source>) {
        let Some((_, ref mut source)) = self.map.get_mut(&id) else {
            panic!("Internal Error: SourceId '{:?}' does not exist", id)
        };
        *source = src.into();
    }

    pub fn source(&self, id: SourceId) -> Option<&Source> {
        self.map.get(&id).map(|(_, source)| source)
    }

    pub fn name(&self, id: SourceId) -> Option<&String> {
        self.map.get(&id).map(|(name, _)| name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u32);

impl Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Default for SourceId {
    fn default() -> Self {
        Self::INVALID
    }
}

impl SourceId {
    pub const INVALID: Self = Self(0);
    const FIRST: Self = Self(1);
}
