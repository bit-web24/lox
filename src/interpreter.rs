use crate::{
    error::{self, Error, ParseError, RuntimeError},
    expr,
    object::Object,
    token::token_type::TokenType,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Interpreter;

impl Interpreter {
    fn evaluate(self, expr: &dyn expr::Expr<Object>) -> Result<Object, Error> {
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
    fn visit_literal_expr(&self, expr: &expr::Literal) -> Result<Object, Error> {
        Ok(expr.value.clone())
    }

    fn visit_assign_expr(&self, expr: &expr::Assign<Object>) -> Result<Object, Error> {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &expr::Binary<Object>) -> Result<Object, Error> {
        let left = self.evaluate(expr.left.as_ref())?;
        let right = self.evaluate(expr.right.as_ref())?;

        match (&left, &right) {
            (Object::Number(_), Object::Number(_)) => (),
            _ => {
                let mut err = Error::new();
                err = err
                    .type_(Box::new(RuntimeError))
                    .message("Operands must be numbers.".to_string())
                    .at_token(expr.operator.to_owned());
                return Err(err);
            }
        }

        match expr.operator.type_ {
            TokenType::MINUS => Ok(left - right),
            TokenType::SLASH => Ok(left / right),
            TokenType::STAR => Ok(left * right),
            TokenType::PLUS => Ok(left + right),
            TokenType::GREATER => Ok(Object::Boolean(left > right)),
            TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
            TokenType::LESS => Ok(Object::Boolean(left < right)),
            TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
            _ => {
                let mut err = Error::new();
                err = err
                    .type_(Box::new(RuntimeError))
                    .message(format!(
                        "Unsupported binary operator: {}",
                        expr.operator.lexeme
                    ))
                    .at_token(expr.operator.to_owned());
                Err(err)
            }
        }
    }

    fn visit_call_expr(&self, expr: &expr::Call<Object>) -> Result<Object, Error> {
        todo!()
    }

    fn visit_get_expr(&self, expr: &expr::Get<Object>) -> Result<Object, Error> {
        todo!()
    }

    fn visit_group_expr(&self, expr: &expr::Grouping<Object>) -> Result<Object, Error> {
        self.clone().evaluate(expr.expression.as_ref())
    }

    fn visit_logical_expr(&self, expr: &expr::Logical<Object>) -> Result<Object, Error> {
        todo!()
    }

    fn visit_set_expr(&self, expr: &expr::Set<Object>) -> Result<Object, Error> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &expr::Super) -> Result<Object, Error> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &expr::This) -> Result<Object, Error> {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &expr::Unary<Object>) -> Result<Object, Error> {
        let right = self.evaluate(expr.right.as_ref())?;

        match expr.operator.type_ {
            TokenType::MINUS => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => {
                    let err = Error::new();
                    let err_ = err
                        .type_(Box::new(RuntimeError))
                        .at_token(expr.operator.to_owned())
                        .message("Operand must be a number".to_string());
                    Err(err_)
                }
            },
            TokenType::BANG => Ok(Object::Boolean(right.is_nil())),
            _ => {
                let mut err = Error::new();
                err = err
                    .type_(Box::new(RuntimeError))
                    .at_token(expr.operator.to_owned())
                    .message("Expected Number found".to_string());
                Err(err)
            }
        }
    }

    fn visit_variable_expr(&self, expr: &expr::Variable) -> Result<Object, Error> {
        todo!()
    }
}
