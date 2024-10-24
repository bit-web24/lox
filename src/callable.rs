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

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        format!("<native fn>")
    }
}

pub fn get_native_functions() -> Vec<(&'static str, Object)> {
    vec![("clock", Object::Function(vec![], None, Some(clock)))]
}

fn clock(argv: Vec<Object>) -> Result<Object, Box<dyn Error>> {
    let argc = argv.len();
    if argc != 0 {
        return Err(format!("Expected 0 arguments found {} arguments", argc).into());
    }

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    Ok(Object::Number(current_time))
}
