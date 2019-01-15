use crate::ast::node::SomeNode;
use crate::interpreter::space::Space;

pub struct Interpreter {
    space: Space
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut space = Space::new();

        space.init_primitive_classes();
        space.init_primitive_values();

        Interpreter {
            space
        }
    }

    pub fn eval(&mut self, node: SomeNode) -> () {
        panic!();
    }
}
