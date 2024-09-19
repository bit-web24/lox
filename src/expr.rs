use crate::{object::Object, token::Token};

pub trait Expr<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

trait Visitor<T> {
    fn visit_assign_expr(&self, expr: &Assign<T>) -> T;
    fn visit_binary_expr(&self, expr: &Binary<T>) -> T;
    fn visit_call_expr(&self, expr: &Call<T>) -> T;
    fn visit_get_expr(&self, expr: &Get<T>) -> T;
    fn visit_group_expr(&self, expr: &Grouping<T>) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_logical_expr(&self, expr: &Logical<T>) -> T;
    fn visit_set_expr(&self, expr: &Set<T>) -> T;
    fn visit_super_expr(&self, expr: &Super) -> T;
    fn visit_this_expr(&self, expr: &This) -> T;
    fn visit_unary_expr(&self, expr: &Unary<T>) -> T;
    fn visit_variable_expr(&self, expr: &Variable) -> T;
}

struct Assign<T> {
    name: Token,
    value: Box<dyn Expr<T>>,
}

impl<T> Assign<T> {
    fn new(name: Token, value: Box<dyn Expr<T>>) -> Self {
        Self { name, value }
    }
}

impl<T> Expr<T> for Assign<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_assign_expr(self);
    }
}

pub struct Binary<T> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T> Binary<T> {
    pub fn new(left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<T> Expr<T> for Binary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_binary_expr(self);
    }
}

struct Call<T> {
    callee: Box<dyn Expr<T>>,
    paren: Token,
    arguments: Vec<Box<dyn Expr<T>>>,
}

impl<T> Call<T> {
    fn new(callee: Box<dyn Expr<T>>, paren: Token, arguments: Vec<Box<dyn Expr<T>>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

impl<T> Expr<T> for Call<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_call_expr(self);
    }
}

struct Get<T> {
    object: Box<dyn Expr<T>>,
    name: Token,
}

impl<T> Get<T> {
    fn new(object: Box<dyn Expr<T>>, name: Token) -> Self {
        Self { object, name }
    }
}

impl<T> Expr<T> for Get<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_get_expr(self);
    }
}

pub struct Grouping<T> {
    expression: Box<dyn Expr<T>>,
}

impl<T> Grouping<T> {
    pub fn new(expression: Box<dyn Expr<T>>) -> Self {
        Self { expression }
    }
}

impl<T> Expr<T> for Grouping<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_group_expr(self);
    }
}

pub struct Literal {
    value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl<T> Expr<T> for Literal {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_literal_expr(self);
    }
}

struct Logical<T> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T> Logical<T> {
    fn new(left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<T> Expr<T> for Logical<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_logical_expr(self);
    }
}

struct Set<T> {
    object: Box<dyn Expr<T>>,
    name: Token,
    value: Box<dyn Expr<T>>,
}

impl<T> Set<T> {
    fn new(object: Box<dyn Expr<T>>, name: Token, value: Box<dyn Expr<T>>) -> Self {
        Self {
            object,
            name,
            value,
        }
    }
}

impl<T> Expr<T> for Set<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_set_expr(self);
    }
}

struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}

impl<T> Expr<T> for Super {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_super_expr(self);
    }
}

struct This {
    keyword: Token,
}

impl This {
    fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

impl<T> Expr<T> for This {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_this_expr(self);
    }
}

pub struct Unary<T> {
    operator: Token,
    right: Box<dyn Expr<T>>,
}

impl<T> Unary<T> {
    pub fn new(operator: Token, right: Box<dyn Expr<T>>) -> Self {
        Self { operator, right }
    }
}

impl<T> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_unary_expr(self);
    }
}

pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl<T> Expr<T> for Variable {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_variable_expr(self);
    }
}
