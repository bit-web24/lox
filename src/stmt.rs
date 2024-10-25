use crate::expr::{self, Expr};
use crate::token::Token;
use std::cell::{Ref, RefCell};
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Stmt<T: Debug>: Debug {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor<T: Debug> {
    fn visit_block_stmt(&mut self, stmt: &Block<T>) -> Result<(), Box<dyn Error>>;
    fn visit_class_stmt(&self, stmt: &Class<T>) -> Result<(), Box<dyn Error>>;
    fn visit_expr_stmt(&mut self, stmt: &mut Expression<T>) -> Result<(), Box<dyn Error>>;
    fn visit_func_stmt(&self, stmt: &Function<T>) -> Result<(), Box<dyn Error>>;
    fn visit_if_stmt(&mut self, stmt: &mut If<T>) -> Result<(), Box<dyn Error>>;
    fn visit_print_stmt(&mut self, stmt: &mut Print<T>) -> Result<(), Box<dyn Error>>;
    fn visit_return_stmt(&self, stmt: &Return<T>) -> Result<(), Box<dyn Error>>;
    fn visit_var_stmt(&mut self, stmt: &mut Var<T>) -> Result<(), Box<dyn Error>>;
    fn visit_while_stmt(&mut self, stmt: &While<T>) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Block<T> {
    pub statements: Vec<Rc<RefCell<Box<dyn Stmt<T>>>>>,
}

impl<T> Block<T> {
    pub fn new(statements: Vec<Rc<RefCell<Box<dyn Stmt<T>>>>>) -> Self {
        Self {
            statements: statements,
        }
    }
}

impl<T: Debug> Stmt<T> for Block<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
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
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_class_stmt(self);
    }
}

#[derive(Debug)]
pub struct Expression<T> {
    pub expression: Rc<RefCell<Box<dyn expr::Expr<T>>>>,
}

impl<T> Expression<T> {
    pub fn new(expression: Box<dyn Expr<T>>) -> Self {
        Self {
            expression: Rc::new(RefCell::new(expression)),
        }
    }
}

impl<T: Debug> Stmt<T> for Expression<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_expr_stmt(self);
    }
}

#[derive(Debug)]
pub struct Function<T> {
    name: Token,
    params: Vec<Token>,
    body: Rc<RefCell<Box<dyn Stmt<T>>>>,
}

impl<T> Function<T> {
    pub fn new(name: Token, params: Vec<Token>, body: Box<dyn Stmt<T>>) -> Self {
        Self {
            name,
            params,
            body: Rc::new(RefCell::new(body)),
        }
    }
}

impl<T: Debug> Stmt<T> for Function<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_func_stmt(self);
    }
}

#[derive(Debug)]
pub struct If<T> {
    pub condition: Rc<RefCell<Box<dyn expr::Expr<T>>>>,
    pub then_branch: Rc<RefCell<Box<dyn Stmt<T>>>>,
    pub else_branch: Option<Rc<RefCell<Box<dyn Stmt<T>>>>>,
}

impl<T> If<T> {
    pub fn new(
        condition: Box<dyn expr::Expr<T>>,
        then_branch: Box<dyn Stmt<T>>,
        else_branch: Option<Box<dyn Stmt<T>>>,
    ) -> Self {
        Self {
            condition: Rc::new(RefCell::new(condition)),
            then_branch: Rc::new(RefCell::new(then_branch)),
            else_branch: else_branch.map(|stmt| Rc::new(RefCell::new(stmt))),
        }
    }
}

impl<T: Debug> Stmt<T> for If<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_if_stmt(self);
    }
}

#[derive(Debug)]
pub struct Print<T> {
    pub expression: Rc<RefCell<Box<dyn expr::Expr<T>>>>,
}

impl<T> Print<T> {
    pub fn new(expression: Box<dyn expr::Expr<T>>) -> Self {
        Self {
            expression: Rc::new(RefCell::new(expression)),
        }
    }
}

impl<T: Debug> Stmt<T> for Print<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
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
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_return_stmt(self);
    }
}

#[derive(Debug)]
pub struct Var<T> {
    pub name: Token,
    pub initializer: Option<Rc<RefCell<Box<dyn expr::Expr<T>>>>>,
}

impl<T> Var<T> {
    pub fn new(name: Token, initializer: Option<Box<dyn Expr<T>>>) -> Self {
        Self {
            name,
            initializer: initializer.map(|init| Rc::new(RefCell::new(init))),
        }
    }
}

impl<T: Debug> Stmt<T> for Var<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_var_stmt(self);
    }
}

#[derive(Debug)]
pub struct While<T> {
    pub condition: Rc<RefCell<Box<dyn expr::Expr<T>>>>,
    pub body: Rc<RefCell<Box<dyn Stmt<T>>>>,
}

impl<T> While<T> {
    pub fn new(condition: Box<dyn Expr<T>>, body: Box<dyn Stmt<T>>) -> Self {
        Self {
            condition: Rc::new(RefCell::new(condition)),
            body: Rc::new(RefCell::new(body)),
        }
    }
}

impl<T: Debug> Stmt<T> for While<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<(), Box<dyn Error>> {
        return visitor.visit_while_stmt(self);
    }
}
