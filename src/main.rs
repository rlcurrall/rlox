use error::RuntimeError;
use lox::Lox;

mod error;
mod lox;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => todo!("Add REPL"),
        2 => match Lox::new().run_file(&args[1]) {
            Ok(_) => std::process::exit(0),
            Err(err) => std::process::exit(handle_error(err)),
        },
        _ => {
            show_usage();
            std::process::exit(64);
        }
    }
}

fn show_usage() {
    println!("Usage: rlox [script]");
}

fn handle_error(error: RuntimeError) -> i32 {
    match error {
        RuntimeError::GeneralError(msg) => {
            eprintln!("{msg}");
            1
        }
        RuntimeError::ScanError {
            line,
            column,
            offset: _,
            message,
        } => {
            eprintln!("line {line}:{column} | Error: {message}");
            2
        }
        RuntimeError::ParseError(parse_err, _token) => {
            eprintln!("{parse_err}");
            3
        }
        RuntimeError::InvalidArgumentTarget(parse_err) => {
            eprintln!("{parse_err}");
            3
        }
    }
}
