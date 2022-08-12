use std::io::stdin;

use crate::{algebra::metric, interpret::eval, parse};

pub fn repl() {
    let metric = metric::Metric(vec![
        metric::Square::Zero,
        metric::Square::Pos,
        metric::Square::Pos,
    ]);
    println!("Metric: {metric}");

    loop {
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
                    println!(r"Left contraction:   a >> b");
                    println!(r"Right contraction:  a << b");
                    println!(r"Inner product:      a | b");
                    println!(r"Scalar product:     a * b");
                    println!(r"Division:           a / b");
                    println!(r"Power:              a ^ b");
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
            let expr = match parse::parse(input) {
                Some(expr) => expr,
                None => {
                    println!();
                    continue;
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

        println!();
    }
}
