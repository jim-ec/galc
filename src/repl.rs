use std::io::stdin;

use crate::{algebra::metric, interpret::eval, parse};

pub fn repl() {
    let metric = metric::Metric(vec![
        metric::Square::Pos,
        metric::Square::Pos,
        metric::Square::Pos,
    ]);

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed read from stdin");
        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input == ":q" {
            println!("Quit");
            return;
        }

        let expr = match parse::parse(input) {
            Some(expr) => expr,
            None => {
                println!();
                continue;
            }
        };

        match eval::eval(expr, &metric) {
            Ok(result) => println!("  = {result}"),
            Err(eval::Undefined(cause)) => println!("{}", cause),
        };

        println!();
    }
}
