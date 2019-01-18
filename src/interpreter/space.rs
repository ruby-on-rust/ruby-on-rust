// 
// space holds and manages objects for interpreter, including:
// - to remember a top-level Object
// - to define class/const
// 

mod arena;
mod hierarchy;

use std::cell::{RefCell, RefMut};
use crate::interpreter::{
    object::{
        Object,
        obj_cell::ObjCell,
        value::Value
    },
    space::arena::Arena,
};

pub struct Space {
    arena: Arena,

    reserved_basic_object: ObjCell,
    reserved_object: ObjCell,
    reserved_module: ObjCell,
    reserved_class: ObjCell,
    reserved_nil: ObjCell,
}

impl Space {
    pub fn new() -> Space {
        Space {
            arena: Arena::new(),

            // pre-allocate placeholder obj cells
            reserved_basic_object: ObjCell::new_placeholder(),
            reserved_object: ObjCell::new_placeholder(),
            reserved_module: ObjCell::new_placeholder(),
            reserved_class: ObjCell::new_placeholder(),
            reserved_nil: ObjCell::new_placeholder(),
        }
    }

    // nil, true, false, and classes
    pub fn init_primitive_values(&mut self) {
        let nil_class = self.define_class("NilClass");
        let nil = Object::new(
            nil_class, Value::Nil
        );
        self.arena.add(nil);
    }

    // returns a nil
    pub fn nil(&self) -> ObjCell { self.reserved_nil.clone() }
}
