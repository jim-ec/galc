use std::f64::consts::{E, PI, TAU};

use crate::algebra::{basis::Basis, blade::Blade, metric::Metric, sign::Sign};

use super::expr::{self, Binary, Expr, Unary};

pub struct Undefined(pub String);

pub fn eval(expr: Expr, metric: &Metric) -> Result<Blade, Undefined> {
    let new_scalar = |scalar: f64| -> Blade { Blade::from(scalar, metric.dimension()) };

    match expr {
        Expr::Blade(factor, bases) => match bases {
            expr::Basis::Pseudoscalar => Ok(Blade(factor, Basis::pseudoscalar(metric.dimension()))),
            expr::Basis::Vectors(vectors) => {
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
                    Ok(Blade(sign * factor, basis))
                } else {
                    Ok(Blade::null(metric.dimension()))
                }
            }
        },
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
        Expr::Application(name, arguments) => {
            let argument_count = arguments.len();
            let mut arguments = arguments.into_iter();

            let expect_arity = |arity: usize| -> Result<(), Undefined> {
                if arity == argument_count {
                    Ok(())
                } else {
                    Err(Undefined(format!(
                        "Arity of {name} is {arity} but got {argument_count} arguments"
                    )))
                }
            };

            let expect_scalar = |blade: Blade| -> Result<f64, Undefined> {
                if blade.grade() == 0 {
                    Ok(blade.0)
                } else {
                    Err(Undefined(format!(
                        "Expected a scalar but got a {}-blade: {}",
                        blade.grade(),
                        blade
                    )))
                }
            };

            match name.as_str() {
                "gr" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let y = x.grade() as f64;
                    Ok(new_scalar(y))
                }
                "agr" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let y = x.anti_grade() as f64;
                    Ok(new_scalar(y))
                }
                "sin" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let x = expect_scalar(x)?;
                    let y = x.sin();
                    Ok(new_scalar(y))
                }
                "cos" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let x = expect_scalar(x)?;
                    let y = x.cos();
                    Ok(new_scalar(y))
                }
                "tan" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let x = expect_scalar(x)?;
                    let y = x.tan();
                    Ok(new_scalar(y))
                }
                "exp" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let x = expect_scalar(x)?;
                    let y = x.exp();
                    Ok(new_scalar(y))
                }
                "ln" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    let x = expect_scalar(x)?;
                    if x != 0.0 {
                        let y = x.ln();
                        Ok(new_scalar(y))
                    } else {
                        Err(Undefined(format!("Logarithm of null")))
                    }
                }
                _ => Err(Undefined(format!("Unknown function {name}"))),
            }
        }
        Expr::Variable(name) => match name.as_str() {
            "undefined" | "⊥" => Err(Undefined(format!("Undefined computation"))),
            "e" => Ok(new_scalar(E)),
            "tau" | "τ" => Ok(new_scalar(TAU)),
            "pi" | "π" => Ok(new_scalar(PI)),
            _ => Err(Undefined(format!("Unknown variable {name}"))),
        },
    }
}
