use std::fmt::Debug;

use crate::{object::Object, token::Token};

pub trait Expr<T: Debug>: Debug {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T>;
}

pub trait Visitor<T: Debug> {
    fn visit_assign_expr(&self, expr: &Assign<T>) -> Option<T>;
    fn visit_binary_expr(&self, expr: &Binary<T>) -> Option<T>;
    fn visit_call_expr(&self, expr: &Call<T>) -> Option<T>;
    fn visit_get_expr(&self, expr: &Get<T>) -> Option<T>;
    fn visit_group_expr(&self, expr: &Grouping<T>) -> Option<T>;
    fn visit_literal_expr(&self, expr: &Literal) -> Option<T>;
    fn visit_logical_expr(&self, expr: &Logical<T>) -> Option<T>;
    fn visit_set_expr(&self, expr: &Set<T>) -> Option<T>;
    fn visit_super_expr(&self, expr: &Super) -> Option<T>;
    fn visit_this_expr(&self, expr: &This) -> Option<T>;
    fn visit_unary_expr(&self, expr: &Unary<T>) -> Option<T>;
    fn visit_variable_expr(&self, expr: &Variable) -> Option<T>;
}

#[derive(Debug)]
pub struct Assign<T: Debug> {
    name: Token,
    pub value: Box<dyn Expr<T>>,
}

impl<T: Debug> Assign<T> {
    fn new(name: Token, value: Box<dyn Expr<T>>) -> Self {
        Self { name, value }
    }
}

impl<T: Debug> Expr<T> for Assign<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_assign_expr(self)
    }
}

#[derive(Debug)]
pub struct Binary<T: Debug> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T: Debug> Binary<T> {
    pub fn new(left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<T: Debug> Expr<T> for Binary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Debug)]
pub struct Call<T: Debug> {
    callee: Box<dyn Expr<T>>,
    paren: Token,
    arguments: Vec<Box<dyn Expr<T>>>,
}

impl<T: Debug> Call<T> {
    fn new(callee: Box<dyn Expr<T>>, paren: Token, arguments: Vec<Box<dyn Expr<T>>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

impl<T: Debug> Expr<T> for Call<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_call_expr(self)
    }
}

#[derive(Debug)]
pub struct Get<T: Debug> {
    object: Box<dyn Expr<T>>,
    name: Token,
}

impl<T: Debug> Get<T> {
    fn new(object: Box<dyn Expr<T>>, name: Token) -> Self {
        Self { object, name }
    }
}

impl<T: Debug> Expr<T> for Get<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_get_expr(self)
    }
}

#[derive(Debug)]
pub struct Grouping<T: Debug> {
    pub expression: Box<dyn Expr<T>>,
}

impl<T: Debug> Grouping<T> {
    pub fn new(expression: Box<dyn Expr<T>>) -> Self {
        Self { expression }
    }
}

impl<T: Debug> Expr<T> for Grouping<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_group_expr(self)
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl<T: Debug> Expr<T> for Literal {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Debug)]
pub struct Logical<T: Debug> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T: Debug> Logical<T> {
    fn new(left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<T: Debug> Expr<T> for Logical<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_logical_expr(self)
    }
}

#[derive(Debug)]
pub struct Set<T: Debug> {
    object: Box<dyn Expr<T>>,
    name: Token,
    value: Box<dyn Expr<T>>,
}

impl<T: Debug> Set<T> {
    fn new(object: Box<dyn Expr<T>>, name: Token, value: Box<dyn Expr<T>>) -> Self {
        Self {
            object,
            name,
            value,
        }
    }
}

impl<T: Debug> Expr<T> for Set<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_set_expr(self)
    }
}

#[derive(Debug)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}

impl<T: Debug> Expr<T> for Super {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_super_expr(self)
    }
}

#[derive(Debug)]
pub struct This {
    keyword: Token,
}

impl This {
    fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

impl<T: Debug> Expr<T> for This {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_this_expr(self)
    }
}

#[derive(Debug)]
pub struct Unary<T: Debug> {
    pub operator: Token,
    pub right: Box<dyn Expr<T>>,
}

impl<T: Debug> Unary<T> {
    pub fn new(operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self { operator, right }
    }
}

impl<T: Debug> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_unary_expr(self)
    }
}

#[derive(Debug)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl<T: Debug> Expr<T> for Variable {
    fn accept(&self, visitor: &dyn Visitor<T>) -> Option<T> {
        visitor.visit_variable_expr(self)
    }
}
