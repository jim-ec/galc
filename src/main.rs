#![allow(dead_code)]

use crate::{blade::Blade, shape::Shape};

mod blade;
mod common;
mod metric;
mod shape;
mod sign;

fn main() {
    let metric = metric::Metric(vec![metric::Square::Pos, metric::Square::Pos]);
    let a = Blade(2.0, Shape(vec![true, true]));
    let b = Blade(3.0, Shape(vec![true, false]));
    let c = a.geometric(&b, &metric);

    println!("a = {a}");
    println!("b = {b}");
    println!("b = {c}");
}
