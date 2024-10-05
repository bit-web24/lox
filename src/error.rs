use crate::token::{token_type::TokenType, Token};

pub struct Error {
    error_type: Option<Box<dyn ErrorType>>,
    at_token: Option<Token>,
    message: Option<String>,
}

impl Error {
    pub fn new() -> Self {
        Self {
            error_type: None,
            at_token: None,
            message: None,
        }
    }

    pub fn type_(self, error_type: Box<dyn ErrorType>) -> Self {
        Self {
            error_type: Some(error_type),
            at_token: self.at_token,
            message: self.message,
        }
    }

    pub fn at_token(self, location: Token) -> Self {
        Self {
            at_token: Some(location),
            error_type: self.error_type,
            message: self.message,
        }
    }

    pub fn message(self, message: String) -> Self {
        Self {
            message: Some(message),
            error_type: self.error_type,
            at_token: self.at_token,
        }
    }

    pub fn report(&self) {
        if let Some(error_type) = &self.error_type {
            if let Some(token) = &self.at_token {
                error_type.report(token.clone(), self.message.clone().unwrap());
            }
        }
    }
}

pub trait ErrorType {
    fn report(&self, token: Token, message: String);
    fn write(&self, error_type: &str, line: i64, where_: &str, message: String) {
        println!(
            "{} [line {}] Error {}: {}",
            error_type, line, where_, message
        );
    }
}

#[derive(Debug)]
pub struct ParseError;

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
pub struct RuntimeError;

impl ErrorType for RuntimeError {
    fn report(&self, token: Token, message: String) {
        self.write("RuntimeError", token.line, "", message);
    }
}
