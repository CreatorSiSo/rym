use super::Diagnostic;
use crate::span::SourceId;
use ariadne::{Cache, FileCache, Source};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io;
use std::sync::mpsc::{self, Receiver, Sender};

// pub trait Emitter {
//     fn emit_diagnostic(&mut self, diagnostic: &Diagnostic);
// }

pub struct AriadneEmitter<W: io::Write> {
    out: RefCell<W>,
    cache: SourceMap,
    pub receiver: Receiver<Diagnostic>,
}

impl<W: io::Write> AriadneEmitter<W> {
    pub fn new(out: W) -> (Sender<Diagnostic>, Self) {
        let (sender, receiver) = mpsc::channel();
        let emitter = Self {
            out: RefCell::new(out),
            cache: SourceMap::new(),
            receiver,
        };
        (sender, emitter)
    }

    pub fn add_source<'src>(&'src mut self, label: &'static str, src: &'src str) {
        self.cache.other.insert(label, Source::from(src));
    }

    pub fn emit(&self, diagnostic: Diagnostic) {
        self.out
            .borrow_mut()
            .write_all(format!("{:?}: {}\n", diagnostic.level, diagnostic.message).as_bytes())
            .unwrap();
        for diagnostic in diagnostic.children {
            self.out
                .borrow_mut()
                .write_all(format!("{}", diagnostic.message).as_bytes())
                .unwrap();
        }
    }

    // /// Waits for a diagnostic, then formats and writes it to self.out
    // pub fn receive(&mut self) -> anyhow::Result<()> {
    //     let diagnostic = self.receiver.recv()?;
    //     self.out.write_all(format!("{diagnostic:?}").as_bytes())?;
    //     Ok(())
    // }
}

struct SourceMap {
    files: FileCache,
    other: HashMap<&'static str, Source>,
}

impl Cache<SourceId> for &mut SourceMap {
    fn fetch(&mut self, id: &SourceId) -> Result<&Source, Box<dyn Debug + '_>> {
        match id {
            SourceId::File(pathbuf) => self.files.fetch(pathbuf),
            SourceId::Other(id) => self.other.get(id).ok_or(Box::new(format!(
                "Could not find source cache entry [{id}]"
            ))),
        }
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn Display + 'a>> {
        match id {
            SourceId::File(pathbuf) => Some(Box::new(pathbuf.display())),
            SourceId::Other(id) => Some(Box::new(*id)),
        }
    }
}

impl SourceMap {
    fn new() -> Self {
        Self {
            files: FileCache::default(),
            other: HashMap::new(),
        }
    }
}
