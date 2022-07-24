#![allow(dead_code)]

mod common;
mod metric;
mod shape;
mod sign;

fn main() {
    let metric = metric::Metric(vec![metric::Square::Pos, metric::Square::Pos]);
    let a = shape::Shape(vec![true, true]);
    let b = shape::Shape(vec![true, false]);
    let c = a.geometric(&b, &metric);

    println!("a = {a}");
    println!("b = {b}");

    if let Some((sign, shape)) = c {
        println!("c = {}{}", sign, shape);
    } else {
        println!("c = 0");
    }
}
