// 
// Arena
// 

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        value::Value,
    },
    space::Space,
};

pub struct Arena { map: HashMap<Oid, RefCell<Object>> }

impl Arena {
    pub fn new() -> Arena {
        Arena { map: HashMap::new() }
    }

    pub fn insert(&mut self, object: Object) {
        self.map.insert(object.id, RefCell::new(object));
    }
}
