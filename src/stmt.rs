use crate::expr::{self, Expr};
use crate::token::Token;
use std::error::Error;
use std::fmt::Debug;

pub trait Stmt<T: Debug>: Debug {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor<T: Debug> {
    fn visit_block_stmt(&self, stmt: &Block<T>) -> Result<(), Box<dyn Error>>;
    fn visit_class_stmt(&self, stmt: &Class<T>) -> Result<(), Box<dyn Error>>;
    fn visit_expr_stmt(&self, stmt: &Expression<T>) -> Result<(), Box<dyn Error>>;
    fn visit_func_stmt(&self, stmt: &Function<T>) -> Result<(), Box<dyn Error>>;
    fn visit_if_stmt(&self, stmt: &If<T>) -> Result<(), Box<dyn Error>>;
    fn visit_print_stmt(&self, stmt: &Print<T>) -> Result<(), Box<dyn Error>>;
    fn visit_return_stmt(&self, stmt: &Return<T>) -> Result<(), Box<dyn Error>>;
    fn visit_var_stmt(&self, stmt: &Var<T>) -> Result<(), Box<dyn Error>>;
    fn visit_while_stmt(&self, stmt: &While<T>) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Block<T> {
    statements: Vec<Box<dyn Stmt<T>>>,
}

impl<T> Block<T> {
    fn new(statements: Vec<Box<dyn Stmt<T>>>) -> Self {
        Self { statements }
    }
}

impl<T: Debug> Stmt<T> for Block<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_block_stmt(self);
    }
}

#[derive(Debug)]
pub struct Class<T> {
    name: Token,
    superclass: expr::Variable,
    methods: Vec<Function<T>>,
}

impl<T> Class<T> {
    fn new(name: Token, superclass: expr::Variable, methods: Vec<Function<T>>) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }
}

impl<T: Debug> Stmt<T> for Class<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_class_stmt(self);
    }
}

#[derive(Debug)]
pub struct Expression<T> {
    pub expression: Box<dyn expr::Expr<T>>,
}

impl<T> Expression<T> {
    pub fn new(expression: Box<dyn Expr<T>>) -> Self {
        Self { expression }
    }
}

impl<T: Debug> Stmt<T> for Expression<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_expr_stmt(self);
    }
}

#[derive(Debug)]
pub struct Function<T> {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<dyn Stmt<T>>>,
}

impl<T> Function<T> {
    fn new(name: Token, params: Vec<Token>, body: Vec<Box<dyn Stmt<T>>>) -> Self {
        Self { name, params, body }
    }
}

impl<T: Debug> Stmt<T> for Function<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_func_stmt(self);
    }
}

#[derive(Debug)]
pub struct If<T> {
    condition: Box<dyn expr::Expr<T>>,
    then_branch: Box<dyn Stmt<T>>,
    else_branch: Box<dyn Stmt<T>>,
}

impl<T> If<T> {
    fn new(
        condition: Box<dyn Expr<T>>,
        then_branch: Box<dyn Stmt<T>>,
        else_branch: Box<dyn Stmt<T>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl<T: Debug> Stmt<T> for If<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_if_stmt(self);
    }
}

#[derive(Debug)]
pub struct Print<T> {
    pub expression: Box<dyn expr::Expr<T>>,
}

impl<T> Print<T> {
    pub fn new(expression: Box<dyn expr::Expr<T>>) -> Self {
        Self { expression }
    }
}

impl<T: Debug> Stmt<T> for Print<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_print_stmt(self);
    }
}

#[derive(Debug)]
pub struct Return<T> {
    keyword: Token,
    value: Box<dyn expr::Expr<T>>,
}

impl<T> Return<T> {
    fn new(keyword: Token, value: Box<dyn Expr<T>>) -> Self {
        Self { keyword, value }
    }
}

impl<T: Debug> Stmt<T> for Return<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_return_stmt(self);
    }
}

#[derive(Debug)]
pub struct Var<T> {
    name: Token,
    initializer: Box<dyn expr::Expr<T>>,
}

impl<T> Var<T> {
    fn new(name: Token, initializer: Box<dyn Expr<T>>) -> Self {
        Self { name, initializer }
    }
}

impl<T: Debug> Stmt<T> for Var<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_var_stmt(self);
    }
}

#[derive(Debug)]
pub struct While<T> {
    condition: Box<dyn expr::Expr<T>>,
    body: Box<dyn Stmt<T>>,
}

impl<T> While<T> {
    fn new(condition: Box<dyn Expr<T>>, body: Box<dyn Stmt<T>>) -> Self {
        Self { condition, body }
    }
}

impl<T: Debug> Stmt<T> for While<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_while_stmt(self);
    }
}
