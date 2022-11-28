use error::{Result, RuntimeError};

mod error;
mod expr;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            show_usage();
            std::process::exit(64);
        }
        2 => match run_file(&args[1]) {
            Ok(_) => std::process::exit(0),
            Err(err) => match err {
                RuntimeError::OsError(os_err) => {
                    eprintln!("{os_err}");
                    std::process::exit(1)
                }
                RuntimeError::ParseError(parse_err) => {
                    eprintln!("{parse_err}");
                    std::process::exit(2)
                }
            },
        },
        _ => {
            println!("Hello, world! {args:#?}");
            std::process::exit(64);
        }
    }
}

fn show_usage() {
    println!("Usage: rlox [script]");
}

fn run_file(file_path: &str) -> Result<()> {
    let contents = std::fs::read_to_string(file_path)?;
    run(contents)?;

    Ok(())
}

fn run(source: String) -> Result<()> {
    let scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    println!("{tokens:#?}");

    Ok(())
}
