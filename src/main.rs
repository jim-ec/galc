use algebra::{
    basis::Basis,
    factor::Factor,
    metric::{Metric, Square},
    monom::Monom,
};
use common_macros::hash_map;

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

    let a = Factor {
        scalar: 2.5,
        monom: Monom(hash_map!(
            "a".to_string() => 2,
            "b".to_string() => 1,
        )),
        basis: Basis(vec![true, false]),
    };

    let b = Factor {
        scalar: 3.0,
        monom: Monom(hash_map!(
            "a".to_string() => 2,
            "b".to_string() => 1,
        )),
        basis: Basis(vec![false, true]),
    };

    let c = a.geometric_product(&b, &metric);

    println!("{a} {b} = {c}");
}
