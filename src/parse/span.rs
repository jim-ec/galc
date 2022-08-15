use std::ops::Range;

pub struct Spanned<T>(pub T, pub Range<usize>);
