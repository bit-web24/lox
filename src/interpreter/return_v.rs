use core::error;

use crate::object::Object;

#[derive(Debug)]
pub struct Return {
    pub value: Object,
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {}", self.value)
    }
}

impl error::Error for Return {}
