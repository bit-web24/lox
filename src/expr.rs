use crate::{object::Object, token::Token};
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

pub trait Expr<T: Debug>: Debug {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Visitor<T: Debug> {
    fn visit_assign_expr(&mut self, expr: &mut Assign<T>) -> Result<T, Box<dyn Error>>; // a = 30;
    fn visit_binary_expr(&mut self, expr: &mut Binary<T>) -> Result<T, Box<dyn Error>>;
    fn visit_call_expr(&self, expr: &Call<T>) -> Result<T, Box<dyn Error>>;
    fn visit_get_expr(&self, expr: &Get<T>) -> Result<T, Box<dyn Error>>;
    fn visit_group_expr(&mut self, expr: &mut Grouping<T>) -> Result<T, Box<dyn Error>>;
    fn visit_literal_expr(&self, expr: &Literal) -> Result<T, Box<dyn Error>>;
    fn visit_logical_expr(&self, expr: &Logical<T>) -> Result<T, Box<dyn Error>>;
    fn visit_set_expr(&self, expr: &Set<T>) -> Result<T, Box<dyn Error>>;
    fn visit_super_expr(&self, expr: &Super) -> Result<T, Box<dyn Error>>;
    fn visit_this_expr(&self, expr: &This) -> Result<T, Box<dyn Error>>;
    fn visit_unary_expr(&mut self, expr: &mut Unary<T>) -> Result<T, Box<dyn Error>>;
    fn visit_variable_expr(&self, expr: &Variable) -> Result<T, Box<dyn Error>>; // var a = 20;
}

#[derive(Debug)]
pub struct Assign<T: Debug> {
    pub name: Token,
    pub value: Box<dyn Expr<T>>,
}

impl<T: Debug> Assign<T> {
    pub fn new(name: Token, value: Box<dyn Expr<T>>) -> Self {
        Self { name, value }
    }
}

impl<T: Debug + 'static> Expr<T> for Assign<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_assign_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Binary<T: Debug> {
    pub left: Box<dyn Expr<T>>,
    pub operator: Token,
    pub right: Box<dyn Expr<T>>,
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

impl<T: Debug + 'static> Expr<T> for Binary<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_binary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Call<T: Debug> {
    callee: Box<dyn Expr<T>>,
    paren: Token,
    arguments: Vec<Box<dyn Expr<T>>>,
}

impl<T: Debug> Call<T> {
    pub fn new(callee: Box<dyn Expr<T>>, paren: Token, arguments: Vec<Box<dyn Expr<T>>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

impl<T: Debug + 'static> Expr<T> for Call<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_call_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Get<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_get_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Grouping<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_group_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Literal {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_literal_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Logical<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_logical_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Set<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_set_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Super {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_super_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for This {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_this_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl<T: Debug + 'static> Expr<T> for Unary<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_unary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl<T: Debug + 'static> Expr<T> for Variable {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, Box<dyn Error>> {
        visitor.visit_variable_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
