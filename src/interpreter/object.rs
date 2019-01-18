pub mod oid;
pub mod obj_cell;
pub mod value;

use std::fmt;
use crate::interpreter::object::{
    oid::Oid,
    obj_cell::ObjCell,
    value::Value
};

pub struct Object {
    pub id: Oid,
    pub class: ObjCell,
    pub value: Value
}

impl Object {
    pub fn new(class: ObjCell, value: Value) -> Object {
        Object {
            id: Oid::new(),
            class,
            value
        }
    }
}
