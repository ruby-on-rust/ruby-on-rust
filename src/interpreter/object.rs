mod oid;
mod value;

use crate::interpreter::object::{
    oid::Oid,
    value::Value
};

pub struct Object {
    id: Oid,
    value: Value
}

impl Object {
    pub fn new(value: Value) -> Object {
        Object {
            id: oid::new_oid(),
            value
        }
    }
}
