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
    pub fn new(value: Value, class: Oid) -> Object {
        Object {
            id: oid::new_oid(),
            class,
            value
        }
    }
}
