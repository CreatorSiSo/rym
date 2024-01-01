use crate::Formatter;
use std::{fmt::Display, marker::PhantomData};

pub struct StringFormatter<T, const LENGTH: usize> {
    parts: [&'static str; LENGTH],
    result: String,
    next: usize,
    phantom: PhantomData<T>,
}

impl<T: Display, const LENGTH: usize> Formatter for StringFormatter<T, LENGTH> {
    type Result = String;
    type Value = T;

    #[inline]
    fn fill(mut self, value: Self::Value) -> Self {
        self.result.push_str(&value.to_string());
        self.result.push_str(self.parts[self.next]);
        self.next += 1;
        self
    }

    #[inline]
    fn finish(self) -> Self::Result {
        self.result
    }
}

#[inline]
pub fn f<T, const LENGTH: usize>(parts: [&'static str; LENGTH]) -> StringFormatter<T, LENGTH> {
    assert!(LENGTH > 0);

    StringFormatter {
        parts,
        result: String::from(parts[0]),
        next: 1,
        phantom: PhantomData::default(),
    }
}
