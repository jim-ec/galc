use algebra::{
    basis::Basis,
    blade::Blade,
    metric::{Metric, Square},
    monom::Monom,
};

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
        vec![("a".to_string(), 2)],
        Blade(2.0, Basis(vec![true, false])),
    );

    // let b = Monom(vec![], Blade(2.0, Basis(vec![false, false])));

    // println!("a = {a}");
    // println!("b = {b}");
    print!("[{a}] = ");
    // let c = a.geometric(b, &metric);
    println!("{}", a.norm(&metric));
}
