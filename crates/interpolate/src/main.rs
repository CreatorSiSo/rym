use std::{fmt::Display, marker::PhantomData, ops::Deref};

trait Formatter {
    type Result;
    type Value;

    fn fill(self, value: Self::Value) -> Self;
    fn finish(self) -> Self::Result;
}

pub struct StringFormatter<T, const LENGTH: usize> {
    parts: [&'static str; LENGTH],
    result: String,
    next: usize,
    phantom: PhantomData<T>,
}

impl<T: Display, const LENGTH: usize> Formatter for StringFormatter<T, LENGTH> {
    type Result = String;
    type Value = T;

    fn fill(mut self, value: Self::Value) -> Self {
        self.result.push_str(&value.to_string());
        self.result.push_str(self.parts[self.next]);
        self.next += 1;
        self
    }

    fn finish(self) -> Self::Result {
        self.result
    }
}

pub fn f<T, const LENGTH: usize>(parts: [&'static str; LENGTH]) -> StringFormatter<T, LENGTH> {
    assert!(LENGTH > 0);

    StringFormatter {
        parts,
        result: String::from(parts[0]),
        next: 1,
        phantom: PhantomData::default(),
    }
}

fn print<T: Deref<Target = str>>(value: T) {
    std::io::Write::write_all(&mut std::io::stdout(), value.as_bytes()).unwrap()
}

fn main() {
    let name = "Robot";

    // Generated from: ```
    // print(f"Hello {name}!\n");
    // ```
    print(f(["Hello ", "!\n"]).fill(name).finish());
}
