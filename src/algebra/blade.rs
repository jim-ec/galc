use super::{basis::Basis, metric::Metric};

#[derive(Debug, Clone, PartialEq)]
pub struct Blade(pub f64, pub Basis);

impl Blade {
    pub fn null(dimension: usize) -> Blade {
        Blade(0.0, Basis(vec![false; dimension]))
    }

    pub fn one(dimension: usize) -> Blade {
        Blade(1.0, Basis(vec![false; dimension]))
    }

    pub fn from(scalar: f64, dimension: usize) -> Blade {
        Blade(scalar, Basis(vec![false; dimension]))
    }

    pub fn geometric(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.geometric(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn exterior(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.exterior(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn regressive(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.regressive(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn left_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.left_contraction(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn right_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.right_contraction(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn inner(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.inner(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn scalar(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, basis)) = self.1.scalar(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, basis)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn reverse(&self) -> Blade {
        Blade(self.1.reverse() * self.0, self.1.clone())
    }

    pub fn involute(&self) -> Blade {
        Blade(self.1.involute() * self.0, self.1.clone())
    }

    pub fn conjugate(&self) -> Blade {
        Blade(self.1.conjugate() * self.0, self.1.clone())
    }

    pub fn dual(&self) -> Blade {
        Blade(self.0, self.1.dual())
    }

    pub fn grade(&self) -> usize {
        self.1.grade()
    }

    pub fn anti_grade(&self) -> usize {
        self.1.anti_grade()
    }

    pub fn norm_squared(&self, metric: &Metric) -> f64 {
        self.scalar(&self.reverse(), metric).0
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
        let n = self.scalar(&self.reverse(), metric).0;
        if n != 0.0 {
            let mut inverse = self.reverse();
            inverse.0 /= n;
            Some(inverse)
        } else {
            None
        }
    }

    pub fn divide(&self, rhs: &Blade, metric: &Metric) -> Option<Blade> {
        let rhs = rhs.inverse(metric)?;
        Some(self.geometric(&rhs, metric))
    }

    pub fn exponentiate(&self, rhs: &Blade, metric: &Metric) -> Option<Blade> {
        println!("Warning: Exponent must be a scalar integer");
        if rhs.grade() != 0 {
            return None;
        }
        let rhs = rhs.0.round() as isize;

        let mut exponentiation = Blade::one(metric.dimension());
        let factor = if rhs > 0 {
            self.clone()
        } else {
            self.inverse(metric)?
        };

        for _ in 0..rhs.abs() {
            exponentiation = exponentiation.geometric(&factor, metric);
        }

        Some(exponentiation)
    }
}

impl std::ops::Neg for Blade {
    type Output = Blade;

    fn neg(self) -> Self::Output {
        Blade(-self.0, self.1)
    }
}

impl std::ops::Mul<Blade> for f64 {
    type Output = Blade;

    fn mul(self, rhs: Blade) -> Self::Output {
        Blade(self * rhs.0, rhs.1)
    }
}

impl std::fmt::Display for Blade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = self.1.grade();
        let s = if self.0 != 0.0 { self.0 } else { self.0.abs() };

        // Print 0 if near enough
        let digits = 10;
        let epsilon = 0.1_f64.powi(digits);
        let s = if s.abs() < epsilon { 0.0 } else { s };

        if r == 0 || s == 0.0 {
            write!(f, "{}", s)
        } else if r > 0 && s == 1.0 {
            write!(f, "{}", self.1)
        } else if r > 0 && s == -1.0 {
            write!(f, "-{}", self.1)
        } else {
            write!(f, "{}{}", s, self.1)
        }
    }
}
