use std::collections::HashMap;
use crate::{
    ast::node::{Node, SomeNode},
    interpreter::{
        object::{Value, Object, ObjectId},
        object_space::{ObjectSpace},
    }
};

pub struct Interpreter {
    objects: ObjectSpace,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            objects: ObjectSpace::new(),
        };

        interpreter
    }

    pub fn eval(&mut self, node: SomeNode) -> &Object {
        if node.is_none() { return self.objects.get_primitive_nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { self.objects.get_primitive_nil() },
            Node::True => { self.objects.get_primitive_true() },
            Node::False => { self.objects.get_primitive_false() },

            Node::Class { name, superclass, body } => {
                if let Node::Const { scope, name } = *name {
                    // TODO superclass

                    let class = self.objects.def_class(name).unwrap();
                    self.objects.get(class)
                } else { unreachable!() }
            }

            Node::If {condition, then_body, else_body} => {
                if self.eval(Some(*condition)).test_bool() { 
                    self.eval(*then_body)
                } else {
                    self.eval(*else_body)
                }
            },

            _ => { panic!("eval: don't know how to handle node: {:?}", node); }
        }
    }
}
