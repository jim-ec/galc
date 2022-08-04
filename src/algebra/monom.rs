use std::collections::HashMap;

use super::{blade::Blade, metric::Metric};

pub struct Monom(pub HashMap<String, isize>, pub Blade);

impl Monom {
    pub fn geometric(&self, rhs: &Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::geometric)
    }

    pub fn exterior(&self, rhs: &Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::exterior)
    }

    pub fn left_contraction(&self, rhs: &Monom, metric: &Metric) -> Monom {
        self.product(rhs, metric, Blade::left_contraction)
    }

    fn product<F>(&self, rhs: &Monom, metric: &Metric, f: F) -> Monom
    where
        F: FnOnce(&Blade, &Blade, &Metric) -> Blade,
    {
        // Add multiplicities
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult += mult_rhs;
            } else {
                mults.insert(name.clone(), mult_rhs);
            }
        }

        Monom(mults, f(&self.1, &rhs.1, metric))
    }

    pub fn norm(&self, metric: &Metric) -> Monom {
        Monom(
            self.0.clone(),
            Blade::from(self.1.norm(metric), metric.dimension()),
        )
    }

    pub fn power(&self, rhs: Monom, metric: &Metric) -> Option<Monom> {
        assert!(rhs.0.is_empty(), "Cannot raise to a variable monom");

        // Multiply multiplicities
        let (mult_factor, blade) = self.1.power(&rhs.1, metric)?;
        let mut mults = self.0.clone();
        for (_, mult) in mults.iter_mut() {
            *mult *= mult_factor;
        }

        Some(Monom(mults, blade))
    }

    pub fn divide(&self, rhs: &Monom, metric: &Metric) -> Option<Monom> {
        let blade = self.1.divide(&rhs.1, metric)?;

        // Subtract multiplicities
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult -= mult_rhs;
            } else {
                mults.insert(name.clone(), mult_rhs);
            }
        }

        Some(Monom(mults, blade))
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
