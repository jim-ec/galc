use super::{blade::Blade, metric::Metric};

pub struct Monom(pub Vec<(String, isize)>, pub Blade);

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

    pub fn power(mut self, rhs: Monom, metric: &Metric) -> Option<Monom> {
        assert!(rhs.0.is_empty(), "Cannot raise to a variable monom");
        let (multiplicity, blade) = self.1.power(&rhs.1, metric)?;
        for (_, m) in self.0.iter_mut() {
            *m *= multiplicity;
        }
        self.1 = blade;
        Some(self)
    }
}

impl std::fmt::Display for Monom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (name, multiplicity) in &self.0 {
            // if *multiplicity != 0 {
            write!(f, "{name}")?;
            if *multiplicity != 1 {
                write!(f, "^{multiplicity}")?;
            }
            write!(f, " ")?;
            // }
        }
        write!(f, "{})", self.1)?;
        Ok(())
    }
}
