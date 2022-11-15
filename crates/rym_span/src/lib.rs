use std::ops::Range;

pub type Span = Range<usize>;

pub trait ImplSpan {}

impl ImplSpan for Span {}
