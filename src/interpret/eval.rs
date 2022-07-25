use crate::algebra::{blade::Blade, metric::Metric, shape::Shape, sign::Sign};

use super::expr::Expr;

pub struct Undefined(pub String);

pub fn eval(expr: Expr, metric: &Metric) -> Result<Blade, Undefined> {
    match expr {
        Expr::Blade(factor, bases) => {
            for &basis in &bases {
                if basis >= metric.dimension() {
                    return Err(Undefined(format!(
                        "Invalid basis {basis} for algebra dimension {}",
                        metric.dimension()
                    )));
                }
            }

            if let Some((sign, shape)) = bases
                .into_iter()
                .map(|basis| {
                    let mut shape = Shape::one(metric.dimension());
                    shape.0[basis] = true;
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
        Expr::Geometric(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.geometric(&rhs, metric))
        }
        Expr::Exterior(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.exterior(&rhs, metric))
        }
        Expr::Regressive(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.regressive(&rhs, metric))
        }
        Expr::LeftContraction(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.left_contraction(&rhs, metric))
        }
        Expr::RightContraction(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.right_contraction(&rhs, metric))
        }
        Expr::Inner(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.inner(&rhs, metric))
        }
        Expr::Scalar(lhs, rhs) => {
            let lhs = eval(*lhs, metric)?;
            let rhs = eval(*rhs, metric)?;
            Ok(lhs.scalar(&rhs, metric))
        }
        Expr::Dual(x) => Ok(eval(*x, metric)?.dual()),
        Expr::Reverse(x) => Ok(eval(*x, metric)?.reverse()),
        Expr::Inverse(x) => eval(*x, metric)?
            .inverse(metric)
            .ok_or(Undefined(format!("Inverse of 0-blade"))),
        Expr::Involute(x) => Ok(eval(*x, metric)?.involute()),
        Expr::Conjugate(x) => Ok(eval(*x, metric)?.conjugate()),
        Expr::Grade(x) => Ok(Blade::from(
            eval(*x, metric)?.grade() as f64,
            metric.dimension(),
        )),
        Expr::AntiGrade(x) => Ok(Blade::from(
            eval(*x, metric)?.anti_grade() as f64,
            metric.dimension(),
        )),
    }
}
