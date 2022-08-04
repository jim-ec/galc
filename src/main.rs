use algebra::{
    basis::Basis,
    blade::Blade,
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

    let metric = Metric(vec![Square::Neg, Square::Pos]);
    let a = Monom(
        hash_map!(
            "a".to_string() => 2,
        ),
        Blade(2.0, Basis(vec![true, false])),
    );

    let b = Monom(
        hash_map![
            "a".to_string() => 1
        ],
        Blade(2.0, Basis(vec![false, false])),
    );

    println!("a = {a}");
    println!("b = {b}");
    print!("{a} / {b} = ");
    let c = a.divide(&b, &metric).unwrap();
    println!("{}", c);
}
