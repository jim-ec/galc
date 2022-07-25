use crate::algebra::{blade::Blade, metric::Metric, shape::Shape, sign::Sign};

use super::expr::{Basis, Binary, Expr, Unary};

pub struct Undefined(pub String);

pub fn eval(expr: Expr, metric: &Metric) -> Result<Blade, Undefined> {
    match expr {
        Expr::Blade(factor, bases) => match bases {
            Basis::Pseudoscalar => Ok(Blade(factor, Shape::pseudoscalar(metric.dimension()))),
            Basis::Vectors(vectors) => {
                for &vector in &vectors {
                    if vector >= metric.dimension() {
                        return Err(Undefined(format!(
                            "Invalid basis-vector {vector} for algebra of dimension {}",
                            metric.dimension()
                        )));
                    }
                }
                if let Some((sign, shape)) = vectors
                    .into_iter()
                    .map(|vector| {
                        let mut shape = Shape::one(metric.dimension());
                        shape.0[vector] = true;
                        shape
                    })
                    .try_fold(
                        (Sign::Pos, Shape::one(metric.dimension())),
                        |(sign_a, a), b| -> Option<(Sign, Shape)> {
                            let (sign, product) = a.geometric(&b, metric)?;
                            Some((sign * sign_a, product))
                        },
                    )
                {
                    Ok(Blade(sign * factor, shape))
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
                    .ok_or(Undefined(format!("Inverse of 0-blade"))),
                Unary::Involute => Ok(x.involute()),
                Unary::Conjugate => Ok(x.conjugate()),
            }
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

            match name.as_str() {
                "gr" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    Ok(Blade::from(x.grade() as f64, metric.dimension()))
                }
                "agr" => {
                    expect_arity(1)?;
                    let x = eval(arguments.next().unwrap(), metric)?;
                    Ok(Blade::from(x.anti_grade() as f64, metric.dimension()))
                }
                _ => Err(Undefined(format!("Unknown function {name}"))),
            }
        }
    }
}
