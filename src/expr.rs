use crate::{object::Object, token::Token};
use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Expr: Debug + ExprClone {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}
pub trait ExprClone {
    fn clone_box(&self) -> Box<dyn Expr>;
}
impl<T> ExprClone for T
where
    T: 'static + Clone + Expr,
{
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Box<dyn Expr> {
        self.clone_box()
    }
}

pub trait Visitor {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Object, Box<dyn Error>>; // a = 30;
    fn visit_binary_expr(&mut self, expr: &mut Binary) -> Result<Object, Box<dyn Error>>;
    fn visit_call_expr(&mut self, expr: &Call) -> Result<Object, Box<dyn Error>>;
    fn visit_get_expr(&mut self, expr: &mut Get) -> Result<Object, Box<dyn Error>>;
    fn visit_group_expr(&mut self, expr: &mut Grouping) -> Result<Object, Box<dyn Error>>;
    fn visit_literal_expr(&self, expr: &Literal) -> Result<Object, Box<dyn Error>>;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Object, Box<dyn Error>>;
    fn visit_set_expr(&mut self, expr: &Set) -> Result<Object, Box<dyn Error>>;
    fn visit_super_expr(&self, expr: &Super) -> Result<Object, Box<dyn Error>>;
    fn visit_this_expr(&self, expr: &This) -> Result<Object, Box<dyn Error>>;
    fn visit_unary_expr(&mut self, expr: &mut Unary) -> Result<Object, Box<dyn Error>>;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Object, Box<dyn Error>>; // var a = 20;
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Rc<RefCell<Box<dyn Expr>>>,
}

impl Assign {
    pub fn new(name: Token, value: Box<dyn Expr>) -> Self {
        Self {
            name,
            value: Rc::new(RefCell::new(value)),
        }
    }
}

impl Expr for Assign {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_assign_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Rc<RefCell<Box<dyn Expr>>>,
    pub operator: Token,
    pub right: Rc<RefCell<Box<dyn Expr>>>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left: Rc::new(RefCell::new(left)),
            operator,
            right: Rc::new(RefCell::new(right)),
        }
    }
}

impl Expr for Binary {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_binary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Rc<RefCell<Box<dyn Expr>>>,
    pub paren: Token,
    pub arguments: Vec<Rc<RefCell<Box<dyn Expr>>>>,
}

impl Call {
    pub fn new(callee: Box<dyn Expr>, paren: Token, arguments: Vec<Box<dyn Expr>>) -> Self {
        Self {
            callee: Rc::new(RefCell::new(callee)),
            paren,
            arguments: arguments
                .into_iter()
                .map(|arg| Rc::new(RefCell::new(arg)))
                .collect(),
        }
    }
}

impl Expr for Call {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_call_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Rc<RefCell<Box<dyn Expr>>>,
    pub name: Token,
}

impl Get {
    pub fn new(object: Box<dyn Expr>, name: Token) -> Self {
        Self {
            object: Rc::new(RefCell::new(object)),
            name,
        }
    }
}

impl Expr for Get {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_get_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Rc<RefCell<Box<dyn Expr>>>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self {
            expression: Rc::new(RefCell::new(expression)),
        }
    }
}

impl Expr for Grouping {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_group_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl Expr for Literal {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_literal_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Rc<RefCell<Box<dyn Expr>>>,
    pub operator: Token,
    pub right: Rc<RefCell<Box<dyn Expr>>>,
}

impl Logical {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left: Rc::new(RefCell::new(left)),
            operator,
            right: Rc::new(RefCell::new(right)),
        }
    }
}

impl Expr for Logical {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_logical_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Rc<RefCell<Box<dyn Expr>>>,
    pub name: Token,
    pub value: Rc<RefCell<Box<dyn Expr>>>,
}

impl Set {
    pub fn new(object: Rc<RefCell<Box<dyn Expr>>>, name: Token, value: Box<dyn Expr>) -> Self {
        Self {
            object,
            name,
            value: Rc::new(RefCell::new(value)),
        }
    }
}

impl Expr for Set {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_set_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}

impl Expr for Super {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_super_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct This {
    keyword: Token,
}

impl This {
    fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

impl Expr for This {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_this_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Rc<RefCell<Box<dyn Expr>>>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            operator,
            right: Rc::new(RefCell::new(right)),
        }
    }
}

impl Expr for Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_unary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl Expr for Variable {
    fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<Object, Box<dyn Error>> {
        visitor.visit_variable_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
