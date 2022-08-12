use super::{
    metric::{Metric, Square},
    sign::Sign,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basis(pub Vec<bool>);

impl Basis {
    /// Basis for a scalar.
    pub fn scalar(dimension: usize) -> Basis {
        Basis(vec![false; dimension])
    }

    /// Basis for a pseudo-scalar.
    pub fn pseudoscalar(dimension: usize) -> Basis {
        Basis(vec![true; dimension])
    }

    pub fn dimension(&self) -> usize {
        self.0.len()
    }

    /// Parity of the reversion operator.
    pub fn reverse(&self) -> Sign {
        let r = self.grade();
        if r > 0 && odd(r * (r - 1) / 2) {
            Sign::Neg
        } else {
            Sign::Pos
        }
    }

    /// Parity of the grade involution, reversing the sign of odd graded bases.
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
    pub fn dual(&self) -> Basis {
        Basis(self.0.iter().map(|vector| !vector).collect())
    }

    /// Geometric product
    pub fn geometric_product(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        debug_assert_eq!(
            self.dimension(),
            rhs.dimension(),
            "To compute the geometric product, both bases must have the same dimension"
        );
        debug_assert_eq!(
            self.dimension(),
            metric.dimension(),
            "To compute the geometric product, the basis and metric must match in dimension"
        );
        let mut vectors = vec![false; self.dimension()];
        let mut sign = Sign::Pos;

        for i in 0..self.dimension() {
            if self.0[i] {
                for j in 0..i {
                    if rhs.0[j] {
                        sign = -sign;
                    }
                }
            }
            vectors[i] = match (self.0[i], rhs.0[i]) {
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

        Some((sign, Basis(vectors)))
    }

    /// Exterior product
    pub fn exterior_product(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        let (sign, product) = self.geometric_product(rhs, metric)?;
        if self.grade() + rhs.grade() == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Regressive product
    pub fn regressive_product(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        self.dual()
            .exterior_product(&rhs.dual(), metric)
            .map(|(sign, product)| (sign, product.dual()))
    }

    /// Contraction of `self` onto `rhs`.
    /// Intuitively, this returns the sub-basis of `rhs` which is prependicular to `self`.
    pub fn left_contraction(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        let (sign, product) = self.geometric_product(rhs, metric)?;
        if rhs.grade().checked_sub(self.grade()) == Some(product.grade()) {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Contraction of `self` by `rhs`.
    /// Intuitively, this returns the sub-basis of `self` which is prependicular to `rhs`.
    pub fn right_contraction(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        let (sign, product) = rhs.left_contraction(self, metric)?;
        let sign = sign * rhs.reverse() * self.reverse() * product.reverse();
        Some((sign, product))
    }

    /// Inner product
    pub fn inner_product(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        let (sign, product) = self.geometric_product(rhs, metric)?;
        if rhs.grade().abs_diff(self.grade()) == product.grade() {
            Some((sign, product))
        } else {
            None
        }
    }

    /// Scalar product
    pub fn scalar_product(&self, rhs: &Basis, metric: &Metric) -> Option<(Sign, Basis)> {
        let (sign, product) = self.geometric_product(rhs, metric)?;
        if product.grade() == 0 {
            Some((sign, product))
        } else {
            None
        }
    }

    pub fn grade(&self) -> usize {
        self.0
            .iter()
            .fold(0, |grade, &vector| if vector { grade + 1 } else { grade })
    }

    pub fn anti_grade(&self) -> usize {
        self.dimension() - self.grade()
    }
}

impl std::fmt::Display for Basis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.anti_grade() == 0 {
            write!(f, "i")?;
        } else if self.grade() > 0 {
            write!(f, "e")?;
            for (i, vector) in self.0.iter().enumerate() {
                if *vector {
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
