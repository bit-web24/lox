use std::{collections::HashMap, error::Error};

use crate::{
    error::{error_types::RuntimeError, LoxError},
    object::Object,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, token: &Token) -> Result<&Object, Box<dyn Error>> {
        if self.values.contains_key(token.lexeme.as_str()) {
            return Ok(self.values.get(token.lexeme.as_str()).unwrap());
        }

        Err(Environment::error(format!(
            "Undefined variable '{}'.",
            token.lexeme
        )))
    }

    pub fn define(&mut self, name: &str, value: Object) -> Result<(), Box<dyn Error>> {
        if self.values.contains_key(name) {
            return Err(Environment::error("variable already defined.".into()));
        }

        self.values.insert(name.to_owned(), value);
        Ok(())
    }

    fn error(message: String) -> Box<dyn Error> {
        Box::new(
            LoxError::new()
                .type_(Box::new(RuntimeError))
                .message(message),
        )
    }
}
