// 
// Arena
// 
// - to allocate new object
// 

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};
use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        obj_cell::ObjCell,
        value::Value,
    },
    space::Space,
};

pub struct Arena { cells: Vec<ObjCell> }

impl Arena {
    pub fn new() -> Arena {
        Arena { cells: vec![] }
    }

    // adds a object, returns a ObjCell
    pub fn add(&mut self, object: Object) -> ObjCell {
        let cell = ObjCell::new(object);
        self.cells.push(cell.clone());
        cell
    }
}
