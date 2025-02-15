use crate::callable::Callable;
use crate::class::Class;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Instance {
    class: Rc<RefCell<Class>>,
}

impl Instance {
    pub fn new(class: Rc<RefCell<Class>>) -> Self {
        Self { class }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().to_string())
    }
}
