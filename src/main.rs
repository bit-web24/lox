use std::{error::Error, io::Write, process::exit};

mod callable;
mod env;
mod error;
mod expr;
mod function;
mod interpreter;
mod object;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;

#[cfg(test)]
mod tests;

use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use token::Token;

struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn exec(&mut self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let n = args.len();
        if n < 2 || n > 2 {
            println!("Usage: lox <script>");
            exit(64);
        } else {
            let mut args = args.into_iter();
            args.next();
            self.run_file(args.next().unwrap())?;
        }

        Ok(())
    }

    fn run_file(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        self.run(contents)?;
        if self.had_error {
            exit(65);
        } else if self.had_runtime_error {
            exit(70)
        }
        Ok(())
    }

    fn run(&mut self, source: String) -> Result<(), Box<dyn Error>> {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        let mut parser_: Parser = parser::Parser::new(tokens);
        let mut statements = parser_.parse()?;

        let mut interpreter = Interpreter::new();
        let mut resolver: Resolver<'_> = Resolver::new(&mut interpreter);
        resolver.resolve(&mut statements)?;

        interpreter.interpret(statements)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lox: Lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();
    lox.exec(args)?;
    Ok(())
}
