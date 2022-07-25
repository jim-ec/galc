#![allow(dead_code)]

use interpret::{
    eval::{eval, Undefined},
    expr::Expr,
};

use crate::algebra::metric::{Metric, Square};

pub mod algebra;
pub mod interpret;

#[cfg(test)]
mod test;

fn main() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);
    let a = Expr::Blade(2.0, vec![0]);
    let b = Expr::Blade(3.0, vec![2]);
    let c = Expr::Geometric(Box::new(a), Box::new(b));
    match eval(c, &metric) {
        Ok(result) => println!("{result}"),
        Err(Undefined(cause)) => println!("undefined: {}", cause),
    };
}
