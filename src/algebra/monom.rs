use itertools::Itertools;

use num::{BigRational, One, Signed, Zero};

use std::collections::HashMap;

use super::{
    basis::Basis,
    metric::{Metric, Square},
    Product,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Monomial {
    pub scalar: BigRational,
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
                scalar: sign * self.scalar.clone() * rhs.scalar.clone(),
                symbols,
                basis,
            }
        } else {
            Monomial {
                scalar: BigRational::zero(),
                symbols: Default::default(),
                basis: Basis::scalar(metric.dimension()),
            }
        }
    }

    pub fn reverse(&self) -> Monomial {
        Monomial {
            scalar: self.basis.reverse() * self.scalar.clone(),
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn involute(&self) -> Monomial {
        Monomial {
            scalar: self.basis.involution() * self.scalar.clone(),
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn conjugate(&self) -> Monomial {
        Monomial {
            scalar: self.basis.conjugate() * self.scalar.clone(),
            symbols: self.symbols.clone(),
            basis: self.basis.clone(),
        }
    }

    pub fn dual(&self) -> Monomial {
        Monomial {
            scalar: self.scalar.clone(),
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

    pub fn norm_squared(&self, metric: &Metric) -> BigRational {
        self.product(Product::Scalar, &self.conjugate(), metric)
            .scalar
            .abs()
    }

    pub fn norm(&self, metric: &Metric) -> BigRational {
        for (i, &vector) in self.basis.0.iter().enumerate() {
            if vector && metric.0[i] == Square::Zero {
                return BigRational::zero();
            }
        }
        self.scalar.abs()
    }

    /// The inverse can be defined as `~A / (~A * A)`.
    /// It is thus very similar to the reversion and always has the sign,
    /// since the denominator is always a non-negative scalar.
    /// The difference is that `A * ~A = 1` only holds if `A` is normalized.
    /// On the other hand, `A * inverse(A) = 1` always holds, unless `A` vanishes.
    /// `inverse` thus behaves as a true inverse of the geometric product.
    pub fn inverse(&self, metric: &Metric) -> Option<Monomial> {
        if self.norm_squared(metric) == BigRational::zero() {
            return None;
        }

        let mut inverse = self.reverse();

        inverse.scalar /= self.norm_squared(metric);

        for (_, multiplicity) in inverse.symbols.iter_mut() {
            *multiplicity = -*multiplicity;
        }

        Some(inverse)
    }

    pub fn power(&self, exponent: isize, metric: &Metric) -> Option<Monomial> {
        let mut power = Monomial {
            scalar: BigRational::one(),
            symbols: Default::default(),
            basis: Basis::scalar(metric.dimension()),
        };
        let monomial = if exponent > 0 {
            self.clone()
        } else {
            self.inverse(metric)?
        };

        for _ in 0..exponent.abs() {
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

impl std::ops::Mul<Monomial> for BigRational {
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
        let symbols = self
            .symbols
            .iter()
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(name, &multiplicity)| {
                if multiplicity == 1 {
                    format!("{name}")
                } else {
                    format!("{name}^{multiplicity}")
                }
            })
            .join(" ");

        let just_scalar = symbols.is_empty() && self.basis.grade() == 0;
        if self.scalar == -BigRational::one() && !just_scalar {
            write!(f, "-")?;
        } else if self.scalar != BigRational::one() || just_scalar {
            write!(f, "{}", self.scalar)?;
            if !just_scalar {
                write!(f, " ")?;
            }
        }

        write!(f, "{symbols}")?;

        if self.anti_grade() == 0 && self.basis.dimension() > 0 || self.grade() > 0 {
            if !symbols.is_empty() {
                write!(f, " ")?;
            }
        }

        write!(f, "{}", self.basis)?;

        Ok(())
    }
}
