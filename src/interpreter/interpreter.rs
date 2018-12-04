use crate::{
    ast::node::Node,
    interpreter::object::{Object, ObjectSpace},
};

pub struct Interpreter {
    object_space: ObjectSpace
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            object_space: ObjectSpace::new(),
        }
    }

    pub fn eval(&mut self, node: Option<Node>) -> Object {
        if node.is_none() { return Object::new_nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { Object::new_nil() },
            Node::True => { Object::new_true() },
            _ => { panic!("eval: don't know how to handle node: {:?}", node); }
        }
    }
}
