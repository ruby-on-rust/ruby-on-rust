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
        value,
        value::Value
    },
    space::arena::Arena,
};

pub struct Space {
    arena: Arena,

    reserved_basic_object: Oid,
    reserved_object: Oid,
    reserved_module: Oid,
    reserved_class: Oid,
    reserved_nil: Oid,
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
            reserved_nil: Oid::new(),
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
            value: Value::Class(
                value::class::Class {
                    superclass: None
                }
            )
        };

        let object = Object {
            id: self.reserved_object,
            class: self.reserved_class,
            value: value::new_class_value(self.reserved_basic_object)
        };

        let module = Object {
            id: self.reserved_module,
            class: self.reserved_class,
            value: value::new_class_value(self.reserved_object)
        };

        let class = Object {
            id: self.reserved_class,
            class: self.reserved_class,
            value: value::new_class_value(self.reserved_module)
        };

        self.arena.insert(basic_object);
        self.arena.insert(object);
        self.arena.insert(module);
        self.arena.insert(class);
    }

    // nil, true, false, and classes
    pub fn init_primitive_values(&mut self) {
        let nil_class = self.define_class("NilClass");

        let nil = Object {
            id: self.reserved_nil,
            class: nil_class,
            value: Value::Nil,
        };
        self.arena.insert(nil);
    }
}
