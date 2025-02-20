pub mod instance;

use crate::callable::Callable;
use crate::class::instance::Instance;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Rc<RefCell<Function>>>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Rc<RefCell<Function>>>) -> Class {
        Class { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<RefCell<Function>>> {
        if self.methods.contains_key(name) {
            return Some(self.methods.get(name).unwrap().clone());
        }
        None
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
