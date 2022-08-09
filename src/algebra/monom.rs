use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Monomial(pub HashMap<String, isize>);

impl Monomial {
    pub fn product(&self, rhs: &Monomial) -> Monomial {
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult += mult_rhs;
            } else {
                mults.insert(name.clone(), mult_rhs);
            }
        }
        Monomial(mults)
    }

    pub fn divide(&self, rhs: &Monomial) -> Monomial {
        let mut mults = self.0.clone();
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = mults.get_mut(name) {
                *mult -= mult_rhs;
            } else {
                mults.insert(name.clone(), -mult_rhs);
            }
        }
        Monomial(mults)
    }

    pub fn power(&self, exponent: isize) -> Monomial {
        let mut mults = self.0.clone();
        for (_, mult) in mults.iter_mut() {
            *mult *= exponent;
        }
        Monomial(mults)
    }
}
