#![allow(dead_code)]

use crate::{
    algebra::blade::Blade,
    algebra::{
        metric::{Metric, Square},
        shape::Shape,
    },
};

pub mod algebra;
pub mod common;

fn main() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);
    let a = Blade(2.0, Shape(vec![true, true]));
    let b = Blade(3.0, Shape(vec![true, false]));
    let c = a.geometric(&b, &metric);

    println!("a = {a}");
    println!("b = {b}");
    println!("b = {c}");
}
