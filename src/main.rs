use std::{io::{Result, Write}, process::exit};

mod token;
mod scanner;
mod object;

use token::Token;
use scanner::Scanner;

struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn exec(&mut self, args: Vec<String>) -> Result<()> {
        if args.len() > 2 {
            println!("Usage: lox [script]");
            exit(64);
        } else if args.len() == 2 {
            let mut args = args.into_iter();
            args.next();
            self.run_file(args.next().unwrap())?;
        } else {
            self.run_prompt()?;
        }

        Ok(())
    }

    fn run_file(&self, path: String) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
        self.run(contents)?;
        if self.had_error {
            exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("lox> ");
            std::io::stdout().flush()?;
            let mut line = String::new();
            if std::io::stdin().read_line(&mut line)? == 0 {
                break;
            }
            self.run(line.trim().to_string())?;
            self.had_error = false;
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, where_: String, message: String) {
        println!("[line {}] Error {}: {}", line, where_, message);
        self.had_error = true;
    }
}

fn main() -> std::io::Result<()> {
    let mut lox: Lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();
    lox.exec(args)?;
    Ok(())
}
