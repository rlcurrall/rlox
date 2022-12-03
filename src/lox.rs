use crate::{error::Result, scanner::Scanner};

pub struct Lox;

impl Lox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_file(&self, file_path: &str) -> Result<()> {
        let contents = std::fs::read_to_string(file_path)?;
        self.run(contents)?;

        Ok(())
    }

    fn run(&self, source: String) -> Result<()> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        println!("{tokens:#?}");

        Ok(())
    }
}
