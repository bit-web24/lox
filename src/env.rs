use core::error;
use std::{collections::HashMap, error::Error, fmt};

use crate::{
    error::{error_types::RuntimeError, LoxError},
    object::Object,
    token::{self, Token},
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
        if let Some(value) = self.values.get(token.lexeme.as_str()) {
            return Ok(value);
        }

        Err(Self::error(
            format!("Undefined variable '{}'.", token.lexeme),
            token.to_owned(),
        ))
    }

    pub fn define(&mut self, token: &Token, value: Object) -> Result<(), Box<dyn Error>> {
        if self.values.contains_key(token.lexeme.as_str()) {
            return Err(Self::error(
                "variable already defined.".into(),
                token.to_owned(),
            ));
        }

        self.values.insert(token.lexeme.to_owned(), value);
        Ok(())
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<(), Box<dyn Error>> {
        if self.values.contains_key(token.lexeme.as_str()) {
            self.values.insert(token.lexeme.to_owned(), value);
            return Ok(());
        }

        Err(Self::error(
            format!("Undefined variable '{}'.", value.to_string()),
            token.to_owned(),
        ))
    }

    fn error(message: String, token: Token) -> Box<dyn Error> {
        Box::new(
            LoxError::new()
                .type_(Box::new(RuntimeError))
                .message(message)
                .at_token(token),
        )
    }
}
