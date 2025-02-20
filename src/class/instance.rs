use crate::callable::Callable;
use crate::class::Class;
use crate::error::error_types::RuntimeError;
use crate::error::LoxError;
use crate::function::Function;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Instance {
    class: Rc<RefCell<Class>>,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(class: Rc<RefCell<Class>>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object, Box<dyn Error>> {
        if self.fields.contains_key(token.lexeme.as_str()) {
            let value = self.fields.get(token.lexeme.as_str());
            return Ok(value.unwrap().clone());
        }
        let method: Option<Rc<RefCell<Function>>> =
            self.class.borrow().find_method(token.lexeme.as_str());
        if let Some(method) = method {
            return Ok(Object::Function(Some(method), None));
        }
        Err(self.error(
            format!("Undefined property '{}'.", token.lexeme).as_str(),
            token,
        ))
    }

    pub fn set(&mut self, token: &Token, value: Object) {
        self.fields.insert(token.lexeme.clone(), value);
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

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().to_string())
    }
}
