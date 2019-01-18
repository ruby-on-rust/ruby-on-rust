pub mod class;

use std::collections::HashMap;
use crate::interpreter::object::obj_cell::ObjCell;

pub enum Value {
    Nil,

    Class(class::Class)
}

pub fn new_class_value(superclass: ObjCell) -> Value {
    Value::Class(
        class::Class {
            superclass: Some(superclass),
            consts: HashMap::new()
        }
    )
}
