use std::error::Error;

use crate::object::Object;

use crate::token::Token;
use crate::Interpreter;

pub trait Callable {
    fn call(
        &self,
        interpreter: Interpreter,
        arguments: Vec<Object>,
        paren: Token,
    ) -> Result<Object, Box<dyn Error>>;
}

impl Callable for Object {
    fn call(
        &self,
        mut interpreter: Interpreter,
        arguments: Vec<Object>,
        paren: Token,
    ) -> Result<Object, Box<dyn Error>> {
        match self {
            Object::Function(params, body) => {
                let (expected_len, found_len) = (params.len(), arguments.len());
                if expected_len != found_len {
                    return Err(interpreter.error(
                        &format!("Expected {} arguments but got {}.", expected_len, found_len),
                        &paren,
                    ));
                }

                let returned_val: Object = interpreter.execute_function(
                    interpreter.env.clone(),
                    (params.to_owned(), arguments),
                    body.clone(),
                )?;

                Ok(returned_val)
            }
            _ => Err(interpreter.error("Can only call functions and classes.", &paren)),
        }
    }
}
