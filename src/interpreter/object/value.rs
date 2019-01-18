pub mod class;

use std::fmt;
use std::collections::HashMap;
use crate::interpreter::object::obj_cell::ObjCell;

#[derive(Display)]
pub enum Value {
    Nil,
    True,
    False,

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
