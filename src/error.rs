use crate::token::{token_type::TokenType, Token};

struct Error {
    error_type: Option<Box<dyn ErrorType>>,
    at_token: Option<Token>,
    message: Option<String>,
}

impl Error {
    fn new() -> Self {
        Self {
            error_type: None,
            at_token: None,
            message: None,
        }
    }

    fn type_(&mut self, error_type: Box<dyn ErrorType>) {
        self.error_type = Some(error_type);
    }

    fn at_token(&mut self, location: Token) {
        self.at_token = Some(location);
    }

    fn message(&mut self, message: String) {
        self.message = Some(message);
    }

    fn report(&self) {
        if let Some(error_type) = &self.error_type {
            if let Some(token) = &self.at_token {
                error_type.report(token.clone(), self.message.clone().unwrap());
            }
        }
    }
}

trait ErrorType {
    fn report(&self, token: Token, message: String);
    fn write(&self, error_type: &str, line: i64, where_: &str, message: String) {
        println!(
            "{} [line {}] Error {}: {}",
            error_type, line, where_, message
        );
    }
}

#[derive(Debug)]
struct ParseError;

impl ErrorType for ParseError {
    fn report(&self, token: Token, message: String) {
        if token.type_ == TokenType::EOF {
            self.write("ParseError", token.line, " at end", message);
        } else {
            self.write(
                "ParseError",
                token.line,
                format!(" at '{}'", token.lexeme).as_str(),
                message,
            );
        }
    }
}

#[derive(Debug)]
struct RuntimeError;

impl ErrorType for RuntimeError {
    fn report(&self, token: Token, message: String) {
        self.write("RuntimeError", token.line, "", message);
    }
}
