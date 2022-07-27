#![allow(dead_code)]

use interpret::eval::{eval, Undefined};
use parse::parse;

use crate::algebra::metric::{Metric, Square};

pub mod algebra;
pub mod interpret;
pub mod parse;
pub mod repl;

#[cfg(test)]
mod test;

fn main() {
    println!("REPL ...");
    repl::repl();
    println!("main() ...");

    let metric = Metric(vec![Square::Pos, Square::Pos, Square::Pos]);

    let string = "4e12 ** -1";

    let expr = if let Some(expr) = parse(string) {
        expr
    } else {
        return;
    };

    match eval(expr, &metric) {
        Ok(result) => println!("{result}"),
        Err(Undefined(cause)) => println!("{}", cause),
    };
}
