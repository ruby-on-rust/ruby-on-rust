use crate::ast::node::SomeNode;
use crate::interpreter::space::Space;

pub struct Interpreter {
    space: Space
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            space: Space::new()
        }
    }

    pub fn eval(&mut self, node: SomeNode) -> () {
        panic!();
    }
}
