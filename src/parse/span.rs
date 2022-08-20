use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Spanned<T>(pub T, pub Range<usize>);
