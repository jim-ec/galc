use super::{blade::Blade, metric::Metric};

pub struct Monom(Vec<String>, Blade);

impl Monom {
    pub fn geometric(self, rhs: Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::geometric)
    }

    pub fn exterior(self, rhs: Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::exterior)
    }

    pub fn left_contraction(self, rhs: Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::left_contraction)
    }

    fn product<F>(self, rhs: Monom, metric: &Metric, f: F) -> Monom
    where
        F: FnOnce(&Blade, &Blade, &Metric) -> Blade,
    {
        let mut names = self.0;
        names.extend(rhs.0);
        Monom(names, f(&self.1, &rhs.1, metric))
    }

    pub fn norm(self, metric: &Metric) -> Monom {
        Monom(self.0, Blade::from(self.1.norm(metric), metric.dimension()))
    }
}
