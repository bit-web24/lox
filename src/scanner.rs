use crate::token::{token_type::TokenType, Token};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i64,
    current: i64,
    line: i64,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i64
    }

    fn scan_token(&mut self) {
        let ch: char = self.advance();
        use TokenType::*;
        let token_type = match ch {
            '(' => Some(LEFT_PAREN),
            ')' => Some(RIGHT_PAREN),
            '{' => Some(LEFT_BRACE),
            '}' => Some(RIGHT_BRACE),
            ',' => Some(COMMA),
            '.' => Some(DOT),
            '-' => Some(MINUS),
            '+' => Some(PLUS),
            ';' => Some(SEMICOLON),
            '*' => Some(STAR),
            _ => None,
        };

        if let Some(tt) = token_type {
            self.add_token(tt);
        }

        panic!("Error: Invalid Token; Line: {}", line!());
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;

        ch
    }

    fn add_token(&self, type_: TokenType) {
        self.add_token_(type_, None);
    }

    fn add_token_(&self, type_: TokenType, literal: Object) {
        if let Some(text) = self
            .source
            .get((self.start as usize)..(self.current as usize))
        {
            self.tokens
                .push(Token::new(type_, text.to_string(), literal, self.line))
        }
        panic!(
            "Error: while adding token; File: scanner.rs; Line: {}",
            line!()
        );
    }
}
