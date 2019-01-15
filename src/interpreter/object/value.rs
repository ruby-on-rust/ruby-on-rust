// TODO struct ClassValue?

use crate::interpreter::object::oid::Oid;

pub enum Value {
    Nil,

    // TODO None or nil for BasicObject's superclass?
    Class { superclass: Option<Oid> }
}

pub fn new_class_value(superclass: Oid) -> Value {
    Value::Class {
        superclass: Some(superclass)
    }
}
