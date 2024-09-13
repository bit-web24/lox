use crate::{object::Object, token::Token};
use crate::expr;

pub trait Stmt<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

trait Visitor<T> {
    fn visit_block_stmt(&self, stmt: &Block<T>) -> T;
    fn visit_class_stmt(&self, stmt: &Class<T>) -> T;
    fn visit_expr_stmt(&self, stmt: &Expression<T>) -> T;
    fn visit_func_stmt(&self, stmt: &Function<T>) -> T;
    fn visit_if_statement(&self, stmt: &If<T>) -> T;
    fn visit_print_stmt(&self, stmt: &Print<T>) -> T;
    fn visit_return_stmt(&self, stmt: &Return<T>) -> T;
    fn visit_var_stmt(&self, stmt: &Var<T>) -> T;
    fn visit_while_stmt(&self, stmt: &While<T>) -> T;
}

struct Block<T> {
    statements: Vec<Box<dyn Stmt<T>>>,
}

impl<T> Block<T> {
    fn new(statements: Vec<Box<dyn Stmt<T>>>) -> Self {
        Self {
            statements,
        }
    }
}

impl<T> Stmt<T> for Block<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        return visitor.visit_block_stmt(self);
    }
}

struct Class<T> {
    name: Token,
    superclass: expr::Variable,
    methods: Vec<Function<T>>,
}

struct Expression<T> {
    expression: Box<dyn expr::Expr<T>>,
}

struct Function<T> {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<dyn Stmt<T>>>,
}

struct If<T> {
    condition: Box<dyn expr::Expr<T>>,
    then_branch: Box<dyn Stmt<T>>,
    else_branch: Box<dyn Stmt<T>>,
}

struct Print<T> {
    expression: Box<dyn expr::Expr<T>>,
}

struct Return<T> {
    keyword: Token,
    value: Box<dyn expr::Expr<T>>,
}

struct Var<T> {
    name: Token,
    initializer: Box<dyn expr::Expr<T>>,
}

struct While<T> {
    condition: Box<dyn expr::Expr<T>>,
    body: Box<dyn Stmt<T>>,
}
