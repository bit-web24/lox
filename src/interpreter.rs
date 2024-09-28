use crate::{expr, object::Object, token::token_type::TokenType};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Interpreter;

impl Interpreter {
    fn evaluate(self, expr: &dyn expr::Expr<Object>) -> Option<Object> {
        expr.accept(&self)
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &expr::Literal) -> Option<Object> {
        Some(expr.value.clone())
    }

    fn visit_assign_expr(&self, expr: &expr::Assign<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &expr::Binary<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_call_expr(&self, expr: &expr::Call<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_get_expr(&self, expr: &expr::Get<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_group_expr(&self, expr: &expr::Grouping<Object>) -> Option<Object> {
        self.clone().evaluate(expr.expression.as_ref())
    }

    fn visit_logical_expr(&self, expr: &expr::Logical<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_set_expr(&self, expr: &expr::Set<Object>) -> Option<Object> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &expr::Super) -> Option<Object> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &expr::This) -> Option<Object> {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &expr::Unary<Object>) -> Option<Object> {
        let right: Option<Object> = self.clone().evaluate(expr.right.as_ref());

        match expr.operator.type_ {
            TokenType::MINUS => match right {
                Some(Object::Number(n)) => Some(Object::Number(-n)),
                _ => None,
            },
            TokenType::BANG => Some(Object::Boolean(right.is_none())),
            _ => None,
        }
    }

    fn visit_variable_expr(&self, expr: &expr::Variable) -> Option<Object> {
        todo!()
    }
}
