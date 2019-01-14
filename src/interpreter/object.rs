pub mod oid;
pub mod value;

use crate::interpreter::object::{
    oid::Oid,
    value::Value
};

pub struct Object {
    pub id: Oid,
    pub class: Oid,
    pub value: Value
}

impl Object {
    pub fn new(class: Oid, value: Value) -> Object {
        Object {
            id: Oid::new(),
            class,
            value
        }
    }
}
