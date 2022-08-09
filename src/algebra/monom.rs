use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Monom(pub HashMap<String, isize>);

impl Monom {
    pub fn product(&self, rhs: &Monom) -> Monom {
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult += mult_rhs;
            } else {
                mults.insert(name.clone(), mult_rhs);
            }
        }
        Monom(mults)
    }

    pub fn divide(&self, rhs: &Monom) -> Monom {
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult -= mult_rhs;
            } else {
                mults.insert(name.clone(), -mult_rhs);
            }
        }
        Monom(mults)
    }

    pub fn power(&self, exponent: isize) -> Monom {
        let mut mults = self.0.clone();
        for (_, mult) in mults.iter_mut() {
            *mult *= exponent;
        }
        Monom(mults)
    }
}
