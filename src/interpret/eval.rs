use std::f64::consts::{E, PI, TAU};

use crate::algebra::{basis::Basis, blade::Blade, metric::Metric, sign::Sign};

use super::expr::{Binary, Expr, Unary};

pub struct Undefined(pub String);

pub fn eval(expr: Expr, metric: &Metric) -> Result<Blade, Undefined> {
    let new_scalar = |scalar: f64| -> Blade { Blade::from(scalar, metric.dimension()) };

    match expr {
        Expr::Number(n) => Ok(Blade(n, Basis::one(metric.dimension()))),
        Expr::Pseudoscalar => Ok(Blade(1.0, Basis::pseudoscalar(metric.dimension()))),
        Expr::Basis(vectors) => {
            for &vector in &vectors {
                if vector >= metric.dimension() {
                    return Err(Undefined(format!(
                        "Invalid basis-vector e{vector} for algebra of dimension {}",
                        metric.dimension()
                    )));
                }
            }
            if let Some((sign, basis)) = vectors
                .into_iter()
                .map(|vector| {
                    let mut basis = Basis::one(metric.dimension());
                    basis.0[vector] = true;
                    basis
                })
                .try_fold(
                    (Sign::Pos, Basis::one(metric.dimension())),
                    |(sign_a, a), b| -> Option<(Sign, Basis)> {
                        let (sign, product) = a.geometric(&b, metric)?;
                        Some((sign * sign_a, product))
                    },
                )
            {
                Ok(Blade(sign * 1.0, basis))
            } else {
                Ok(Blade::null(metric.dimension()))
            }
        }
        Expr::Binary(binary, lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(match binary {
                Binary::Geometric => lhs.geometric(&rhs, metric),
                Binary::Exterior => lhs.exterior(&rhs, metric),
                Binary::Regressive => lhs.regressive(&rhs, metric),
                Binary::LeftContraction => lhs.left_contraction(&rhs, metric),
                Binary::RightContraction => lhs.right_contraction(&rhs, metric),
                Binary::Inner => lhs.inner(&rhs, metric),
                Binary::Scalar => lhs.scalar(&rhs, metric),
                Binary::Divide => lhs
                    .divide(&rhs, metric)
                    .ok_or(Undefined(format!("Division by {rhs} not defined")))?,
                Binary::Power => lhs
                    .power(&rhs, metric)
                    .map(|(_, blade)| blade)
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
                    .inverse(metric)
                    .ok_or(Undefined(format!("Inverse not defined for {x}"))),
                Unary::Involute => Ok(x.involute()),
                Unary::Conjugate => Ok(x.conjugate()),
            }
        }
        Expr::Norm(x) => {
            let x = eval(*x, metric)?;
            let y = x.norm(metric);
            Ok(new_scalar(y))
        }
        Expr::Identifier(name) => match name.as_str() {
            "undefined" | "⊥" => Err(Undefined(format!("Undefined computation"))),
            "e" => Ok(new_scalar(E)),
            "tau" | "τ" => Ok(new_scalar(TAU)),
            "pi" | "π" => Ok(new_scalar(PI)),
            _ => Err(Undefined(format!("Unknown variable {name}"))),
        },
    }
}
