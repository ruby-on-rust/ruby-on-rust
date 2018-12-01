use crate::{
    ast::node::Node,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, node: Option<Node>) {
    }
}
