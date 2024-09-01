pub mod token_type;
use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: i64,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Object>, line: i64) -> Self {
        Self {
            type_,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}
