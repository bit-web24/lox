use std::{borrow::Borrow, vec};

use crate::{
    expr::{self, Expr},
    object::Object,
    token::{token_type::TokenType, Token},
};

struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        return self.equality::<T>();
    }

    fn equality<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        let mut expression: Box<dyn Expr<T>> = self.comparison::<T>();

        while self.match_::<T>(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.previous::<T>();
            let right: Box<dyn Expr<T>> = self.comparison::<T>();
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }

        expression
    }

    fn match_<T>(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance::<T>();
                return true;
            }
        }

        false
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().type_ == type_
    }

    fn advance<T>(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous::<T>();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().type_ == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return self.tokens[self.current as usize].borrow();
    }

    fn previous<T>(&self) -> Token {
        self.tokens.get(self.current as usize - 1).unwrap().clone()
    }

    fn comparison<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        use TokenType::*;

        let mut expression: Box<dyn Expr<T>> = self.term::<T>();
        while self.match_::<T>(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator: Token = self.previous::<T>();
            let right: Box<dyn Expr<T>> = self.term::<T>();
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return expression;
    }

    fn term<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        let mut expression: Box<dyn Expr<T>> = self.factor::<T>();

        while self.match_::<T>(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous::<T>();
            let right: Box<dyn Expr<T>> = self.factor::<T>();
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return expression;
    }

    fn factor<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        let mut expression: Box<dyn Expr<T>> = self.unary::<T>();

        while self.match_::<T>(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous::<T>();
            let right: Box<dyn Expr<T>> = self.unary::<T>();
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return expression;
    }

    fn unary<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        if self.match_::<T>(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous::<T>();
            let right: Box<dyn Expr<T>> = self.unary();
            return Box::new(expr::Unary::new(operator, right));
        }

        return self.primary();
    }

    fn primary<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
        if self.match_::<T>(vec![TokenType::FALSE]) {
            return Box::new(expr::Literal::new(Object::Boolean(false)));
        }

        if self.match_::<T>(vec![TokenType::TRUE]) {
            return Box::new(expr::Literal::new(Object::Boolean(true)));
        }

        if self.match_::<T>(vec![TokenType::NIL]) {
            return Box::new(expr::Literal::new(Object::Nil));
        }

        if self.match_::<T>(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Box::new(expr::Literal::new(self.previous::<T>().literal.unwrap()));
        }

        if self.match_::<T>(vec![TokenType::LEFT_PAREN]) {
            let expression: Box<dyn Expr<T>> = self.expression();
            self.consume::<T>(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Box::new(expr::Grouping::new(expression));
        }

        panic!("Expect expression.");
    }

    fn consume<T>(&mut self, type_: TokenType, message: &str) {
        if self.check(type_) {
            self.advance::<T>();
            return;
        }
        error(self.peek(), message);
    }

    fn synchronize<T>(&mut self) {
        self.advance::<T>();
        while !self.is_at_end() {
            if self.previous::<T>().type_ == TokenType::SEMICOLON {
                return;
            }
            match self.peek().type_ {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => (),
                _ => {
                    self.advance::<T>();
                }
            }
        }
    }
}

fn error(token: &Token, message: &str) {
    if token.type_ == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        report(
            token.line,
            format!(" at '{}'", token.lexeme).as_str(),
            message,
        );
    }
}

fn report(line: i64, at: &str, msg: &str) {
    panic!("[line {}] Error{}: {}", line, at, std::format!("{}", msg));
}
