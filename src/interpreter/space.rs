// 
// space holds and manages objects for interpreter, including:
// - to remember a top-level Object
// - to define class/const
// 

mod arena;
mod hierarchy;

use std::fmt;
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
    reserved_true: ObjCell,
    reserved_false: ObjCell,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arena:\n");
        write!(f, "{}\n", self.arena);
        write!(f, "reserved_basic_object: {}\n", self.reserved_basic_object);
        write!(f, "reserved_object: {}\n", self.reserved_object);
        write!(f, "reserved_module: {}\n", self.reserved_module);
        write!(f, "reserved_class: {}\n", self.reserved_class);
        write!(f, "reserved_nil: {}\n", self.reserved_nil);
        write!(f, "reserved_true: {}\n", self.reserved_true);
        write!(f, "reserved_false: {}\n", self.reserved_false)
    }
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
            reserved_true: ObjCell::new_placeholder(),
            reserved_false: ObjCell::new_placeholder(),
        }
    }

    // nil, true, false, and classes
    pub fn init_primitive_values(&mut self) {
        // nil and NilClass
        let nil_class = self.define_class("NilClass");
        let nil = Object::new(
            nil_class, Value::Nil
        );
        self.reserved_nil.replace_placeholder_with(nil);

        // true and TrueClass
        let true_class = self.define_class("TrueClass");
        let r#true = Object::new(
            true_class, Value::True
        );
        self.reserved_true.replace_placeholder_with(r#true);

        // false and FalseClass
        let false_class = self.define_class("FalseClass");
        let r#false = Object::new(
            false_class, Value::False
        );
        self.reserved_false.replace_placeholder_with(r#false);
    }

    pub fn nil(&self) -> ObjCell { self.reserved_nil.clone() }
    pub fn truthy(&self) -> ObjCell { self.reserved_true.clone() }
    pub fn falsey(&self) -> ObjCell { self.reserved_false.clone() }
}
