use crate::expr::{Expr, Variable};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ExprKey {
    pub expr: Rc<Box<dyn Expr>>,
}

impl PartialEq for ExprKey {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(x), Some(y)) = (
            self.expr.as_any().downcast_ref::<Variable>(),
            other.expr.as_any().downcast_ref::<Variable>(),
        ) {
            x.name.lexeme == y.name.lexeme && x.name.line == y.name.line
        } else {
            false
        }
    }
}

impl Eq for ExprKey {}

impl Hash for ExprKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(var) = self.expr.as_any().downcast_ref::<Variable>() {
            var.name.lexeme.hash(state);
            var.name.line.hash(state);
        }
    }
}
