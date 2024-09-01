use std::{io::Result, process::exit};
mod token;
use token::Token;

struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn exec(&self, args: Vec<String>) -> Result<()> {
        if args.len() > 1 {
            println!("Usage: lox [script]");
            exit(64);
        } else if args.len() == 1 {
            run_file(args[0]);
        } else {
            run_prompt();
        }

        Ok(())
    }

    fn run_file(&self, path: String) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
        run(contents);
        if self.had_error {
            exit(65);
        }
        Ok(())
    }

    fn run_prompt(&self) -> Result<()> {
        loop {
            print!("lox> ");
            let mut line = String::new();
            if std::io::stdin().read_line(&mut line)? == 0 {
                break;
            }
            run(line.trim().to_string());
            self.had_error = false;
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<()> {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    }

    fn error(&self, line: usize, message: String) {
        report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: String, message: String) {
        println!("[line {}] Error {}: {}", line, where_, message);
        self.had_error = true;
    }
}

fn main() {
    let lox: Lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();
    lox.exec(args);
}
