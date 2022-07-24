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

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, square) in self.0.iter().enumerate() {
            write!(
                f,
                "e{i}²={} ",
                match square {
                    Square::Pos => "1",
                    Square::Neg => "-1",
                    Square::Zero => "0",
                }
            )?;
        }
        Ok(())
    }
}
