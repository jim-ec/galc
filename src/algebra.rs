pub mod basis;
pub mod metric;
pub mod monom;
pub mod polynom;
pub mod sign;

pub enum Product {
    Geometric,
    Exterior,
    Regressive,
    LeftContraction,
    RightContraction,
    Inner,
    Scalar,
}
