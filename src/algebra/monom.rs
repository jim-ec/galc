use itertools::Itertools;

use std::collections::HashMap;

use super::{basis::Basis, metric::Metric, Product};

#[derive(Debug, Clone, PartialEq)]
pub struct Monomial {
    pub scalar: f64,
    pub symbols: HashMap<String, isize>,
    pub basis: Basis,
}

impl Monomial {
    pub fn product(&self, product: Product, rhs: &Monomial, metric: &Metric) -> Monomial {
        let f = match product {
            Product::Geometric => Basis::geometric_product,
            Product::Exterior => Basis::exterior_product,
            Product::Regressive => Basis::regressive_product,
            Product::LeftContraction => Basis::left_contraction,
            Product::RightContraction => Basis::right_contraction,
            Product::Inner => Basis::inner_product,
            Product::Scalar => Basis::scalar_product,
        };
        if let Some((sign, basis)) = f(&self.basis, &rhs.basis, metric) {
            let mut symbols = self.symbols.clone();
            for (name, &multiplicity_rhs) in &rhs.symbols {
                if let Some(multiplicity) = symbols.get_mut(name) {
                    *multiplicity += multiplicity_rhs;
                } else {
                    symbols.insert(name.clone(), multiplicity_rhs);
                }
            }

            Monomial {
                scalar: sign * self.scalar * rhs.scalar,
                symbols,
                basis,
            }
        } else {
            Monomial {
                scalar: 0.0,
                symbols: Default::default(),
                basis: Basis::scalar(metric.dimension()),
            }
        }
    }

    pub fn reverse(&self) -> Monomial {
        Monomial {
            scalar: self.basis.reverse() * self.scalar,
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn involute(&self) -> Monomial {
        Monomial {
            scalar: self.basis.involute() * self.scalar,
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn conjugate(&self) -> Monomial {
        Monomial {
            scalar: self.basis.conjugate() * self.scalar,
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn dual(&self) -> Monomial {
        Monomial {
            scalar: self.scalar,
            symbols: self.symbols.clone(),
            basis: self.basis.dual(),
        }
    }

    pub fn grade(&self) -> usize {
        self.basis.grade()
    }

    pub fn anti_grade(&self) -> usize {
        self.basis.anti_grade()
    }

    pub fn norm_squared(&self, metric: &Metric) -> f64 {
        self.product(Product::Scalar, &self.conjugate(), metric)
            .scalar
    }

    pub fn norm(&self, metric: &Metric) -> f64 {
        self.norm_squared(metric).sqrt()
    }

    /// The inverse can be defined as `~A / (~A * A)`.
    /// It is thus very similar to the reversion and always has the sign,
    /// since the denominator is always a non-negative scalar.
    /// The difference is that `A * ~A = 1` only holds if `A` is normalized.
    /// On the other hand, `A * inverse(A) = 1` always holds, unless `A` vanishes.
    /// `inverse` thus behaves as a true inverse of the geometric product.
    pub fn inverse(&self, metric: &Metric) -> Option<Monomial> {
        let n = self
            .product(Product::Scalar, &self.reverse(), metric)
            .scalar;
        if n != 0.0 {
            let mut inverse = self.reverse();
            inverse.scalar /= n;
            for (_, multiplicity) in inverse.symbols.iter_mut() {
                *multiplicity = -*multiplicity;
            }
            Some(inverse)
        } else {
            None
        }
    }

    pub fn divide(&self, rhs: &Monomial, metric: &Metric) -> Option<Monomial> {
        let rhs = rhs.inverse(metric)?;
        Some(self.product(Product::Geometric, &rhs, metric))
    }

    pub fn power(&self, rhs: &Monomial, metric: &Metric) -> Option<Monomial> {
        if rhs.grade() != 0 || !rhs.symbols.is_empty() {
            return None;
        }
        let rhs = rhs.scalar.round() as isize;

        let mut power = Monomial {
            scalar: 1.0,
            symbols: Default::default(),
            basis: Basis::scalar(metric.dimension()),
        };
        let monomial = if rhs > 0 {
            self.clone()
        } else {
            self.inverse(metric)?
        };

        for _ in 0..rhs.abs() {
            power = power.product(Product::Geometric, &monomial, metric);
        }

        Some(power)
    }
}

impl std::ops::Neg for Monomial {
    type Output = Monomial;

    fn neg(self) -> Self::Output {
        Monomial {
            scalar: -self.scalar,
            symbols: self.symbols,
            basis: self.basis,
        }
    }
}

impl std::ops::Mul<Monomial> for f64 {
    type Output = Monomial;

    fn mul(self, rhs: Monomial) -> Self::Output {
        Monomial {
            scalar: self * rhs.scalar,
            symbols: rhs.symbols,
            basis: rhs.basis,
        }
    }
}

impl std::fmt::Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ", self.scalar)?;

        for (name, &multiplicity) in self.symbols.iter().sorted_by(|(a, _), (b, _)| a.cmp(b)) {
            write!(f, "{name}")?;
            if multiplicity != 1 {
                write!(f, "^{multiplicity}")?;
            }
            write!(f, " ")?;
        }

        write!(f, "{}", self.basis)?;

        Ok(())
    }
}
