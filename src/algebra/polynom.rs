use super::monom::Monomial;

#[derive(Debug, Clone)]
pub struct Polynomial {
    monomials: Vec<Monomial>,
}

/// The zero polynomial.
impl Default for Polynomial {
    fn default() -> Self {
        Self {
            monomials: Vec::default(),
        }
    }
}

impl std::ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, mut rhs: Polynomial) -> Self::Output {
        let mut monomials = self.monomials;
        monomials.append(&mut rhs.monomials);
        Polynomial { monomials }
    }
}

impl std::ops::Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Monomial) -> Self::Output {
        let mut monomials = self.monomials;
        monomials.push(rhs);
        Polynomial { monomials }
    }
}
