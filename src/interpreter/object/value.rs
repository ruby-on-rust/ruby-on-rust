pub mod class;

use crate::interpreter::object::oid::Oid;

pub enum Value {
    Nil,

    Class(class::Class)
}

pub fn new_class_value(superclass: Oid) -> Value {
    Value::Class(
        class::Class {
            superclass: Some(superclass)
        }
    )
}
