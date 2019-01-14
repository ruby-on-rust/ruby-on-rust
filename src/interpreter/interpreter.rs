
use crate::ast::node::SomeNode;

pub struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, node: SomeNode) -> () {
        panic!();
    }
}
