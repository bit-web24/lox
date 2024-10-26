use return_v::Return;

use crate::{
    callable,
    env::Environment,
    error::{error_types::RuntimeError, LoxError},
    expr, function,
    object::Object,
    stmt::{self, Stmt},
    token::{token_type::TokenType, Token},
};

pub mod return_v;

use crate::callable::Callable;

use std::{borrow::Borrow, cell::RefCell, error::Error, ops::Not, rc::Rc};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn evaluate(
        &mut self,
        expr: Rc<RefCell<Box<dyn expr::Expr<Object>>>>,
    ) -> Result<Object, Box<dyn std::error::Error>> {
        expr.borrow_mut().accept(self)
    }

    pub fn new() -> Self {
        let interpreter = Self {
            env: Rc::new(RefCell::new(Environment::new())),
        };

        for (name, function) in callable::get_native_functions() {
            interpreter
                .env
                .borrow_mut()
                .define(
                    &Token::new(TokenType::IDENTIFIER, name.to_string(), None, 0),
                    function,
                )
                .unwrap();
        }

        interpreter
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
        let statements = statements;

        for statement in statements {
            self.execute(Rc::new(RefCell::new(statement)))?;
        }

        Ok(Object::Nil)
    }
    pub fn execute(
        &mut self,
        stmt: Rc<RefCell<Box<dyn Stmt<Object>>>>,
    ) -> Result<(), Box<dyn Error>> {
        stmt.borrow_mut().accept(self)?;
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Rc<RefCell<Box<dyn Stmt<Object>>>>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), Box<dyn Error>> {
        let previous = self.env.clone();
        self.env = environment.clone();

        for statement in statements {
            self.execute(statement)?;
        }

        self.env = previous;
        Ok(())
    }

    pub fn error(&self, message: &str, token: &Token) -> Box<dyn Error> {
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
        let value = self.evaluate(expr.value.clone())?;
        self.env.borrow_mut().assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(
        &mut self,
        expr: &mut expr::Binary<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;

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
            TokenType::MINUS => Ok(left - right),
            TokenType::SLASH => check_number_operands(left / right),
            TokenType::STAR => Ok(left * right),
            TokenType::PLUS => check_addition_operands(left + right),
            TokenType::GREATER => Ok(Object::Boolean(left > right)),
            TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
            TokenType::LESS => Ok(Object::Boolean(left < right)),
            TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
            TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
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

    fn visit_call_expr(&mut self, expr: &expr::Call<Object>) -> Result<Object, Box<dyn Error>> {
        let function: Box<dyn Callable> = Box::new(self.evaluate(expr.callee.clone())?);

        let arguments = expr
            .arguments
            .iter()
            .map(|arg| self.evaluate(arg.clone()))
            .collect::<Result<Vec<Object>, Box<dyn Error>>>()?;
        let returned_v = function.call(self.clone(), arguments, expr.paren.to_owned())?;

        Ok(returned_v)
    }

    fn visit_get_expr(&self, expr: &expr::Get<Object>) -> Result<Object, Box<dyn Error>> {
        todo!()
    }

    fn visit_group_expr(
        &mut self,
        expr: &mut expr::Grouping<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        self.evaluate(expr.expression.clone())
    }

    fn visit_logical_expr(
        &mut self,
        expr: &expr::Logical<Object>,
    ) -> Result<Object, Box<dyn Error>> {
        let left = self.evaluate(expr.left.clone())?;

        let mut get_truth = |token_type: &TokenType| {
            let mut truth = Interpreter::is_truthy(left.borrow());
            truth = if token_type.eq(&TokenType::AND) {
                truth.not()
            } else {
                truth
            };

            if truth {
                Ok(left.borrow().to_owned())
            } else {
                Ok(self.evaluate(expr.right.clone())?)
            }
        };

        match expr.operator.type_.borrow() {
            TokenType::AND => get_truth(&TokenType::AND),
            TokenType::OR => get_truth(&TokenType::OR),
            _ => Err(self.error("Unexpected Token", &expr.operator)),
        }
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
        let right = self.evaluate(expr.right.clone())?;

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
        let value = self.env.borrow_mut().get(&expr.name)?;
        Ok(value)
    }
}

impl stmt::Visitor<Object> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &stmt::Block<Object>) -> Result<(), Box<dyn Error>> {
        self.execute_block(
            stmt.statements.clone(),
            Rc::new(RefCell::new(Environment::from(self.env.clone()))),
        )?;
        Ok(())
    }

    fn visit_class_stmt(&self, stmt: &stmt::Class<Object>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn visit_expr_stmt(
        &mut self,
        stmt: &mut stmt::Expression<Object>,
    ) -> Result<(), Box<dyn Error>> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }

    fn visit_func_stmt(&self, stmt: &stmt::Function<Object>) -> Result<(), Box<dyn Error>> {
        let function: function::Function = function::Function::new(stmt.to_owned());
        let fn_obj = Object::Function(Some(Rc::new(RefCell::new(function))), None);
        self.env.borrow_mut().define(&stmt.name, fn_obj)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &mut stmt::If<Object>) -> Result<(), Box<dyn Error>> {
        if Interpreter::is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())?;
        } else {
            if let Some(else_stmt) = stmt.else_branch.clone() {
                self.execute(else_stmt.clone())?;
            }
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &mut stmt::Print<Object>) -> Result<(), Box<dyn Error>> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return<Object>) -> Result<(), Box<dyn Error>> {
        if let Some(value) = stmt.value.clone() {
            let value = self.evaluate(value.clone())?;
            return Err(Box::new(Return { value }));
        }

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &mut stmt::Var<Object>) -> Result<(), Box<dyn Error>> {
        let mut value = Object::Nil;
        if stmt.initializer.is_some() {
            value = self.evaluate(stmt.initializer.clone().unwrap())?;
        }

        self.env.borrow_mut().define(&stmt.name, value)?;
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While<Object>) -> Result<(), Box<dyn Error>> {
        while Interpreter::is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.body.clone())?;
        }

        Ok(())
    }
}
