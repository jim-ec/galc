use super::factor::Factor;

#[derive(Debug, Clone)]
pub struct Polynomial {
    factors: Vec<Factor>,
}

/// The zero polynomial.
impl Default for Polynomial {
    fn default() -> Self {
        Self {
            factors: Vec::default(),
        }
    }
}

impl std::ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, mut rhs: Polynomial) -> Self::Output {
        let mut factors = self.factors;
        factors.append(&mut rhs.factors);
        Polynomial { factors }
    }
}

impl std::ops::Add<Factor> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Factor) -> Self::Output {
        let mut factors = self.factors;
        factors.push(rhs);
        Polynomial { factors }
    }
}
