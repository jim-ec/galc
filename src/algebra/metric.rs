#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metric(pub Vec<Square>);

impl Metric {
    pub fn dimension(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Pos,
    Neg,
    Zero,
}
