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

impl std::ops::Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial {
            monomials: self
                .monomials
                .into_iter()
                .map(|monomial| -monomial)
                .collect(),
        }
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

    pub fn power(self, other: Polynomial, metric: &Metric) -> Option<Polynomial> {
        if other.monomials.len() != 1 {
            return None;
        }

        let mut result = Polynomial::default();
        for monomial in self.monomials {
            result = result + monomial.power(&other.monomials[0], metric)?;
        }
        Some(result)
    }

    pub fn inverse(self, metric: &Metric) -> Option<Polynomial> {
        let mut inverse_monomials = Vec::new();
        for monomial in self.monomials {
            inverse_monomials.push(monomial.inverse(metric)?);
        }
        Some(Polynomial {
            monomials: inverse_monomials,
        })
    }

    pub fn dual(self) -> Polynomial {
        Polynomial {
            monomials: self
                .monomials
                .into_iter()
                .map(|monomial| monomial.dual())
                .collect(),
        }
    }

    pub fn involute(self) -> Polynomial {
        Polynomial {
            monomials: self
                .monomials
                .into_iter()
                .map(|monomial| monomial.involute())
                .collect(),
        }
    }

    pub fn conjugate(self) -> Polynomial {
        Polynomial {
            monomials: self
                .monomials
                .into_iter()
                .map(|monomial| monomial.conjugate())
                .collect(),
        }
    }

    pub fn reverse(self) -> Polynomial {
        Polynomial {
            monomials: self
                .monomials
                .into_iter()
                .map(|monomial| monomial.reverse())
                .collect(),
        }
    }

    pub fn norm(self, metric: &Metric) -> f64 {
        self.monomials
            .into_iter()
            .map(|monomial| monomial.norm(metric))
            .sum()
    }
}

impl std::convert::From<Monomial> for Polynomial {
    fn from(monomial: Monomial) -> Self {
        Polynomial {
            monomials: vec![monomial],
        }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
