use crate::{
    expr::{self, Expr},
    interpreter::Interpreter,
    object::Object,
    stmt::{self, Stmt},
    token::Token,
};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ops::DerefMut;
use std::rc::Rc;
use std::{collections::HashMap, error::Error};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_func: FuncType,
}

#[derive(Clone, PartialEq)]
enum FuncType {
    None,
    Function,
    Method,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver<'a> {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_func: FuncType::None,
        }
    }

    pub fn resolve(&mut self, statements: &mut Vec<Box<dyn Stmt>>) -> Result<(), Box<dyn Error>> {
        for statement in statements.iter_mut() {
            self.resolve_statement(statement.as_mut())?;
        }
        Ok(())
    }

    pub fn resolve_rc(
        &mut self,
        statements: &mut Vec<Rc<RefCell<Box<dyn Stmt>>>>,
    ) -> Result<(), Box<dyn Error>> {
        for statement in statements.iter_mut() {
            self.resolve_statement(statement.borrow_mut().as_mut())?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &mut dyn Stmt) -> Result<(), Box<dyn Error>> {
        statement.accept(self)?;
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &mut dyn Expr) -> Result<Object, Box<dyn Error>> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) -> Result<(), Box<dyn Error>> {
        if self.scopes.pop().is_none() {
            Err(self.interpreter.error(
                "ResolverError: error while ending sub-scope.",
                &Token::new(
                    crate::token::token_type::TokenType::NIL,
                    "None".to_string(),
                    None,
                    0,
                ),
            ))
        } else {
            Ok(())
        }
    }

    pub fn declare(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                return Err(self.interpreter.error(
                    "ResolverError: Already a variable with this name in this scope.",
                    &Token::new(
                        crate::token::token_type::TokenType::NIL,
                        "None".to_string(),
                        None,
                        0,
                    ),
                ));
            }
            scope.insert(name.to_string(), false);
        }
        Ok(())
    }

    pub fn define(&mut self, name: &str) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    pub fn resolve_local(&mut self, expr: &dyn Expr, name: &str) {
        let n = self.scopes.len();
        for i in (0..n).rev() {
            if self.scopes.get(i).unwrap().contains_key(name) {
                let distance = self.scopes.len() - 1 - i;
                self.interpreter.resolve(expr.clone_box(), distance as i32);
                return;
            }
        }
    }

    fn resolve_func(
        &mut self,
        func: &stmt::Function,
        func_type: FuncType,
    ) -> Result<(), Box<dyn Error>> {
        let enclosing_func = self.current_func.clone();
        self.current_func = func_type;

        self.begin_scope();
        for param in &func.params {
            self.declare(param.lexeme.as_str())?;
            self.define(param.lexeme.as_str());
        }
        self.end_scope()?;
        self.current_func = enclosing_func;
        Ok(())
    }
}

