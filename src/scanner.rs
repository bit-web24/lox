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
            self.scan_token();
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
        let token_type: Result<Option<TokenType>, Option<()>> = match ch {
            '(' => Ok(Some(LEFT_PAREN)),
            ')' => Ok(Some(RIGHT_PAREN)),
            '{' => Ok(Some(LEFT_BRACE)),
            '}' => Ok(Some(RIGHT_BRACE)),
            ',' => Ok(Some(COMMA)),
            '.' => Ok(Some(DOT)),
            '-' => Ok(Some(MINUS)),
            '+' => Ok(Some(PLUS)),
            ';' => Ok(Some(SEMICOLON)),
            '*' => Ok(Some(STAR)),
            '!' => {
                if self.match_('=') {
                    Ok(Some(BANG_EQUAL))
                } else {
                    Ok(Some(BANG))
                }
            }
            '=' => {
                if self.match_('=') {
                    Ok(Some(EQUAL_EQUAL))
                } else {
                    Ok(Some(EQUAL))
                }
            }
            '<' => {
                if self.match_('=') {
                    Ok(Some(LESS_EQUAL))
                } else {
                    Ok(Some(LESS))
                }
            }
            '>' => {
                if self.match_('=') {
                    Ok(Some(GREATER_EQUAL))
                } else {
                    Ok(Some(GREATER))
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(SLASH))
                }
            }
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            ' ' | '\r' | '\t' => Ok(None),
            '"' => {
                if let Err(_) = self.string() {
                    Err(None)
                } else {
                    Ok(None)
                }
            }
            dig => {
                if Self::is_digit(dig) {
                    self.number();
                    Ok(None)
                } else {
                    Err(None)
                }
            }
        };

        match token_type {
            Ok(Some(tt)) => self.add_token(tt),
            Ok(None) => {}
            Err(_) => panic!("Error: Invalid Token; Line: {}", self.line),
        }
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

    fn match_(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize) != Some(ch) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }

    fn string(&mut self) -> Result<(), ()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Line: {}; Message: Unterminated string;", self.line);
            return Err(());
        }

        self.advance();

        let value = self
            .source
            .get(((self.start + 1) as usize)..((self.current - 1) as usize))
            .unwrap()
            .to_string();
        self.add_token_(TokenType::STRING, value);
        Ok(())
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i64 {
            return '\0';
        }
        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }
}
