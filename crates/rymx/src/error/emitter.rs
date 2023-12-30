use super::Diagnostic;
use crate::span::SourceId;
use ariadne::{Cache, FileCache, Source};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::mpsc::{self, Receiver, Sender};

// pub trait Emitter {
//     fn emit_diagnostic(&mut self, diagnostic: &Diagnostic);
// }

pub struct AriadneEmitter<'a> {
    out: RefCell<Write<'a>>,
    cache: DynamicSourceMap,
    pub receiver: Receiver<Diagnostic>,
}

impl<'a> AriadneEmitter<'a> {
    pub fn new(out: Box<dyn std::io::Write>) -> (Sender<Diagnostic>, Self) {
        Self::new_inner(Write::Io(out))
    }

    pub fn new_string_out(out: &'a mut String) -> (Sender<Diagnostic>, Self) {
        Self::new_inner(Write::String(out))
    }

    fn new_inner(out: Write<'a>) -> (Sender<Diagnostic>, Self) {
        let (sender, receiver) = mpsc::channel();
        (
            sender,
            Self {
                out: out.into(),
                cache: DynamicSourceMap::new(),
                receiver,
            },
        )
    }

    pub fn add_source<'src>(&'src mut self, label: &'static str, src: &'src str) {
        self.cache.other.insert(label, Source::from(src));
    }

    pub fn emit(&self, diagnostic: Diagnostic) {
        self.out
            .borrow_mut()
            .write_str(&format!("{:?}: {}\n", diagnostic.level, diagnostic.message))
            .unwrap();
        for diagnostic in diagnostic.children {
            self.out
                .borrow_mut()
                .write_str(&format!("{}", diagnostic.message))
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

pub enum Write<'a> {
    Io(Box<dyn std::io::Write>),
    String(&'a mut String),
}

impl Write<'_> {
    fn write_str(&mut self, str: &str) -> anyhow::Result<()> {
        match self {
            Write::Io(inner) => inner.write_all(str.as_bytes())?,
            Write::String(inner) => std::fmt::Write::write_str(inner, str)?,
        }
        Ok(())
    }
}

struct DynamicSourceMap {
    files: FileCache,
    other: HashMap<&'static str, Source>,
}

impl Cache<SourceId> for &mut DynamicSourceMap {
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

impl DynamicSourceMap {
    fn new() -> Self {
        Self {
            files: FileCache::default(),
            other: HashMap::new(),
        }
    }
}
