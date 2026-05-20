use std::io;
use std::fs;
use std::io::Write;
use std::cell::RefCell;
use std::rc::Rc;

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
mod function;


fn run_file(path: &str) {
    let env = enviorment::Enviorment::new(None);
    let contents = fs::read_to_string(path)
        .expect("Something went wrong loading the script");
    if let Err(e) = run(&contents, env) {
        std::process::exit(e.exit_code());
    }
}

fn run_prompt() {
    let mut line = String::new();
    let env = enviorment::Enviorment::new(None);
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
        if let Err(e) = run(&line, Rc::clone(&env)) {
            println!("Error: {:?}", e);
        }
        line.clear();
    }
}

fn run(source: &str, env: Rc<RefCell<enviorment::Enviorment>>) -> Result<(), error::Error> {
    let tokens = scanner::scan_tokens(source)?;
    let statements = parser::Parser::new(tokens).parse()?;
    let mut interpreter = interpreter::Interpreter { enviorment: env };
    interpreter.interpret(&statements)
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
