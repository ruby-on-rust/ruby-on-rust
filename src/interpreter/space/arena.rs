// 
// Arena
// 
// - to allocate new object
// - to get object from id
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

    // 
    // returns the inserted object's id
    // 
    // panic! if the key exists
    // 
    // TODO refine this panicing
    // 
    pub fn insert(&mut self, object: Object) -> Oid {
        // save the id
        let id = object.id;

        if self.map.contains_key(&object.id) {
            panic!();
        }

        self.map.insert(object.id, RefCell::new(object));

        id
    }
}
