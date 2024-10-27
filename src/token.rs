pub mod token_type;

use crate::object::Object;
use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<Object>>,
    pub line: i64,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Object>, line: i64) -> Self {
        Self {
            type_,
            lexeme,
            literal: literal.map(|x| Box::new(x)),
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}
