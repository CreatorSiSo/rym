use std::ops::Deref;

mod fstring;
pub use fstring::f;

pub fn print<T: Deref<Target = str>>(value: T) {
    std::io::Write::write_all(&mut std::io::stdout(), value.as_bytes()).unwrap()
}

pub trait Formatter {
    type Result;
    type Value;

    fn fill(self, value: Self::Value) -> Self;
    fn finish(self) -> Self::Result;
}
