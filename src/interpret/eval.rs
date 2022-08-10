use common_macros::hash_map;

use crate::algebra::{
    basis::Basis, metric::Metric, monom::Monomial, polynom::Polynomial, sign::Sign, Product,
};

use super::expr::{Binary, Expr, Unary};

pub struct Undefined(pub String);

pub fn eval(expr: Expr, metric: &Metric) -> Result<Polynomial, Undefined> {
    let dimension = metric.dimension();

    match expr {
        Expr::Number(n) => Ok(Monomial {
            scalar: n,
            symbols: Default::default(),
            basis: Basis::scalar(dimension),
        }
        .into()),

        Expr::Pseudoscalar => Ok(Monomial {
            scalar: 1.0,
            symbols: Default::default(),
            basis: Basis::pseudoscalar(dimension),
        }
        .into()),

        Expr::Basis(vectors) => {
            for &vector in &vectors {
                if vector >= dimension {
                    return Err(Undefined(format!(
                        "Invalid basis-vector e{vector} for algebra of dimension {}",
                        dimension
                    )));
                }
            }
            if let Some((sign, basis)) = vectors
                .into_iter()
                .map(|vector| {
                    let mut basis = Basis::scalar(dimension);
                    basis.0[vector] = true;
                    basis
                })
                .try_fold(
                    (Sign::Pos, Basis::scalar(dimension)),
                    |(sign_a, a), b| -> Option<(Sign, Basis)> {
                        let (sign, product) = a.geometric_product(&b, metric)?;
                        Some((sign * sign_a, product))
                    },
                )
            {
                Ok(Monomial {
                    scalar: sign * 1.0,
                    symbols: Default::default(),
                    basis,
                }
                .into())
            } else {
                Ok(Monomial {
                    scalar: 0.0,
                    symbols: Default::default(),
                    basis: Basis::scalar(dimension),
                }
                .into())
            }
        }

        Expr::Binary(binary, lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(match binary {
                Binary::Geometric => lhs.product(Product::Geometric, rhs, metric),
                Binary::Exterior => lhs.product(Product::Exterior, rhs, metric),
                Binary::Regressive => lhs.product(Product::Regressive, rhs, metric),
                Binary::LeftContraction => lhs.product(Product::LeftContraction, rhs, metric),
                Binary::RightContraction => lhs.product(Product::RightContraction, rhs, metric),
                Binary::Inner => lhs.product(Product::Inner, rhs, metric),
                Binary::Scalar => lhs.product(Product::Scalar, rhs, metric),
                Binary::Divide => match rhs.clone().inverse(metric) {
                    Some(rhs) => lhs.product(Product::Geometric, rhs, metric),
                    None => return Err(Undefined(format!("Division by {rhs} not defined"))),
                },
                Binary::Power => lhs
                    .clone()
                    .power(rhs.clone(), metric)
                    .ok_or(Undefined(format!("Power of {lhs} to {rhs} not defined")))?,
            })
        }

        Expr::Unary(unary, x) => {
            let x = eval(*x, metric)?;
            match unary {
                Unary::Neg => Ok(-x),
                Unary::Dual => Ok(x.dual()),
                Unary::Reverse => Ok(x.reverse()),
                Unary::Inverse => x
                    .clone()
                    .inverse(metric)
                    .ok_or(Undefined(format!("Inverse not defined for {x}"))),
                Unary::Involute => Ok(x.involute()),
                Unary::Conjugate => Ok(x.conjugate()),
            }
        }

        Expr::Norm(x) => {
            let x = eval(*x, metric)?;
            let norm = x.norm(metric);
            Ok(Monomial {
                scalar: norm,
                symbols: Default::default(),
                basis: Basis::scalar(dimension),
            }
            .into())
        }

        Expr::Unknown(name) => Ok(Monomial {
            scalar: 1.0,
            symbols: hash_map![name => 1],
            basis: Basis::scalar(dimension),
        }
        .into()),

        Expr::Bottom => Err(Undefined(format!("Undefined computation"))),
    }
}
