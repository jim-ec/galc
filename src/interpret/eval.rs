use crate::algebra::{blade::Blade, metric::Metric, shape::Shape, sign::Sign};

use super::expr::Expr;

fn bases_to_shape(bases: Vec<usize>, metric: &Metric) -> Option<(Sign, Shape)> {
    bases
        .into_iter()
        .map(|basis| {
            assert!(
                basis < metric.dimension(),
                "Invalid basis {basis} for algebra dimension {}",
                metric.dimension()
            );
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
}

pub fn eval(expr: Expr, metric: &Metric) -> Blade {
    match expr {
        Expr::Blade(factor, bases) => {
            if let Some((sign, shape)) = bases_to_shape(bases, metric) {
                Blade(sign * factor, shape)
            } else {
                Blade::null(metric.dimension())
            }
        }
        Expr::Geometric(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.geometric(&rhs, metric)
        }
        Expr::Exterior(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.exterior(&rhs, metric)
        }
        Expr::Regressive(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.regressive(&rhs, metric)
        }
        Expr::LeftContraction(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.left_contraction(&rhs, metric)
        }
        Expr::RightContraction(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.right_contraction(&rhs, metric)
        }
        Expr::Inner(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.inner(&rhs, metric)
        }
        Expr::Scalar(lhs, rhs) => {
            let lhs = eval(*lhs, metric);
            let rhs = eval(*rhs, metric);
            lhs.scalar(&rhs, metric)
        }
    }
}
