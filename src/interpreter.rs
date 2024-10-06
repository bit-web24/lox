use crate::{
    error::{error_types::RuntimeError, LoxError},
    expr,
    object::Object,
    token::{token_type::TokenType, Token},
};
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(
        self,
        expr: &dyn expr::Expr<Object>,
    ) -> Result<Object, Box<dyn std::error::Error>> {
        expr.accept(&self)
    }

    pub fn new() -> Self {
        Self
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    fn error(&self, message: &str, token: &Token) -> Box<dyn Error> {
        let mut err = LoxError::new();
        err = err
            .type_(Box::new(RuntimeError))
            .at_token(token.to_owned())
            .message(message.to_string());
        Box::new(err)
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &expr::Literal) -> Result<Object, Box<dyn Error>> {
        Ok(expr.value.clone())
    }

    fn visit_assign_expr(&self, expr: &expr::Assign<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &expr::Binary<Object>) -> Result<Object, Box<dyn Error>> {
        let left = self.evaluate(expr.left.as_ref())?;
        let right = self.evaluate(expr.right.as_ref())?;

        let rhs = right.to_owned();
        let check_number_operands = |v: Object| {
            if v.is_nil() {
                if rhs == Object::Number(0.0) {
                    return Err(self.error("Can't divide by zero.", &expr.operator));
                }
                return Err(self.error("Operands must be numbers.", &expr.operator));
            }

            Ok(v)
        };

        let check_addition_operands = |v: Object| {
            if v.is_nil() {
                return Err(self.error(
                    "Operands can be in pairs of:\n\t(String::String)\n\t(Number::Number)\n\t(String::Number)\n\t(Number::String)\n\t(Boolean::String) or\n\t(String::Boolean)",
                    &expr.operator,
                ));
            }
            Ok(v)
        };

        match expr.operator.type_ {
            TokenType::MINUS => check_number_operands(left - right),
            TokenType::SLASH => check_number_operands(left / right),
            TokenType::STAR => check_number_operands(left * right),
            TokenType::PLUS => check_addition_operands(left + right),
            TokenType::GREATER => check_number_operands(Object::Boolean(left > right)),
            TokenType::GREATER_EQUAL => check_number_operands(Object::Boolean(left >= right)),
            TokenType::LESS => check_number_operands(Object::Boolean(left < right)),
            TokenType::LESS_EQUAL => check_number_operands(Object::Boolean(left <= right)),
            _ => {
                let mut err = LoxError::new();
                err = err
                    .type_(Box::new(RuntimeError))
                    .message(format!(
                        "Unsupported binary operator: {}",
                        expr.operator.lexeme
                    ))
                    .at_token(expr.operator.to_owned());
                Err(Box::new(err))
            }
        }
    }

    fn visit_call_expr(&self, expr: &expr::Call<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_get_expr(&self, expr: &expr::Get<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_group_expr(&self, expr: &expr::Grouping<Object>) -> Result<Object, Box<dyn Error>> {
        self.clone().evaluate(expr.expression.as_ref())
    }

    fn visit_logical_expr(&self, expr: &expr::Logical<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_set_expr(&self, expr: &expr::Set<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &expr::Super) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &expr::This) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &expr::Unary<Object>) -> Result<Object, Box<dyn Error>> {
        let right = self.evaluate(expr.right.as_ref())?;

        match expr.operator.type_ {
            TokenType::MINUS => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => {
                    let err = LoxError::new();
                    let err_ = err
                        .type_(Box::new(RuntimeError))
                        .at_token(expr.operator.to_owned())
                        .message("Operand must be a number".to_string());
                    Err(Box::new(err_))
                }
            },
            TokenType::BANG => Ok(Object::Boolean(right.is_nil())),
            _ => {
                let mut err = LoxError::new();
                err = err
                    .type_(Box::new(RuntimeError))
                    .at_token(expr.operator.to_owned())
                    .message("Expected Number found".to_string());
                Err(Box::new(err))
            }
        }
    }

    fn visit_variable_expr(&self, expr: &expr::Variable) -> Result<Object, Box<dyn Error>> {
        todo!()
    }
}
