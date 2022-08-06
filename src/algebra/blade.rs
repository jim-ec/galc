use super::{basis::Basis, metric::Metric, sign::Sign};

#[derive(Debug, Clone, PartialEq)]
pub struct Blade {
    pub scalar: f64,
    pub basis: Basis,
}

impl Blade {
    pub fn product<F>(&self, rhs: &Blade, metric: &Metric, f: F) -> Blade
    where
        F: FnOnce(&Basis, &Basis, &Metric) -> Option<(Sign, Basis)>,
    {
        if let Some((sign, basis)) = f(&self.basis, &rhs.basis, metric) {
            Blade {
                scalar: sign * self.scalar * rhs.scalar,
                basis,
            }
        } else {
            Blade {
                scalar: 0.0,
                basis: Basis::one(metric.dimension()),
            }
        }
    }

    pub fn geometric(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::geometric)
    }

    pub fn exterior(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::exterior)
    }

    pub fn regressive(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::regressive)
    }

    pub fn left_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::left_contraction)
    }

    pub fn right_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::right_contraction)
    }

    pub fn inner(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::inner)
    }

    pub fn scalar(&self, rhs: &Blade, metric: &Metric) -> Blade {
        self.product(rhs, metric, Basis::scalar)
    }

    pub fn reverse(&self) -> Blade {
        Blade {
            scalar: self.basis.reverse() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn involute(&self) -> Blade {
        Blade {
            scalar: self.basis.involute() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn conjugate(&self) -> Blade {
        Blade {
            scalar: self.basis.conjugate() * self.scalar,
            basis: self.basis.clone(),
        }
    }

    pub fn dual(&self) -> Blade {
        Blade {
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
        self.scalar(&self.conjugate(), metric).scalar
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
    pub fn inverse(&self, metric: &Metric) -> Option<Blade> {
        let n = self.scalar(&self.reverse(), metric).scalar;
        if n != 0.0 {
            let mut inverse = self.reverse();
            inverse.scalar /= n;
            Some(inverse)
        } else {
            None
        }
    }

    pub fn divide(&self, rhs: &Blade, metric: &Metric) -> Option<Blade> {
        let rhs = rhs.inverse(metric)?;
        Some(self.geometric(&rhs, metric))
    }

    pub fn power(&self, rhs: &Blade, metric: &Metric) -> Option<(isize, Blade)> {
        if rhs.grade() != 0 {
            return None;
        }
        let rhs = rhs.scalar.round() as isize;

        let mut power = Blade {
            scalar: 1.0,
            basis: Basis::one(metric.dimension()),
        };
        let factor = if rhs > 0 {
            self.clone()
        } else {
            self.inverse(metric)?
        };

        for _ in 0..rhs.abs() {
            power = power.geometric(&factor, metric);
        }

        Some((rhs, power))
    }
}

impl std::ops::Neg for Blade {
    type Output = Blade;

    fn neg(self) -> Self::Output {
        Blade {
            scalar: -self.scalar,
            basis: self.basis,
        }
    }
}

impl std::ops::Mul<Blade> for f64 {
    type Output = Blade;

    fn mul(self, rhs: Blade) -> Self::Output {
        Blade {
            scalar: self * rhs.scalar,
            basis: rhs.basis,
        }
    }
}

impl std::fmt::Display for Blade {
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
