use crate::expr::{self, Expr};
use crate::token::Token;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Stmt: Debug {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor {
    fn visit_block_stmt(&mut self, stmt: &mut Block) -> Result<(), Box<dyn Error>>;
    fn visit_class_stmt(&self, stmt: &Class) -> Result<(), Box<dyn Error>>;
    fn visit_expr_stmt(&mut self, stmt: &mut Expression) -> Result<(), Box<dyn Error>>;
    fn visit_func_stmt(&self, stmt: &Function) -> Result<(), Box<dyn Error>>;
    fn visit_if_stmt(&mut self, stmt: &mut If) -> Result<(), Box<dyn Error>>;
    fn visit_print_stmt(&mut self, stmt: &mut Print) -> Result<(), Box<dyn Error>>;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<(), Box<dyn Error>>;
    fn visit_var_stmt(&mut self, stmt: &mut Var) -> Result<(), Box<dyn Error>>;
    fn visit_while_stmt(&mut self, stmt: &While) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Rc<RefCell<Box<dyn Stmt>>>>,
}

impl Block {
    pub fn new(statements: Vec<Rc<RefCell<Box<dyn Stmt>>>>) -> Self {
        Self {
            statements: statements,
        }
    }
}

impl Stmt for Block {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_block_stmt(self);
    }
}

#[derive(Debug)]
pub struct Class {
    name: Token,
    superclass: expr::Variable,
    methods: Vec<Function>,
}

impl Class {
    fn new(name: Token, superclass: expr::Variable, methods: Vec<Function>) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }
}

impl Stmt for Class {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_class_stmt(self);
    }
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Rc<RefCell<Box<dyn expr::Expr>>>,
}

impl Expression {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self {
            expression: Rc::new(RefCell::new(expression)),
        }
    }
}

impl Stmt for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_expr_stmt(self);
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Rc<RefCell<Box<dyn Stmt>>>>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Rc<RefCell<Box<dyn Stmt>>>>) -> Self {
        Self { name, params, body }
    }
}

impl Stmt for Function {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_func_stmt(self);
    }
}

#[derive(Debug)]
pub struct If {
    pub condition: Rc<RefCell<Box<dyn expr::Expr>>>,
    pub then_branch: Rc<RefCell<Box<dyn Stmt>>>,
    pub else_branch: Option<Rc<RefCell<Box<dyn Stmt>>>>,
}

impl If {
    pub fn new(
        condition: Box<dyn expr::Expr>,
        then_branch: Box<dyn Stmt>,
        else_branch: Option<Box<dyn Stmt>>,
    ) -> Self {
        Self {
            condition: Rc::new(RefCell::new(condition)),
            then_branch: Rc::new(RefCell::new(then_branch)),
            else_branch: else_branch.map(|stmt| Rc::new(RefCell::new(stmt))),
        }
    }
}

impl Stmt for If {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_if_stmt(self);
    }
}

#[derive(Debug)]
pub struct Print {
    pub expression: Rc<RefCell<Box<dyn expr::Expr>>>,
}

impl Print {
    pub fn new(expression: Box<dyn expr::Expr>) -> Self {
        Self {
            expression: Rc::new(RefCell::new(expression)),
        }
    }
}

impl Stmt for Print {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_print_stmt(self);
    }
}

#[derive(Debug)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Rc<RefCell<Box<dyn expr::Expr>>>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Box<dyn Expr>>) -> Self {
        Self {
            keyword,
            value: value.map(|expr| Rc::new(RefCell::new(expr))),
        }
    }
}

impl Stmt for Return {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_return_stmt(self);
    }
}

#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Rc<RefCell<Box<dyn expr::Expr>>>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Box<dyn Expr>>) -> Self {
        Self {
            name,
            initializer: initializer.map(|init| Rc::new(RefCell::new(init))),
        }
    }
}

impl Stmt for Var {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_var_stmt(self);
    }
}

#[derive(Debug)]
pub struct While {
    pub condition: Rc<RefCell<Box<dyn expr::Expr>>>,
    pub body: Rc<RefCell<Box<dyn Stmt>>>,
}

impl While {
    pub fn new(condition: Box<dyn Expr>, body: Box<dyn Stmt>) -> Self {
        Self {
            condition: Rc::new(RefCell::new(condition)),
            body: Rc::new(RefCell::new(body)),
        }
    }
}

impl Stmt for While {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        return visitor.visit_while_stmt(self);
    }
}
