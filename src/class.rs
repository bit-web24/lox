pub mod instance;

use crate::callable::Callable;
use crate::class::instance::Instance;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class { name }
    }
}

impl Callable for Class {
    fn call(
        &self,
        _interpreter: Interpreter,
        _arguments: Vec<Object>,
        _paren: Token,
    ) -> Result<Object, Box<dyn Error>> {
        let class_ref = Rc::new(RefCell::new(self.clone()));
        let instance = Rc::new(RefCell::new(Instance::new(class_ref)));
        Ok(Object::Instance(instance))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        format!("<class {}>", self.name)
    }
}
