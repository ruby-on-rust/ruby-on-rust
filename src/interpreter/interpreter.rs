use crate::{
    ast::node::{Node, SomeNode},
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

    pub fn eval(&mut self, node: SomeNode) -> Object {
        if node.is_none() { return Object::new_nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { Object::new_nil() },
            Node::True => { Object::new_true() },

            Node::If {condition, then_body, else_body} => {
                if self.test_obj_bool(self.eval(Some(*condition))) { // TODO macro for this case
                    self.eval(*then_body)
                } else {
                    self.eval(*else_body)
                }
            },

            _ => { panic!("eval: don't know how to handle node: {:?}", node); }
        }
    }

    fn test_obj_bool(&self, object: Object) -> bool {
        true
    }
}
