pub mod callable;

use std::{
    error::Error,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use std::cell::RefCell;
use std::rc::Rc;

use crate::{interpreter::Interpreter, stmt::Stmt, token::Token};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Function(Vec<Token>, Vec<Rc<RefCell<Box<dyn Stmt<Object>>>>>),
}

impl Object {
    pub fn is_nil(&self) -> bool {
        self == &Self::Nil
    }
}

impl Into<f64> for Object {
    fn into(self) -> f64 {
        match self {
            Object::Number(n) => n,
            _ => 0.0,
        }
    }
}

impl Into<bool> for Object {
    fn into(self) -> bool {
        match self {
            Object::Boolean(b) => b,
            _ => false,
        }
    }
}

impl Into<String> for Object {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            _ => String::from(""),
        }
    }
}

impl Into<String> for &Object {
    fn into(self) -> String {
        match self {
            Object::String(s) => s.to_string(),
            _ => String::from(""),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
            _ => Ok(()),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(s1), Object::String(s2)) => s1 == s2,
            (Object::Number(n1), Object::Number(n2)) => n1 == n2,
            (Object::Boolean(b1), Object::Boolean(b2)) => b1 == b2,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => n1.partial_cmp(n2),
            (Object::Boolean(b1), Object::Boolean(b2)) => b1.partial_cmp(b2),
            _ => None,
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(n1), Object::String(n2)) => {
                Object::String(n1.to_string() + n2.as_ref())
            }
            (Object::String(n1), Object::Number(n2)) => {
                Object::String(n1 + n2.to_string().as_ref())
            }
            (Object::Boolean(n1), Object::String(n2)) => {
                Object::String(n1.to_string() + n2.as_ref())
            }
            (Object::String(n1), Object::Boolean(n2)) => {
                Object::String(n1 + n2.to_string().as_ref())
            }
            (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 + n2),
            (Object::String(s1), Object::String(s2)) => Object::String(format!("{}{}", s1, s2)),
            _ => Object::Nil,
        }
    }
}

impl Sub for Object {
    type Output = Object;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 - n2),
            _ => Object::Nil,
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(_), Object::Number(0.0)) => Object::Nil,
            (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 / n2),
            _ => Object::Nil,
        }
    }
}

impl Mul for Object {
    type Output = Object;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 * n2),
            _ => Object::Nil,
        }
    }
}
