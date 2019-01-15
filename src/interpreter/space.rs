// 
// space holds and manages objects for interpreter, including:
// - to remember a top-level Object
// - to define class/const
// 

mod arena;
mod class;

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        value::{Value, new_class_value},
    },
    space::arena::Arena,
};

pub struct Space {
    arena: Arena,

    reserved_basic_object: Oid,
    reserved_object: Oid,
    reserved_module: Oid,
    reserved_class: Oid,
}

impl Space {
    pub fn new() -> Space {
        Space {
            arena: Arena::new(),

            // pre-generate ids
            reserved_basic_object: Oid::new(),
            reserved_object: Oid::new(),
            reserved_module: Oid::new(),
            reserved_class: Oid::new(),
        }
    }

    // 
    // primitive classes: BasicObject, Object, Class, Module
    // 
    // after this, we could init other classes, like NilClass, like normal
    // 
    pub fn init_primitive_classes(&mut self) {
        let basic_object = Object {
            id: self.reserved_basic_object,
            class: self.reserved_class,
            value: Value::Class {
                superclass: None
            }
        };

        let object = Object {
            id: self.reserved_object,
            class: self.reserved_class,
            value: Value::Class {
                superclass: Some(self.reserved_basic_object)
            }
        };

        let module = Object {
            id: self.reserved_module,
            class: self.reserved_class,
            value: Value::Class {
                superclass: Some(self.reserved_object)
            }
        };

        let class = Object {
            id: self.reserved_class,
            class: self.reserved_class,
            value: Value::Class {
                superclass: Some(self.reserved_module)
            }
        };

        self.arena.insert(basic_object);
        self.arena.insert(object);
        self.arena.insert(module);
        self.arena.insert(class);
    }

    // nil, true, false, and classes
    pub fn init_primitive_values(&mut self) {
        // let nil_class = self.def_class("NilClass");
    }
}
