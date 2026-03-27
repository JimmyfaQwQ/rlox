use std::io;
use std::fs;
use std::io::Write;

use layla_log::*;

mod scanner;
mod token;
mod error;
mod expr;
mod parser;
mod evaluator;

use scanner::Scanner;

fn run_file(path: &str) {
        let contents = fs::read_to_string(path)
            .expect("Something went wrong loading the script");
    run(&contents);
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin().read_line(&mut line)
            .expect("Failed to read new line of input");
        run(&line);
        line.clear();
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens_result = scanner.scan_tokens();
    let tokens = match tokens_result {
        Ok(tokens) => tokens,
        Err(e) => {
            info!("Scanner error: {}", e);
            return;
        }
    };
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    let mut parser = parser::Parser::new(tokens);
    let expr_result = parser.parse();
    let expr = match expr_result {
        Ok(expr) => expr,
        Err(e) => {
            info!("Parser error: {}", e);
            return;
        }
    };
    println!("{:?}", expr);
    let eval_result = evaluator::evaluate(&expr);
    if eval_result.is_err() {
        println!("Evaluation error: {}", eval_result.clone().err().unwrap());
        info!("Evaluation error: {}", eval_result.err().unwrap());
    } else {
        println!("{:?}", eval_result.ok().unwrap());
    }
}

fn main() {
    clean_log();
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    }
    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
