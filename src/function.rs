use crate::callable::Callable;
use crate::env::Environment;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
use crate::token::Token;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Debug)]
pub struct Function {
    pub declaration: stmt::Function<Object>,
}

impl Function {
    pub fn new(declaration: stmt::Function<Object>) -> Self {
        Self { declaration }
    }
}

impl Callable for Function {
    fn call(
        &self,
        mut interpreter: Interpreter,
        arguments: Vec<Object>,
        _paren: Token,
    ) -> Result<Object, Box<dyn Error>> {
        let environment = Rc::new(RefCell::new(Environment::from(interpreter.env.clone())));

        for i in 0..self.declaration.params.len() {
            environment
                .borrow_mut()
                .define(&self.declaration.params[i], arguments[i].clone())?;
        }

        interpreter.execute_block(self.declaration.body.clone(), environment.clone())?;

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
