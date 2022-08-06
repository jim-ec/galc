use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Monom(pub HashMap<String, isize>);

impl Monom {
    pub fn product(mut self, rhs: &Monom) -> Monom {
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = self.0.get_mut(name) {
                *mult += mult_rhs;
            } else {
                self.0.insert(name.clone(), mult_rhs);
            }
        }
        self
    }

    pub fn divide(mut self, rhs: &Monom) -> Monom {
        for (name, &mult_rhs) in &rhs.0 {
            if let Some(mult) = self.0.get_mut(name) {
                *mult -= mult_rhs;
            } else {
                self.0.insert(name.clone(), -mult_rhs);
            }
        }
        self
    }

    pub fn power(mut self, exponent: isize) -> Monom {
        for (_, mult) in self.0.iter_mut() {
            *mult *= exponent;
        }
        self
    }
}
