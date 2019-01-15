// 
// class defining related functions for Space
// 

use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        value,
    },
    space::Space
};

impl Space {
    // 
    // defines a top-level class
    // 
    // TODO
    // - class's name
    // 
    pub fn define_class(&mut self, name: &str) -> Oid {
        let object = Object {
            id: Oid::new(),
            class: self.reserved_class,
            value: value::new_class_value(self.reserved_object)
        };

        self.arena.insert(object)
    }
}
