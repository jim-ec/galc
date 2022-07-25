use super::{
    metric::{Metric, Square},
    sign::Sign,
};

/// Encodes a factorization of a blade:
/// `A = B eᵢ` ⇔ `A[i]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape(pub Vec<bool>);

impl Shape {
    pub fn one(dimension: usize) -> Shape {
        Shape(vec![false; dimension])
    }

    pub fn pseudoscalar(dimension: usize) -> Shape {
        Shape(vec![true; dimension])
    }

    pub fn dimension(&self) -> usize {
        self.0.len()
    }

    /// Parity of the reversion operator, rewriting its factors in reverse order.
    /// - `rev(eᵢⱼ) = eⱼᵢ = -eᵢⱼ` ⇔ `i ≠ j`
    pub fn reverse(&self) -> Sign {
        let r = self.grade();
        if r > 0 && odd(r * (r - 1) / 2) {
            Sign::Neg
        } else {
            Sign::Pos
        }
    }

    /// Parity of the grade involution, reversing the sign of odd blades.
    pub fn involute(&self) -> Sign {
        if even(self.grade()) {
            Sign::Pos
        } else {
            Sign::Neg
        }
    }

    /// Clifford Conjugate
    pub fn conjugate(&self) -> Sign {
        self.reverse() * self.involute()
    }

    /// Poincaré duality operator
    pub fn dual(&self) -> Shape {
        Shape(self.0.iter().map(|factor| !factor).collect())
    }

    /// Compute the geometric product between two blades.
    /// - `eᵢeᵢ = 1`
    /// - `eᵢeⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢeⱼ = -eⱼeᵢ`
    pub fn geometric(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        debug_assert_eq!(
            self.dimension(),
            rhs.dimension(),
            "To compute the geometric product, both blades must have the same dimension"
        );
        debug_assert_eq!(
            self.dimension(),
            metric.dimension(),
            "To compute the geometric product, the blade and metric must match in dimension"
        );
        let mut product = vec![false; self.dimension()];
        let mut sign = Sign::Pos;

        for i in 0..self.dimension() {
            if self.0[i] {
                // Since shapes do not encode any order of factorization, a sign reversal
                // must accomodate for each permutation.
                for j in 0..i {
                    if rhs.0[j] {
                        sign = -sign;
                    }
                }
            }
            product[i] = match (self.0[i], rhs.0[i]) {
                (true, false) | (false, true) => true,
                (true, true) => match metric.0[i] {
                    Square::Pos => {
                        // eᵢeᵢ = 1
                        false
                    }
                    Square::Neg => {
                        // eᵢeᵢ = -1
                        sign = -sign;
                        false
                    }
                    Square::Zero => {
                        // eᵢeᵢ = 0
                        return None;
                    }
                },
                (false, false) => false,
            }
        }

        Some((sign, Shape(product)))
    }

    // Compute the exterior product between two blades.
    /// - `eᵢ ∧ eᵢ = 0`
    /// - `eᵢ ∧ eⱼ = eᵢⱼ` ⇔ `i ≠ j`
    ///- `eᵢ ∧ eⱼ = -eⱼeᵢ`
    pub fn exterior(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        let (sign, product) = self.geometric(rhs, metric)?;
        if self.grade() + rhs.grade() == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    // Compute the regressive product between two blades using the identity
    /// `A ∨ B = J(J(A) ∧ J(B))`
    pub fn regressive(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        self.dual()
            .exterior(&rhs.dual(), metric)
            .map(|(sign, product)| (sign, product.dual()))
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-blade of `rhs` which is prependicular to `self.
    pub fn left_contraction(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        let (sign, product) = self.geometric(rhs, metric)?;
        if rhs.grade().checked_sub(self.grade()) == Some(product.grade()) {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Contraction of `self` by `rhs`.
    /// `A << B = (B~ >> A~)~`
    /// Intuitively, this returns the sub-blade of `self` which is prependicular to `rhs.
    pub fn right_contraction(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        let (sign, product) = rhs.left_contraction(self, metric)?;
        let sign = sign * rhs.reverse() * self.reverse() * product.reverse();
        Some((sign, product))
    }

    /// Bi-directional contraction.
    pub fn inner(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        let (sign, product) = self.geometric(rhs, metric)?;
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Scalar product, producing non-zero scalars only when grades match.
    /// In that case, the result can be interpreted as a metric between blades:
    /// `A~ * A` can be used as the squared norm of `A`.
    pub fn scalar(&self, rhs: &Shape, metric: &Metric) -> Option<(Sign, Shape)> {
        let (sign, product) = self.geometric(rhs, metric)?;
        if product.grade() == 0 {
            Some((sign, product))
        } else {
            None
        }
    }

    /// The *grade* (sometime also called *step*) of this blade, equating to the number of distinct factors.
    /// Returns [None] if this shape is vanishing.
    pub fn grade(&self) -> usize {
        self.0
            .iter()
            .fold(0, |grade, factor| if *factor { grade + 1 } else { grade })
    }

    pub fn anti_grade(&self) -> usize {
        self.dimension() - self.grade()
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.anti_grade() == 0 {
            write!(f, "i")?;
        } else {
            write!(f, "e")?;
            for (i, factor) in self.0.iter().enumerate() {
                if *factor {
                    write!(f, "{i}")?;
                }
            }
        }
        Ok(())
    }
}

fn even(n: usize) -> bool {
    n & 1 == 0
}

fn odd(n: usize) -> bool {
    n & 1 != 0
}
