#![allow(dead_code)]

use interpret::{eval::eval, expr::Expr};

use crate::algebra::metric::{Metric, Square};

pub mod algebra;
pub mod interpret;

#[cfg(test)]
mod test;

fn main() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);
    let a = Expr::Blade(2.0, vec![0]);
    let b = Expr::Blade(3.0, vec![0]);
    let c = Expr::Scalar(Box::new(a), Box::new(b));
    let result = eval(c, &metric);
    println!("{result}");
}
