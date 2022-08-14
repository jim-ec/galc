use std::io::stdin;

use structopt::StructOpt;

use crate::{
    algebra::metric::{self, Metric},
    interpret::eval,
    parse,
};

#[derive(StructOpt, Debug)]
#[structopt()]
struct Options {
    /// Expression to evaluate. Otherwise enter interactive mode
    #[structopt()]
    expression: Option<String>,

    /// Number of dimensions with metric 1.
    #[structopt(short, long)]
    positive: Option<usize>,

    /// Number of dimensions with metric -1.
    #[structopt(short, long)]
    negative: Option<usize>,

    /// Number of dimensions with metric 0.
    #[structopt(short, long)]
    zero: Option<usize>,

    /// Use metric of complex numbers.
    #[structopt(long)]
    complex: bool,

    /// Use metric of hyperbolic numbers.
    #[structopt(long)]
    hyperbolic: bool,

    /// Use metric of dual numbers.
    #[structopt(long)]
    dual: bool,

    /// Plane-based geometric algebra.
    #[structopt(long)]
    pga: Option<usize>,
}

fn eval(input: &str, metric: &Metric) {
    let expr = match parse::parse(input) {
        Some(expr) => expr,
        None => {
            println!();
            return;
        }
    };

    match eval::eval(expr, &metric) {
        Ok(result) => {
            println!("  = {}", result.merge_monomials());
        }
        Err(eval::Undefined(cause)) => {
            println!("  {}", cause);
            println!("  = _|_");
        }
    };
}

pub fn repl() {
    let options = Options::from_args();

    let mut metric = metric::Metric(vec![]);
    if options.hyperbolic {
        metric.0 = vec![metric::Square::Pos];
    } else if options.complex {
        metric.0 = vec![metric::Square::Neg];
    } else if options.dual {
        metric.0 = vec![metric::Square::Zero];
    } else if let Some(n) = options.pga {
        metric.0 = vec![metric::Square::Pos; n];
        metric.0.push(metric::Square::Zero);
    } else {
        if let Some(p) = options.positive {
            metric
                .0
                .extend(std::iter::repeat(metric::Square::Pos).take(p));
        }
        if let Some(q) = options.negative {
            metric
                .0
                .extend(std::iter::repeat(metric::Square::Neg).take(q));
        }
        if let Some(r) = options.zero {
            metric
                .0
                .extend(std::iter::repeat(metric::Square::Zero).take(r));
        }
    }

    if metric.0.len() > 9 {
        println!("Only dimensions up to 9 are supported due to notational constraints");
        return;
    }

    for (i, &square) in metric.0.iter().enumerate() {
        println!(
            "e{i}^2 = {}",
            match square {
                metric::Square::Pos => "1",
                metric::Square::Neg => "-1",
                metric::Square::Zero => "0",
            }
        )
    }

    if let Some(expression) = options.expression {
        eval(&expression, &metric);
        return;
    }

    loop {
        println!();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed read from stdin");
        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input.starts_with(":") {
            match &input[1..] {
                "q" => return,
                "h" => {
                    println!("Commands");
                    println!("--------");
                    println!("Quit   :q");
                    println!("Help   :h");
                    println!();
                    println!("Expressions");
                    println!("-----------");
                    println!(r"Geometric product:  a b");
                    println!(r"Exteriour product:  a /\ b");
                    println!(r"Regressive product: a \/ b");
                    println!(r"Left contraction:   a -| b");
                    println!(r"Right contraction:  a |- b");
                    println!(r"Inner product:      a | b");
                    println!(r"Scalar product:     a * b");
                    println!(r"Division:           a / b");
                    println!(r"Power:              a^b");
                    println!(r"Negation:           -a");
                    println!(r"Dualization:        !a");
                    println!(r"Reversal:           ~a");
                    println!(r"Norm:               [a]");
                }
                _ => {
                    println!("Unknown command. Use :h to see a help screen.");
                }
            }
        } else {
            eval(input, &metric);
        }
    }
}
