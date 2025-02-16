use crate::{
    error::{error_types::ParseError, LoxError},
    expr::Expr,
    stmt::{self, Stmt},
    token::{token_type::TokenType, Token},
};
use std::os::linux::raw::stat;
use std::{borrow::Borrow, error::Error, vec};

pub struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        return expression::assignment(self);
    }

    pub fn statement(&mut self) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        if self.match_(vec![TokenType::PRINT]) {
            return statement::print(self);
        } else if self.match_(vec![TokenType::LEFT_BRACE]) {
            return Ok(Box::new(stmt::Block::new(statement::block(self)?)));
        } else if self.match_(vec![TokenType::IF]) {
            return statement::if_statement(self);
        } else if self.match_(vec![TokenType::WHILE]) {
            return statement::while_statement(self);
        } else if self.match_(vec![TokenType::FOR]) {
            return statement::for_statement(self);
        } else if self.match_(vec![TokenType::FUN]) {
            return statement::function_definition(self, "function")
                .map(|function| Box::new(function) as Box<dyn Stmt>);
        } else if self.match_(vec![TokenType::RETURN]) {
            return statement::return_statement(self);
        } else if self.match_(vec![TokenType::CLASS]) {
            return statement::class_declaration(self);
        }

        statement::expression(self)
    }

    pub fn declaration(&mut self) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        if self.match_(vec![TokenType::VAR]) {
            return statement::var_declaration(self);
        }

        self.statement()
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Stmt>>, Box<dyn Error>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn match_(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance();
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

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().type_ == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return self.tokens[self.current as usize].borrow();
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current as usize - 1).unwrap().clone()
    }

    fn consume(&mut self, type_: TokenType, message: &str) -> Result<Token, Box<dyn Error>> {
        if self.check(type_) {
            self.advance();
            return Ok(self.previous());
        }
        Err(self.error(self.peek(), message))
    }

    fn _synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().type_ == TokenType::SEMICOLON {
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
                    self.advance();
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
    use crate::expr::{self, Expr, Get};
    use crate::object::Object;
    use crate::token::{token_type::TokenType, Token};
    use std::error::Error;

    pub fn assignment(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let exp = or(parser)?;

        if parser.match_(vec![TokenType::EQUAL]) {
            let equals: Token = parser.previous();
            let value: Box<dyn Expr> = assignment(parser)?;

            if let Some(expr::Variable { name }) = exp.as_any().downcast_ref::<expr::Variable>() {
                return Ok(Box::new(expr::Assign::new(name.clone(), value)));
            } else if let Some(get_) = exp.as_any().downcast_ref::<expr::Get>() {
                return Ok(Box::new(expr::Set::new(
                    get_.object.clone(),
                    get_.name.clone(),
                    value,
                )));
            }

            return Err(parser.error(&equals, "Invalid assignment target."));
        }

        Ok(exp)
    }

    pub fn equality(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr> = comparison(parser)?;

        while parser.match_(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = comparison(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }

        Ok(expression)
    }

    fn comparison(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        use TokenType::*;

        let mut expression: Box<dyn Expr> = term(parser)?;
        while parser.match_(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = term(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn term(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr> = factor(parser)?;

        while parser.match_(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = factor(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn factor(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr> = unary(parser)?;

        while parser.match_(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = unary(parser)?;
            expression = Box::new(expr::Binary::new(expression, operator, right));
        }
        return Ok(expression);
    }

    fn unary(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        if parser.match_(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = unary(parser)?;
            return Ok(Box::new(expr::Unary::new(operator, right)));
        }

        return call(parser);
    }

    fn primary(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        if parser.match_(vec![TokenType::FALSE]) {
            return Ok(Box::new(expr::Literal::new(Object::Boolean(false))));
        }

        if parser.match_(vec![TokenType::TRUE]) {
            return Ok(Box::new(expr::Literal::new(Object::Boolean(true))));
        }

        if parser.match_(vec![TokenType::NIL]) {
            return Ok(Box::new(expr::Literal::new(Object::Nil)));
        }

        if parser.match_(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Box::new(expr::Literal::new(
                *parser.previous().literal.unwrap(),
            )));
        }

        if parser.match_(vec![TokenType::IDENTIFIER]) {
            return Ok(Box::new(expr::Variable::new(parser.previous())));
        }

        if parser.match_(vec![TokenType::LEFT_PAREN]) {
            let expression: Box<dyn Expr> = parser.expression()?;
            parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Box::new(expr::Grouping::new(expression)));
        }

        panic!("Expected expression.");
    }

    fn or(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr> = and(parser)?;

        while parser.match_(vec![TokenType::OR]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = and(parser)?;
            expression = Box::new(expr::Logical::new(expression, operator, right));
        }

        Ok(expression)
    }

    fn and(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expression: Box<dyn Expr> = equality(parser)?;

        while parser.match_(vec![TokenType::AND]) {
            let operator: Token = parser.previous();
            let right: Box<dyn Expr> = equality(parser)?;
            expression = Box::new(expr::Logical::new(expression, operator, right));
        }

        Ok(expression)
    }

    fn call(parser: &mut Parser) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut expr: Box<dyn Expr> = primary(parser)?;

        loop {
            if parser.match_(vec![TokenType::LEFT_PAREN]) {
                expr = finish_call(parser, expr)?;
            } else if parser.match_(vec![TokenType::DOT]) {
                let name: Token =
                    parser.consume(TokenType::IDENTIFIER, "Expect property name after '.'.")?;
                expr = Box::new(Get::new(expr, name));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(
        parser: &mut Parser,
        callee: Box<dyn Expr>,
    ) -> Result<Box<dyn Expr>, Box<dyn Error>> {
        let mut arguments: Vec<Box<dyn Expr>> = Vec::new();

        if !parser.check(TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    parser.error(parser.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(parser.expression()?);

                if !parser.match_(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = parser.consume(TokenType::RIGHT_PAREN, "Expected ')' after arguments.")?;

        Ok(Box::new(expr::Call::new(callee, paren, arguments)))
    }
}

mod statement {
    use super::{statement, Parser};
    use crate::expr::{self, Expr};
    use crate::stmt::{self, Stmt};
    use crate::token::token_type::TokenType;
    use crate::token::Token;
    use std::cell::RefCell;
    use std::error::Error;
    use std::rc::Rc;

    pub fn print(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        let value: Box<dyn Expr> = parser.expression()?;
        parser.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;

        Ok(Box::new(stmt::Print::new(value)))
    }

    pub fn expression(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        let expr: Box<dyn Expr> = parser.expression()?;
        parser.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Box::new(stmt::Expression::new(expr)))
    }

    pub fn var_declaration(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        let name: Token = parser
            .consume(TokenType::IDENTIFIER, "Expect variable name.")?
            .to_owned();

        let mut initializer: Option<Box<dyn Expr>> = None;
        if parser.match_(vec![TokenType::EQUAL]) {
            initializer = Some(parser.expression()?);
        }

        parser.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Box::new(stmt::Var::new(name, initializer)))
    }

    pub fn block(
        parser: &mut Parser,
    ) -> Result<Vec<Rc<RefCell<Box<dyn stmt::Stmt>>>>, Box<dyn Error>> {
        let mut statements: Vec<Rc<RefCell<Box<dyn stmt::Stmt>>>> = Vec::new();

        while !parser.check(TokenType::RIGHT_BRACE) && !parser.is_at_end() {
            statements.push(Rc::new(RefCell::new(parser.declaration()?)));
        }

        parser.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    pub fn if_statement(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        parser.consume(TokenType::LEFT_PAREN, "Expect '(' after if.")?;
        let condition: Box<dyn Expr> = parser.expression()?;
        parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;
        let then_branch: Box<dyn Stmt> = parser.statement()?;
        let else_branch: Option<Box<dyn Stmt>> = if parser.match_(vec![TokenType::ELSE]) {
            Some(parser.statement()?)
        } else {
            None
        };
        Ok(Box::new(stmt::If::new(condition, then_branch, else_branch)))
    }

    pub fn while_statement(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        parser.consume(TokenType::LEFT_PAREN, "Expect '(' after if.")?;
        let condition: Box<dyn Expr> = parser.expression()?;
        parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;
        let body: Box<dyn Stmt> = parser.statement()?;

        Ok(Box::new(stmt::While::new(condition, body)))
    }

    pub fn for_statement(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        parser.consume(TokenType::LEFT_PAREN, "Expect '(' after for.")?;

        let initializer: Option<Box<dyn Stmt>> = if parser.match_(vec![TokenType::SEMICOLON]) {
            None
        } else if parser.match_(vec![TokenType::VAR]) {
            Some(self::var_declaration(parser)?)
        } else {
            Some(self::expression(parser)?)
        };

        let mut condition: Option<Box<dyn Expr>> = if !parser.match_(vec![TokenType::SEMICOLON]) {
            Some(parser.expression()?)
        } else {
            None
        };
        parser.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment: Option<Box<dyn Expr>> = if !parser.match_(vec![TokenType::RIGHT_PAREN]) {
            Some(parser.expression()?)
        } else {
            None
        };
        parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

        let mut body: Box<dyn Stmt> = parser.statement()?;

        if let Some(increment) = increment {
            body = Box::new(stmt::Block::new(vec![
                Rc::new(RefCell::new(body)),
                Rc::new(RefCell::new(Box::new(stmt::Expression::new(increment)))),
            ]));
        }

        if condition.is_none() {
            condition = Some(Box::new(expr::Literal::new(
                crate::object::Object::Boolean(true),
            )));
        };

        body = Box::new(stmt::While::new(condition.unwrap(), body));

        if let Some(initializer) = initializer {
            body = Box::new(stmt::Block::new(vec![
                Rc::new(RefCell::new(initializer)),
                Rc::new(RefCell::new(body)),
            ]));
        }

        Ok(body)
    }

    pub fn function_definition(
        parser: &mut Parser,
        kind: &str,
    ) -> Result<stmt::Function, Box<dyn Error>> {
        let name: Token = parser
            .consume(
                TokenType::IDENTIFIER,
                format!("Expect {} name.", kind).as_str(),
            )?
            .to_owned();
        parser.consume(TokenType::LEFT_PAREN, "Expect '(' after function name.")?;
        let mut parameters: Vec<Token> = Vec::new();
        if !parser.check(TokenType::RIGHT_PAREN) {
            loop {
                if parameters.len() >= 255 {
                    parser.error(&parser.previous(), "Cannot have more than 255 parameters.");
                }
                parameters.push(parser.consume(TokenType::IDENTIFIER, "Expect parameter name.")?);
                if !parser.match_(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }
        parser.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.")?;
        parser.consume(
            TokenType::LEFT_BRACE,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;
        let body = block(parser)?;

        Ok(stmt::Function::new(name, parameters, body))
    }

    pub fn return_statement(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        let keyword: Token = parser.previous();
        let mut value: Option<Box<dyn Expr>> = None;
        if !parser.check(TokenType::SEMICOLON) {
            value = Some(parser.expression()?);
        }
        parser.consume(TokenType::SEMICOLON, "Expect ';' after return value.")?;
        Ok(Box::new(stmt::Return::new(keyword, value)))
    }

    pub fn class_declaration(parser: &mut Parser) -> Result<Box<dyn Stmt>, Box<dyn Error>> {
        let class_name: Token = parser.consume(TokenType::IDENTIFIER, "Expect class name.")?;
        parser.consume(TokenType::LEFT_BRACE, "Expect '{' before class body.")?;

        let mut methods: Vec<stmt::Function> = Vec::new();
        while !parser.check(TokenType::RIGHT_BRACE) && !parser.is_at_end() {
            let method = function_definition(parser, "method")?;
            methods.push(method);
        }

        parser.consume(TokenType::RIGHT_BRACE, "Expect '}' after class body.")?;

        Ok(Box::new(stmt::Class::new(class_name, None, methods)))
    }
}
