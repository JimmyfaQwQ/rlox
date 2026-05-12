use std::io;
use std::fs;
use std::io::Write;

use layla_log::*;

mod scanner;
mod token;
mod error;
mod expr;
mod stmt;
mod parser;
mod interpreter;
mod enviorment;
mod callable;
mod object;


fn run_file(path: &str) {
    let mut env = enviorment::Enviorment::new(None);
    let contents = fs::read_to_string(path)
        .expect("Something went wrong loading the script");
    if let Err(e) = run(&contents, &mut env) {
        std::process::exit(e.exit_code());
    }
}

fn run_prompt() {
    let mut line = String::new();
    let mut env = enviorment::Enviorment::new(None);
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin().read_line(&mut line)
            .expect("Failed to read new line of input");
        if line.trim().is_empty() {
            continue;
        }
        if line.trim() == "exit" {
            break;
        }
        if let Err(e) = run(&line, &mut env) {
            info!("{:?}", e);
        }
        line.clear();
    }
}

fn run(source: &str, env: &mut enviorment::Enviorment) -> Result<(), error::Error> {
    let tokens = scanner::scan_tokens(source)?;
    let statements = parser::Parser::new(tokens).parse()?;
    interpreter::interpret(&statements, env)
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
