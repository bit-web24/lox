use crate::expr::Expr;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ExprKey {
    pub expr: Rc<Box<dyn Expr>>,
}

impl PartialEq for ExprKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.expr, &other.expr)
    }
}

impl Eq for ExprKey {}

impl Hash for ExprKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use the address of the Rc as a unique identifier
        let address = Rc::as_ptr(&self.expr) as *const ();
        address.hash(state);
    }
}
