// 
// class defining related functions for Space
// 

use std::rc::Rc;
use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        value, value::Value,
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
        let new_class = self.arena.insert(Object {
            id: Oid::new(),
            class: self.reserved_class,
            value: value::new_class_value(self.reserved_object)
        });

        // TODO separate
        // object.as_class_add_const
        let mut object = self.arena.get(self.reserved_object);
        if let Value::Class(class_value) = &mut object.borrow_mut().value {
            class_value.add_const(name.to_string(), new_class);
        }

        new_class
    }
}
