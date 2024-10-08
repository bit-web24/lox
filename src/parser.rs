use std::{borrow::Borrow, error::Error, fmt::Debug, vec};

use crate::{
    error::{self, error_types::ParseError, LoxError},
    expr::{self, Expr},
    object::Object,
    stmt::{self, Stmt},
    token::{token_type::TokenType, Token},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression<T: 'static + Debug>(&mut self) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        return expression::equality::<T>(self);
    }

    pub fn statement<T: 'static + Debug>(&mut self) -> Result<Box<dyn Stmt<T>>, Box<dyn Error>> {
        if self.match_::<T>(vec![TokenType::PRINT]) {
            return statement::print(self);
        }

        statement::expression(self)
    }

    pub fn declaration<T: 'static + Debug>(&mut self) -> Result<Box<dyn Stmt<T>>, Box<dyn Error>> {
        if self.match_::<T>(vec![TokenType::VAR]) {
            return statement::var_declaration(self);
        }

        self.statement()
    }

    pub fn parse<T: 'static + Debug>(&mut self) -> Result<Vec<Box<dyn Stmt<T>>>, Box<dyn Error>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
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

    fn consume<T>(&mut self, type_: TokenType, message: &str) -> Result<&Token, Box<dyn Error>> {
        if self.check(type_) {
            self.advance::<T>();
            return Ok(self.peek());
        }
        Err(self.error(self.peek(), message))
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

    fn error(&self, token: &Token, message: &str) -> Box<dyn Error> {
        let mut err = LoxError::new();
        err = err
            .type_(Box::new(ParseError))
            .at_token(token.to_owned())
            .message(message.to_string());
        Box::new(err)
    }
}

mod expression {
    use super::Parser;
    use crate::expr::{self, Expr};
    use crate::object::Object;
    use crate::token::{token_type::TokenType, Token};
    use std::error::Error;
    use std::fmt::Debug;

    pub fn equality<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr<T>> = comparison::<T>(parser)?;

        while parser.match_::<T>(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = parser.previous::<T>();
            let right: Box<dyn Expr<T>> = comparison::<T>(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }

        Ok(expression)
    }

    fn comparison<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        use TokenType::*;

        let mut expression: Box<dyn Expr<T>> = term::<T>(parser)?;
        while parser.match_::<T>(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator: Token = parser.previous::<T>();
            let right: Box<dyn Expr<T>> = term::<T>(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn term<T: 'static + Debug>(parser: &mut Parser) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr<T>> = factor::<T>(parser)?;

        while parser.match_::<T>(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = parser.previous::<T>();
            let right: Box<dyn Expr<T>> = factor::<T>(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn factor<T: 'static + Debug>(parser: &mut Parser) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr<T>> = unary::<T>(parser)?;

        while parser.match_::<T>(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = parser.previous::<T>();
            let right: Box<dyn Expr<T>> = unary::<T>(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn unary<T: 'static + Debug>(parser: &mut Parser) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        if parser.match_::<T>(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = parser.previous::<T>();
            let right: Box<dyn Expr<T>> = unary(parser)?;
            return Ok(Box::new(expr::Unary::new(operator, right)));
        }

        return primary(parser);
    }

    fn primary<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Expr<T>>, Box<dyn Error>> {
        if parser.match_::<T>(vec![TokenType::FALSE]) {
            return Ok(Box::new(expr::Literal::new(Object::Boolean(false))));
        }

        if parser.match_::<T>(vec![TokenType::TRUE]) {
            return Ok(Box::new(expr::Literal::new(Object::Boolean(true))));
        }

        if parser.match_::<T>(vec![TokenType::NIL]) {
            return Ok(Box::new(expr::Literal::new(Object::Nil)));
        }

        if parser.match_::<T>(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Box::new(expr::Literal::new(
                parser.previous::<T>().literal.unwrap(),
            )));
        }

        if parser.match_::<T>(vec![TokenType::IDENTIFIER]) {
            return Ok(Box::new(expr::Variable::new(parser.previous::<T>())));
        }

        if parser.match_::<T>(vec![TokenType::LEFT_PAREN]) {
            let expression: Box<dyn Expr<T>> = parser.expression()?;
            parser.consume::<T>(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Box::new(expr::Grouping::new(expression)));
        }

        panic!("Expected expression.");
    }
}

mod statement {
    use super::Parser;
    use crate::expr::Expr;
    use crate::stmt::{self, Stmt};
    use crate::token::token_type::TokenType;
    use crate::token::Token;
    use std::error::Error;
    use std::fmt::Debug;

    pub fn print<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Stmt<T>>, Box<dyn Error>> {
        let value: Box<dyn Expr<T>> = parser.expression()?;
        parser.consume::<T>(TokenType::SEMICOLON, "Expect ';' after value.")?;

        Ok(Box::new(stmt::Print::new(value)))
    }

    pub fn expression<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Stmt<T>>, Box<dyn Error>> {
        let expr: Box<dyn Expr<T>> = parser.expression()?;
        parser.consume::<T>(TokenType::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Box::new(stmt::Expression::new(expr)))
    }

    pub fn var_declaration<T: 'static + Debug>(
        parser: &mut Parser,
    ) -> Result<Box<dyn Stmt<T>>, Box<dyn Error>> {
        let name: Token = parser
            .consume::<T>(TokenType::IDENTIFIER, "Expect variable name.")?
            .to_owned();

        let mut initializer: Option<Box<dyn Expr<T>>> = None;
        if parser.match_::<T>(vec![TokenType::EQUAL]) {
            initializer = Some(parser.expression()?);
        }

        parser.consume::<T>(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Box::new(stmt::Var::new(name, initializer)))
    }
}
