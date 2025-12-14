use super::{Diagnostic, Level};
use crate::error::diagnostic::SubDiagnostic;
use ariadne::{Cache, Color, Config, Label, ReportKind, Source};
use itertools::Itertools;
use span::{SourceId, Span};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::sync::{Mutex, mpsc};

// pub trait Emitter {
//     fn emit_diagnostic(&mut self, diagnostic: &Diagnostic);
// }

pub struct AriadneEmitter<W: io::Write> {
    pub source_map: SourceMap,
    receiver: mpsc::Receiver<Diagnostic>,
    out: Mutex<W>,
}

impl<W: io::Write> AriadneEmitter<W> {
    pub fn new(out: W) -> (mpsc::Sender<Diagnostic>, Self) {
        let (sender, receiver) = mpsc::channel();
        let emitter = Self {
            out: Mutex::new(out),
            source_map: SourceMap::new(),
            receiver,
        };
        (sender, emitter)
    }

    pub fn emit(&self, diagnostic: Diagnostic) {
        type Report<'a> = ariadne::Report<'a, crate::Span>;

        let config = Config::default()
            .with_char_set(ariadne::CharSet::Unicode)
            .with_cross_gap(true);

        if diagnostic.span.is_none() && diagnostic.level == Level::Debug {
            let mut builder = Report::build(
                level_to_kind(diagnostic.level),
                diagnostic.span.unwrap_or(Span::new(0, 0)),
            )
            .with_config(config)
            .with_message(&diagnostic.message);

            for child in &diagnostic.children {
                builder.set_note(&child.message);
            }

            let mut out = self.out.lock().unwrap();
            builder
                .finish()
                .write(&self.source_map, out.by_ref())
                .unwrap();
            writeln!(out).unwrap();

            return;
        };

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

        let mut builder = Report::build(
            level_to_kind(diagnostic.level),
            diagnostic.span.unwrap_or(Span::new(0, 0)),
        )
        .with_config(config)
        .with_message(&diagnostic.message);

        if diagnostic.children.is_empty()
            && let Some(span) = diagnostic.span
        {
            builder.add_label(
                Label::new(span)
                    .with_color(level_to_color(diagnostic.level))
                    .with_message(diagnostic.message),
            )
        } else {
            let (labels, notes, helps) = map_children(&diagnostic.children);
            builder.add_labels(labels);

            // TODO Properly render multiple notes/helps
            if !notes.is_empty() {
                builder.set_note(notes.into_iter().join(", "));
            }
            if !helps.is_empty() {
                builder.set_help(helps.into_iter().join(", "));
            }
        }

        let mut out = self.out.lock().unwrap();
        builder
            .finish()
            .write(&self.source_map, out.by_ref())
            .unwrap();
        writeln!(out).unwrap();
    }

    /// Emit all received [`Diagnostic`]s without blocking
    // pub fn emit_all(&self) {
    //     for diagnostic in self.receiver.try_iter() {
    //         self.emit(diagnostic);
    //     }
    // }

    /// Emit all received [`Diagnostic`]s by blocking until every [Sender] is dropped
    pub fn emit_all_blocking(self) {
        for diagnostic in self.receiver.iter() {
            self.emit(diagnostic);
        }
    }
}

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
        Level::Error => Color::BrightRed,
        Level::Warning => Color::BrightYellow,
        Level::Note => Color::BrightGreen,
        Level::Help => Color::BrightBlue,
        Level::Debug => Color::default(),
    }
}

#[derive(Debug)]
pub struct SourceMap {
    map: HashMap<SourceId, (String, Source)>,
    id: SourceId,
}

impl Cache<SourceId> for &SourceMap {
    type Storage = String;

    fn fetch(&mut self, id: &SourceId) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        match self.source(*id) {
            Some(source) => Ok::<&ariadne::Source, String>(source),
            None => panic!("Internal Error: SourceId '{:?}' does not exist", id),
        }
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<impl std::fmt::Display + 'a> {
        let name = self.name(*id)?;
        Some(Box::new(name.to_owned()))
    }
}

impl SourceMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            id: SourceId::default(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, src: Source) -> SourceId {
        let prev_id = self.id;
        let id = SourceId::new(prev_id);
        self.id = id;
        self.map.insert(id, (name.into(), src));
        id
    }

    pub fn replace(&mut self, id: SourceId, src: Source) {
        let Some((_, source)) = self.map.get_mut(&id) else {
            panic!("Internal Error: SourceId '{:?}' does not exist", id)
        };
        *source = src;
    }

    pub fn source(&self, id: SourceId) -> Option<&Source> {
        self.map.get(&id).map(|(_, source)| source)
    }

    pub fn name(&self, id: SourceId) -> Option<&String> {
        self.map.get(&id).map(|(name, _)| name)
    }
}
