use std::io;
use std::fs;
use std::io::Write;

use layla_log::*;
    
mod scanner;
mod token;
mod error;

use scanner::Scanner;

fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong loading the script");
    run(contents);
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin().read_line(&mut line)
            .expect("Failed to read new line of input");
        run(line.clone());
        line.clear();
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(e) => {
            info!("Scanner error: {}", e);
        }
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
