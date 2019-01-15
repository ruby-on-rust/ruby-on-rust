use std::collections::HashMap;
use crate::interpreter::object::oid::Oid;

pub struct Class {
    pub superclass: Option<Oid>,
    pub consts: HashMap<String, Oid>,
}

impl Class {
    pub fn add_const(&mut self, name: String, value: Oid) {
        self.consts.insert(name, value);
    }
}
