use crate::{metric::Metric, shape::Shape};

pub struct Blade(pub f64, pub Shape);

impl Blade {
    pub fn null(dimension: usize) -> Blade {
        Blade(0.0, Shape(vec![false; dimension]))
    }

    pub fn geometric(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, shape)) = self.1.geometric(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, shape)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn left_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, shape)) = self.1.left_contraction(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, shape)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn right_contraction(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, shape)) = self.1.right_contraction(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, shape)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn inner(&self, rhs: &Blade, metric: &Metric) -> Blade {
        if let Some((sign, shape)) = self.1.inner(&rhs.1, metric) {
            Blade(sign * self.0 * rhs.0, shape)
        } else {
            Blade::null(self.1.dimension())
        }
    }

    pub fn scalar(&self, rhs: &Blade, metric: &Metric) -> f64 {
        if let Some((sign, _)) = self.1.scalar(&rhs.1, metric) {
            sign * self.0 * rhs.0
        } else {
            0.0
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
        self.scalar(self, metric)
    }

    pub fn norm(&self, metric: &Metric) -> f64 {
        self.norm_squared(metric).sqrt()
    }

    pub fn normalized(&self, metric: &Metric) -> Option<Blade> {
        let n = self.norm(metric);
        if n == 0.0 {
            return None;
        }
        Some(Blade(self.1.reverse() * self.0 / n, self.1.clone()))
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
        write!(f, "{}{}", self.0, self.1)
    }
}
