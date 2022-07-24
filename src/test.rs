use crate::algebra::{
    metric::{Metric, Square},
    shape::Shape,
    sign::Sign,
};

#[test]
fn geometric_hyperbolic() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);
    let a = Shape(vec![true, true]);
    let b = Shape(vec![true, false]);
    let (sign, shape) = a.geometric(&b, &metric).unwrap();
    assert_eq!(sign, Sign::Neg);
    assert_eq!(shape, Shape(vec![false, true]));
}

#[test]
fn geometric_elliptic() {
    let metric = Metric(vec![Square::Neg, Square::Pos]);
    let a = Shape(vec![true, true]);
    let b = Shape(vec![true, false]);
    let (sign, shape) = a.geometric(&b, &metric).unwrap();
    assert_eq!(sign, Sign::Pos);
    assert_eq!(shape, Shape(vec![false, true]));
}

#[test]
fn geometric_degenerate() {
    let metric = Metric(vec![Square::Zero, Square::Pos]);
    let a = Shape(vec![true, true]);
    let b = Shape(vec![true, false]);
    assert!(a.geometric(&b, &metric).is_none());
}
