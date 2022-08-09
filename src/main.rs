use algebra::{
    basis::Basis,
    metric::{Metric, Square},
    monom::Monomial,
};
use common_macros::hash_map;

use crate::algebra::Product;

pub mod algebra;
pub mod interpret;
pub mod parse;
pub mod repl;

#[cfg(test)]
mod test;

fn main() {
    // repl::repl();

    let metric = Metric(vec![Square::Pos, Square::Pos]);
    // let dim = metric.dimension();

    let a = Monomial {
        scalar: 2.5,
        symbols: hash_map!(
            "a".to_string() => 2,
            "b".to_string() => 1,
        ),
        basis: Basis(vec![true, false]),
    };

    let b = Monomial {
        scalar: 3.0,
        symbols: hash_map!(
            "a".to_string() => 2,
            "b".to_string() => 1,
        ),
        basis: Basis(vec![false, true]),
    };

    let c = a.product(Product::Geometric, &b, &metric);

    println!("{a} {b} = {c}");
}
