use std::{fmt, ops::{Add, Div, Mul, Sub}};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
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
            (Object::String(s1), Object::String(s2)) => s1.partial_cmp(s2),
            (Object::Number(n1), Object::Number(n2)) => n1.partial_cmp(n2),
            (Object::Boolean(b1), Object::Boolean(b2)) => b1.partial_cmp(b2),
            (Object::Nil, Object::Nil) => Some(std::cmp::Ordering::Equal),
            _ => None,
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 + n2),
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