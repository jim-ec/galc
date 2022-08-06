use super::{basis::Basis, metric::Metric, sign::Sign};

#[derive(Debug, Clone, PartialEq)]
pub struct Factor {
    pub scalar: f64,
    pub basis: Basis,
}

impl Factor {
    pub fn product<F>(&self, rhs: &Factor, metric: &Metric, f: F) -> Factor
    where
        F: FnOnce(&Basis, &Basis, &Metric) -> Option<(Sign, Basis)>,
    {
        if let Some((sign, basis)) = f(&self.basis, &rhs.basis, metric) {
            Factor {
                scalar: sign * self.scalar * rhs.scalar,
                basis,
            }
        } else {
            Factor {
                scalar: 0.0,
                basis: Basis::scalar(metric.dimension()),
            }
        }
    }

    pub fn geometric_product(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::geometric_product)
    }

    pub fn exterior_product(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::exterior_product)
    }

    pub fn regressive_product(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::regressive_product)
    }

    pub fn left_contraction(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::left_contraction)
    }

    pub fn right_contraction(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::right_contraction)
    }

    pub fn inner_product(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::inner_product)
    }

    pub fn scalar_product(&self, rhs: &Factor, metric: &Metric) -> Factor {
        self.product(rhs, metric, Basis::scalar_product)
    }

    pub fn reverse(&self) -> Factor {
        Factor {
            scalar: self.basis.reverse() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn involute(&self) -> Factor {
        Factor {
            scalar: self.basis.involute() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn conjugate(&self) -> Factor {
        Factor {
            scalar: self.basis.conjugate() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn dual(&self) -> Factor {
        Factor {
            scalar: self.scalar,
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
        self.scalar_product(&self.conjugate(), metric).scalar
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
    pub fn inverse(&self, metric: &Metric) -> Option<Factor> {
        let n = self.scalar_product(&self.reverse(), metric).scalar;
        if n != 0.0 {
            let mut inverse = self.reverse();
            inverse.scalar /= n;
            Some(inverse)
        } else {
            None
        }
    }

    pub fn divide(&self, rhs: &Factor, metric: &Metric) -> Option<Factor> {
        let rhs = rhs.inverse(metric)?;
        Some(self.geometric_product(&rhs, metric))
    }

    pub fn power(&self, rhs: &Factor, metric: &Metric) -> Option<(isize, Factor)> {
        if rhs.grade() != 0 {
            return None;
        }
        let rhs = rhs.scalar.round() as isize;

        let mut power = Factor {
            scalar: 1.0,
            basis: Basis::scalar(metric.dimension()),
        };
        let factor = if rhs > 0 {
            self.clone()
        } else {
            self.inverse(metric)?
        };

        for _ in 0..rhs.abs() {
            power = power.geometric_product(&factor, metric);
        }

        Some((rhs, power))
    }
}

impl std::ops::Neg for Factor {
    type Output = Factor;

    fn neg(self) -> Self::Output {
        Factor {
            scalar: -self.scalar,
            basis: self.basis,
        }
    }
}

impl std::ops::Mul<Factor> for f64 {
    type Output = Factor;

    fn mul(self, rhs: Factor) -> Self::Output {
        Factor {
            scalar: self * rhs.scalar,
            basis: rhs.basis,
        }
    }
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = self.basis.grade();
        let s = if self.scalar != 0.0 {
            self.scalar
        } else {
            self.scalar.abs()
        };

        // Print 0 if near enough
        let digits = 10;
        let epsilon = 0.1_f64.powi(digits);
        let s = if s.abs() < epsilon { 0.0 } else { s };

        if r == 0 || s == 0.0 {
            write!(f, "{}", s)
        } else if r > 0 && s == 1.0 {
            write!(f, "{}", self.basis)
        } else if r > 0 && s == -1.0 {
            write!(f, "-{}", self.basis)
        } else {
            write!(f, "{}{}", s, self.basis)
        }
    }
}
