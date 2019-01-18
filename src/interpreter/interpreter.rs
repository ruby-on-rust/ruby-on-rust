use crate::ast::node::{Node, SomeNode};
use crate::interpreter::{
    object::obj_cell::ObjCell,
    space::Space
};

pub struct Interpreter {
    space: Space
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut space = Space::new();

        space.init_hierarchy();
        space.init_primitive_values();

        Interpreter {
            space
        }
    }

    pub fn eval(&mut self, node: SomeNode) -> ObjCell {
        if node.is_none() { return self.space.nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { self.space.nil() },
            _ => { panic!("dont know how to eval node: {:?}", node); }
        }
    }
}
