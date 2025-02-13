use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use crate::{
    error::{error_types::RuntimeError, LoxError},
    object::Object,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(enclosing: Rc<RefCell<Self>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object, Box<dyn Error>> {
        if let Some(value) = self.values.get(&token.lexeme) {
            return Ok(value.clone());
        }

        Err(Self::error(
            format!("Undefined variable '{}'.", token.lexeme),
            token.clone(),
        ))
    }

    pub fn get_at(&self, distance: i32, name: String) -> Result<Object, Box<dyn Error>> {
        let env = self.ancestor(distance)?;
        if let Some(val) = env.borrow().values.get(&name) {
            return Ok(val.to_owned());
        }

        Ok(Object::Nil)
    }

    pub fn ancestor(&self, distance: i32) -> Result<Rc<RefCell<Environment>>, Box<dyn Error>> {
        let mut environ = Rc::new(RefCell::new(self.clone()));

        for _ in 0..distance {
            let x = environ.borrow().enclosing.clone();
            environ = x.unwrap();
        }

        Ok(environ)
    }

    pub fn define(&mut self, token: &Token, value: Object) -> Result<(), Box<dyn Error>> {
        if !self.values.contains_key(token.lexeme.as_str()) {
            self.values.insert(token.lexeme.to_owned(), value);
            return Ok(());
        }

        Err(Self::error(
            "variable already defined.".into(),
            token.to_owned(),
        ))
    }

    pub fn assign(&mut self, token: &Token, value: &Object) -> Result<(), Box<dyn Error>> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.clone(), value.to_owned());
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        Err(Self::error(
            format!("Undefined variable '{}'.", token.lexeme),
            token.clone(),
        ))
    }

    pub fn assign_at(
        &self,
        distance: i32,
        name: &Token,
        value: &Object,
    ) -> Result<(), Box<dyn Error>> {
        self.ancestor(distance)?
            .borrow_mut()
            .values
            .insert(name.lexeme.clone(), value.to_owned());
        Ok(())
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
