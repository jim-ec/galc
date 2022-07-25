#![allow(dead_code)]

use interpret::eval::{eval, Undefined};
use parse::parse;

use crate::algebra::metric::{Metric, Square};

pub mod algebra;
pub mod interpret;
pub mod parse;

#[cfg(test)]
mod test;

fn main() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);

    let string = "1 2 e i";

    let expr = if let Some(expr) = parse(string) {
        expr
    } else {
        return;
    };

    match eval(expr, &metric) {
        Ok(result) => println!("{result}"),
        Err(Undefined(cause)) => println!("undefined: {}", cause),
    };
}
