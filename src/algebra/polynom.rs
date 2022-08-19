use itertools::Itertools;

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
        Polynomial { monomials }.merge_monomials()
    }
}

impl std::ops::Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Self::Output {
        let mut monomials = self.monomials;
        monomials.push(other);
        Polynomial { monomials }.merge_monomials()
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

    pub fn power(self, exponent: isize, metric: &Metric) -> Option<Polynomial> {
        let mut result = Polynomial::default();
        for monomial in self.monomials {
            result = result + monomial.power(exponent, metric)?;
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

    /// Merges monomials with same bases and same symbols.
    pub fn merge_monomials(self) -> Polynomial {
        let mut result = Polynomial::default();
        for monomial in self.monomials {
            let mut found_monomial = false;
            for result_monomial in &mut result.monomials {
                if monomial.basis == result_monomial.basis
                    && monomial.symbols == result_monomial.symbols
                {
                    result_monomial.scalar += monomial.scalar;
                    result_monomial
                        .symbols
                        .extend(monomial.symbols.clone().into_iter());
                    found_monomial = true;
                    break;
                }
            }
            if !found_monomial {
                result.monomials.push(monomial);
            }
        }
        result
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
        if self.monomials.is_empty() {
            write!(f, "0")
        } else {
            write!(
                f,
                "{}",
                self.monomials
                    .iter()
                    .map(|monomial| monomial.to_string())
                    .join(" + ")
            )
        }
    }
}
