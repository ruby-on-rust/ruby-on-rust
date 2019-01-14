// TODO move this file to reflect the structure (> ObjectSpace)
use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{
            object_id::{ ObjectId },
            Value, Object,
            name_tables::ConstsOwning,
            class_value::ClassValue
        },
        object_space::{ObjectSpace},
    }
};

impl ObjectSpace {
    // 
    // Defines a top-level class
    // 
    // A top-level class is a class whose superclass is Object, with the exception of BasicObject, which has nil as superclass
    // 
    // Under the hood, to define a top-level class:
    // 1) coin the actually ClassValue: name being the given one, and superclass being Object
    // 2) create an Object in space, with value being the ClassValue
    // 3) add a const on Object('s ClassValue) and link to the 
    // 
    // TODO use a Result
    pub fn def_class(&mut self, name: String) -> Option<ObjectId> {

        // TODO MAYBE make a macro to add obj to space and return an (id, value) tuple
        let class_obj_value = Value::Class( ClassValue::new( self.primitive_object ) );
        let class_obj_id = self.add(class_obj_value);

        if let Value::Class(mut class) = self.get_primitive_object().value {
            class.consts_add(name, class_obj_id);
        }

        // let primitive_obj = self.
        // self.add_const_in_superclass(self.root_object_id, name, class_obj_id);

        Some(class_obj_id)
    }
}
