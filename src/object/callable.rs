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
            Object::Function(argv, body) => {
                let (expected_len, found_len) = (argv.len(), arguments.len());
                if expected_len != found_len {
                    return Err(interpreter.error(
                        &format!("Expected {} arguments but got {}.", expected_len, found_len),
                        &paren,
                    ));
                }

                interpreter.execute(body.clone())?;
                // need to return the "return value from function".
                Ok(Object::Nil)
            }
            _ => Err(interpreter.error("Can only call functions.", &paren)),
        }
    }
}
