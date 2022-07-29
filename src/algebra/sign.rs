use super::blade::Blade;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Sign {
    Pos,
    Neg,
}

impl std::ops::Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        }
    }
}

impl std::ops::Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Pos, Sign::Neg) => Sign::Neg,
            (Sign::Neg, Sign::Pos) => Sign::Neg,
            (Sign::Neg, Sign::Neg) => Sign::Pos,
        }
    }
}

impl std::ops::Mul<f64> for Sign {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Sign::Pos => rhs,
            Sign::Neg => -rhs,
        }
    }
}

impl std::ops::Mul<Blade> for Sign {
    type Output = Blade;

    fn mul(self, rhs: Blade) -> Self::Output {
        match self {
            Sign::Pos => rhs,
            Sign::Neg => -rhs,
        }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sign::Pos => Ok(()),
            Sign::Neg => write!(f, "-"),
        }
    }
}
