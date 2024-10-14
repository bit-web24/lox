use crate::{
    env::Environment,
    error::{error_types::RuntimeError, LoxError},
    expr,
    object::Object,
    stmt::{self, Stmt},
    token::{token_type::TokenType, Token},
};
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn evaluate(
        &mut self,
        expr: &mut dyn expr::Expr<Object>,
    ) -> Result<Object, Box<dyn std::error::Error>> {
        expr.accept(self)
    }

    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn interpret(
        &mut self,
        statements: Vec<Box<dyn Stmt<Object>>>,
    ) -> Result<Object, Box<dyn Error>> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(Object::Nil)
    }

    fn execute(&mut self, mut stmt: Box<dyn Stmt<Object>>) -> Result<(), Box<dyn Error>> {
        stmt.accept(self)?;
        Ok(())
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

    fn visit_assign_expr(
        &mut self,
        expr: &mut expr::Assign<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        let value = self.evaluate(expr.value.as_mut())?;
        self.env.assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(
        &mut self,
        expr: &mut expr::Binary<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        let left = self.evaluate(expr.left.as_mut())?;
        let right = self.evaluate(expr.right.as_mut())?;

        let rhs = right.to_owned();
        let check_number_operands = |v: Object| {
            if v.is_nil() {
                if rhs == Object::Number(0.0) {
                    return Err(self.error("Can't divide by zero.", &expr.operator));
                } else {
                    return Err(self.error("Operands must be numbers.", &expr.operator));
                }
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

    fn visit_group_expr(
        &mut self,
        expr: &mut expr::Grouping<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        self.evaluate(expr.expression.as_mut())
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

    fn visit_unary_expr(
        &mut self,
        expr: &mut expr::Unary<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        let right = self.evaluate(expr.right.as_mut())?;

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
        let value = self.env.get(&expr.name)?;
        Ok(value.to_owned())
    }
}

impl stmt::Visitor<Object> for Interpreter {
    fn visit_block_stmt(&self, stmt: &stmt::Block<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_class_stmt(&self, stmt: &stmt::Class<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_expr_stmt(
        &mut self,
        stmt: &mut stmt::Expression<Object>,
    ) -> Result<(), Box<dyn Error>> {
        self.evaluate(stmt.expression.as_mut())?;
        Ok(())
    }

    fn visit_func_stmt(&self, stmt: &stmt::Function<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_if_stmt(&self, stmt: &stmt::If<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &mut stmt::Print<Object>) -> Result<(), Box<dyn Error>> {
        let value = self.evaluate(stmt.expression.as_mut())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &stmt::Return<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &mut stmt::Var<Object>) -> Result<(), Box<dyn Error>> {
        let mut value = Object::Nil;
        if stmt.initializer.is_some() {
            value = self.evaluate(stmt.initializer.as_mut().unwrap().as_mut())?;
        }

        self.env.define(&stmt.name, value)?;
        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &stmt::While<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
