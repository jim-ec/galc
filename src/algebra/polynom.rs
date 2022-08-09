use super::{metric::Metric, monom::Monomial, Product};

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

    fn add(self, mut other: Polynomial) -> Self::Output {
        let mut monomials = self.monomials;
        monomials.append(&mut other.monomials);
        Polynomial { monomials }
    }
}

impl std::ops::Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Self::Output {
        let mut monomials = self.monomials;
        monomials.push(other);
        Polynomial { monomials }
    }
}

impl Polynomial {
    pub fn product(self, product: Product, other: Polynomial, metric: &Metric) -> Polynomial {
        let mut result = Polynomial::default();
        for lhs in self.monomials {
            for rhs in &other.monomials {
                result = result + lhs.product(product, rhs, metric);
            }
        }
        result
    }
}
