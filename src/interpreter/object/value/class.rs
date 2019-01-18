use std::collections::HashMap;
use crate::interpreter::object::obj_cell::ObjCell;

pub struct Class {
    pub superclass: Option<ObjCell>,
    pub consts: HashMap<String, ObjCell>,
}

impl Class {
    pub fn add_const(&mut self, name: String, value: ObjCell) {
        self.consts.insert(name, value);
    }
}