impl<'a> stmt::Visitor for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &mut stmt::Block) -> Result<(), Box<dyn Error>> {
        self.begin_scope();
        self.resolve_rc(&mut stmt.statements)?;
        self.end_scope()?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Result<(), Box<dyn Error>> {
        self.declare(stmt.name.lexeme.as_str())?;
        self.define(stmt.name.lexeme.as_str());

        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".to_string(), true);

        for method in stmt.methods.borrow().iter() {
            let declaration = FuncType::Method;
            self.resolve_func(method, declaration)?
        }

        self.end_scope()?;
        Ok(())
    }

    fn visit_expr_stmt(&mut self, stmt: &mut stmt::Expression) -> Result<(), Box<dyn Error>> {
        self.resolve_expression(stmt.expression.borrow_mut().as_mut())?;
        Ok(())
    }

    fn visit_func_stmt(&mut self, stmt: &stmt::Function) -> Result<(), Box<dyn Error>> {
        self.declare(stmt.name.lexeme.as_str())?;
        self.define(stmt.name.lexeme.as_str());
        self.resolve_func(stmt, FuncType::Function)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &mut stmt::If) -> Result<(), Box<dyn Error>> {
        self.resolve_expression(stmt.condition.borrow_mut().as_mut())?;
        self.resolve_statement(stmt.then_branch.borrow_mut().as_mut())?;
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_statement(else_branch.borrow_mut().as_mut())?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &mut stmt::Print) -> Result<(), Box<dyn Error>> {
        self.resolve_expression(stmt.expression.borrow_mut().as_mut())?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Result<(), Box<dyn Error>> {
        if self.current_func == FuncType::None {
            return Err(self.interpreter.error(
                "ResolverError: Can't return from top-level code.",
                &Token::new(
                    crate::token::token_type::TokenType::NIL,
                    "None".to_string(),
                    None,
                    0,
                ),
            ));
        }
        if let Some(value) = &stmt.value {
            self.resolve_expression(value.borrow_mut().as_mut())?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &mut stmt::Var) -> Result<(), Box<dyn Error>> {
        self.declare(&stmt.name.lexeme)?;
        if stmt.initializer.is_some() {
            let expr = stmt.initializer.as_ref().unwrap();
            let mut c = expr.borrow_mut();
            let expr: &mut Box<dyn Expr> = c.deref_mut();
            let expr = expr.as_mut();
            self.resolve_expression(expr)?;
        }
        self.define(stmt.name.lexeme.as_str());
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Result<(), Box<dyn Error>> {
        self.resolve_expression(stmt.condition.borrow_mut().as_mut())?;
        self.resolve_statement(stmt.body.borrow_mut().as_mut())?;
        Ok(())
    }
}

impl<'a> expr::Visitor for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.value.borrow_mut().as_mut())?;
        self.resolve_local(expr, expr.name.lexeme.as_str());
        Ok(Object::Nil)
    }

    fn visit_binary_expr(&mut self, expr: &mut expr::Binary) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.left.borrow_mut().as_mut())?;
        self.resolve_expression(expr.right.borrow_mut().as_mut())?;
        Ok(Object::Nil)
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.callee.borrow_mut().as_mut())?;
        for arg in &expr.arguments {
            self.resolve_expression(arg.borrow_mut().as_mut())?;
        }
        Ok(Object::Nil)
    }

    fn visit_get_expr(&mut self, expr: &mut expr::Get) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.object.borrow_mut().as_mut())?;
        Ok(Object::Nil)
    }

    fn visit_group_expr(&mut self, expr: &mut expr::Grouping) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.expression.borrow_mut().as_mut())?;
        Ok(Object::Nil)
    }

    fn visit_literal_expr(&self, _expr: &expr::Literal) -> Result<Object, Box<dyn Error>> {
        Ok(Object::Nil)
    }

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.left.borrow_mut().as_mut())?;
        self.resolve_expression(expr.right.borrow_mut().as_mut())?;
        Ok(Object::Nil)
    }

    fn visit_set_expr(&mut self, expr: &expr::Set) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.value.borrow_mut().as_mut())?;
        self.resolve_expression(expr.object.borrow_mut().as_mut())?;

        Ok(Object::Nil)
    }

    fn visit_super_expr(&self, expr: &expr::Super) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> Result<Object, Box<dyn Error>> {
        self.resolve_local(expr, expr.keyword.lexeme.as_str());
        Ok(Object::Nil)
    }

    fn visit_unary_expr(&mut self, expr: &mut expr::Unary) -> Result<Object, Box<dyn Error>> {
        self.resolve_expression(expr.right.borrow_mut().as_mut())?;
        Ok(Object::Nil)
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<Object, Box<dyn Error>> {
        if !self.scopes.is_empty() {
            let tmp = self.scopes.last().unwrap().get(&expr.name.lexeme);
            if !tmp.is_none() && tmp.unwrap() == &false {
                return Err(self.interpreter.error(
                    "ResolverError: cannot read local variable in its own initializer.",
                    &expr.name,
                ));
            }
        }
        self.resolve_local(expr, expr.name.lexeme.as_str());
        Ok(Object::Nil)
    }
}
