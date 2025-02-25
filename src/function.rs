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
    pub declaration: stmt::Function,
    pub closeure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: stmt::Function, closeure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closeure,
        }
    }
}

impl Callable for Function {
    fn call(
        &self,
        mut interpreter: Interpreter,
        arguments: Vec<Object>,
        _paren: Token,
    ) -> Result<Object, Box<dyn Error>> {
        let environment = Rc::new(RefCell::new(Environment::from(self.closeure.clone())));

        for i in 0..self.declaration.params.len() {
            environment
                .borrow_mut()
                .define(&self.declaration.params[i], arguments[i].clone())?;
        }

        if let Err(err) =
            interpreter.execute_block(self.declaration.body.clone(), environment.clone())
        {
            let v = err
                .as_ref()
                .downcast_ref::<crate::interpreter::return_v::Return>();

            if let Some(val) = v {
                return Ok(val.value.clone());
            }

            return Err(err);
        }

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
