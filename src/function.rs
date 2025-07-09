use crate::callable::Callable;
use crate::class::instance::Instance;
use crate::env::Environment;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
use crate::token::Token;

use crate::token::token_type::TokenType;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
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

    pub fn bind(&self, instance: &Instance) -> Result<Object, Box<dyn Error>> {
        let this_tkn = Token::new(TokenType::THIS, "this".to_string(), None, -1);
        let environment = Rc::new(RefCell::new(Environment::from(self.closeure.clone())));

        let instance_: Rc<RefCell<Instance>> = Rc::new(RefCell::new(instance.clone()));
        environment
            .borrow_mut()
            .define(&this_tkn, Object::Instance(instance_))?;

        let func = Function::new(self.declaration.clone(), environment);
        Ok(Object::Function(Some(Rc::new(RefCell::new(func))), None))
    }
}

impl Callable for Function {
    fn call(
        &self,
        mut interpreter: Interpreter,
        arguments: Vec<Object>,
        _paren: Token,
    ) -> Result<Object, Box<dyn Error>> {
        use crate::interpreter::return_v::Return;

        let environment = Rc::new(RefCell::new(Environment::from(self.closeure.clone())));
        for (param, arg) in self.declaration.params.iter().zip(arguments.iter()) {
            environment.borrow_mut().define(param, arg.clone())?;
        }
        interpreter.env = environment.clone();
        let result = interpreter.execute_block(self.declaration.body.clone(), environment);
        match result {
            Ok(_) => Ok(Object::Nil),
            Err(e) if e.is::<Return>() => {
                let return_value = e.downcast::<Return>().unwrap();
                Ok(return_value.value)
            }
            Err(e) => Err(e),
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}

// Avoids printing infinite refencing output
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            // .field("declaration", &self.declaration)
            .field("declaration", &"<Declaration>")
            .field("closeure", &"<Environment>")
            .finish()
    }
}
