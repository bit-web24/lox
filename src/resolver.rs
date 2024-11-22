use std::{collections::HashMap, error::Error, fmt::Debug};

use crate::{
    expr::{self, Expr},
    interpreter::Interpreter,
    object::Object,
    stmt::{self, Stmt},
    token::Token,
};

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Resolver<'a> {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(
        &mut self,
        statements: &mut Vec<Box<dyn Stmt<Object>>>,
    ) -> Result<(), Box<dyn Error>> {
        for statement in statements.iter_mut() {
            self.resolve_statement(statement.as_mut())?;
        }
        Ok(())
    }

    fn resolve_statement(
        &mut self,
        statement: &mut dyn Stmt<Object>,
    ) -> Result<(), Box<dyn Error>> {
        statement.accept(self)?;
        Ok(())
    }

    fn resolve_expression(
        &mut self,
        expr: &mut dyn Expr<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) -> Result<(), Box<dyn Error>> {
        if self.scopes.pop().is_none() {
            Err(self.interpreter.error(
                "ResolverError: error while ending subscope.",
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
}

impl<'a, T> stmt::Visitor<T> for Resolver<'a>
where
    T: Debug,
{
    fn visit_block_stmt(
        &mut self,
        stmt: &crate::stmt::Block<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_class_stmt(
        &self,
        stmt: &crate::stmt::Class<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_expr_stmt(
        &mut self,
        stmt: &mut crate::stmt::Expression<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_func_stmt(
        &self,
        stmt: &crate::stmt::Function<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_if_stmt(
        &mut self,
        stmt: &mut crate::stmt::If<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_print_stmt(
        &mut self,
        stmt: &mut crate::stmt::Print<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_return_stmt(
        &mut self,
        stmt: &crate::stmt::Return<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_var_stmt(
        &mut self,
        stmt: &mut crate::stmt::Var<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn visit_while_stmt(
        &mut self,
        stmt: &crate::stmt::While<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl<'a, T> expr::Visitor<T> for Resolver<'a>
where
    T: Debug,
{
    fn visit_assign_expr(&mut self, expr: &expr::Assign<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &mut expr::Binary<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &expr::Call<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_get_expr(&self, expr: &expr::Get<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_group_expr(&mut self, expr: &mut expr::Grouping<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_literal_expr(&self, expr: &expr::Literal) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &expr::Logical<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_set_expr(&self, expr: &expr::Set<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &expr::Super) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &expr::This) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &mut expr::Unary<T>) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn visit_variable_expr(&self, expr: &expr::Variable) -> Result<T, Box<dyn Error>> {
        todo!()
    }
}
