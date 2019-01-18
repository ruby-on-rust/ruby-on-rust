use crate::ast::node::{Node, SomeNode};
use crate::explainer;
use crate::interpreter::{
    object::obj_cell::ObjCell,
    space::Space
};

macro_rules! explain {
    ( $ ( $ arg : tt ) * ) => {
        let message = format!( $($arg)* );
        explainer::explain("interpreter", message);
    };
}

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
        explain!("evaluating node...");
        explain!("space status: {}", self.space);

        if node.is_none() { return self.space.nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { self.space.nil() },
            Node::True => { self.space.truthy() },
            Node::False => { self.space.falsey() },
            Node::Class { name, superclass, body } => {
                if let Node::Const { scope, name } = *name {
                    // TODO superclass, body
                    self.space.define_class(&name)
                } else { unreachable!() }
            },
            _ => { panic!("dont know how to eval node: {:?}", node); }
        }
    }
}
