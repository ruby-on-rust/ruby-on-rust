use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{
            Value, Object, ObjectId,
            name_tables::ConstsOwning,
            class_value::ClassValue
        },
        object_space::{ObjectSpace},
    }
};

impl ObjectSpace {
    pub fn predefine_classes(&mut self) {
        // Module
        let module_id = self.def_class(String::from("Module")).unwrap();

        // Class
        // Class is under Module, but is as well in constants of Object
        // TODO nested under module
        // self.def_class(String::from("Class"), module_id);
    }

    // 
    // Defines a top-level class
    // 
    // A top-level class is a class whose superclass is Object, with the exception of BasicObject.
    // 
    pub fn def_class(&mut self, name: String) -> Option<ObjectId> { // TODO use a Result
        let class_obj_value = Value::Class( ClassValue::new( self.root_object_id ) );
        let class_obj_id = self.add(class_obj_value);

        self.root_obj_consts_add(name, class_obj_id);

        Some(class_obj_id)
    }

    pub fn def_nested_class(&mut self, name: String) {
        // superclass: ObjectId
        // TODO assert superclass is actually a Class
        panic!();
    }

    fn root_obj_consts_add(&mut self, name: String, r#const: ObjectId) {
        if let Value::Class(ref mut class) = (*self.get_root_obj()).value {
            class.consts_add(name, r#const);
        } else { unreachable!(); }
    }
}
