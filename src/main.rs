#[macro_use]
extern crate lalrpop_util;

use rand::prelude::*;
use std::io::{self, BufRead, Stdout, Write};

mod ast;
mod numbers;

const SUCCESS_RATE: f32 = 0.98;

lalrpop_mod!(pub calculator);

fn print_prompt(stdout: &mut Stdout) -> io::Result<()> {
    write!(stdout, "=> ")?;
    stdout.flush()
}

fn evaluate_and_print(rng: &mut ThreadRng, line: &str, success_rate: f32, interactive: bool) {
    let res = calculator::CommandParser::new().parse(success_rate, &line);

    match &res
        .map_err(|e| format!("{}", e))
        .and_then(|expr| expr.evaluate(rng, success_rate, 0))
    {
        Ok(res) => println!(
            "{}{:#x}",
            interactive.then(|| "<= ").unwrap_or_default(),
            res
        ),
        Err(e) => println!("{}{}", interactive.then(|| "<e ").unwrap_or_default(), e),
    };
}

fn main() -> io::Result<()> {
    let expr = std::env::args().nth(1);
    let mut rng = thread_rng();

    let success_rate = option_env!("SUCCESS_RATE")
        .map(|s| {
            s.parse::<f32>()
                .expect("Env var SUCCESS_RATE must be of type float!")
        })
        .unwrap_or(SUCCESS_RATE);

    if let Some(expr) = expr {
        evaluate_and_print(&mut rng, &expr, success_rate, false);
    } else {
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        print_prompt(&mut stdout)?;
        for line in handle.lines() {
            let line = line?.trim().to_string();

            if line != "" {
                evaluate_and_print(&mut rng, &line, success_rate, true);
            }

            print_prompt(&mut stdout)?;
        }
    }

    Ok(())
}
