use crate::algebra::{
    basis::Basis,
    metric::{Metric, Square},
    sign::Sign,
};

#[test]
fn geometric_hyperbolic() {
    let metric = Metric(vec![Square::Pos, Square::Pos]);
    let a = Basis(vec![true, true]);
    let b = Basis(vec![true, false]);
    let (sign, basis) = a.geometric(&b, &metric).unwrap();
    assert_eq!(sign, Sign::Neg);
    assert_eq!(basis, Basis(vec![false, true]));
}

#[test]
fn geometric_elliptic() {
    let metric = Metric(vec![Square::Neg, Square::Pos]);
    let a = Basis(vec![true, true]);
    let b = Basis(vec![true, false]);
    let (sign, basis) = a.geometric(&b, &metric).unwrap();
    assert_eq!(sign, Sign::Pos);
    assert_eq!(basis, Basis(vec![false, true]));
}

#[test]
fn geometric_degenerate() {
    let metric = Metric(vec![Square::Zero, Square::Pos]);
    let a = Basis(vec![true, true]);
    let b = Basis(vec![true, false]);
    assert!(a.geometric(&b, &metric).is_none());
}
