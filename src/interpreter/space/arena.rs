// 
// Arena
// 
// - to allocate new object
// 

use std::fmt;
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

impl fmt::Display for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = self.cells.iter()
            .map(|o| format!("{}", o))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", msg)
    }
}

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
