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
    // A top-level class is a class whose superclass is Object, with the exception of BasicObject.
    // 
    // TODO
    // use a Result,
    // or, could defining a class failed?
    pub fn def_class(&mut self, name: String) -> Option<ObjectId> {
        let class_obj_value = Value::Class( ClassValue::new( self.root_object_id ) );
        let class_obj_id = self.add(class_obj_value);

        self.add_const_in_superclass(self.root_object_id, name, class_obj_id);

        Some(class_obj_id)
    }

    // TODO
    // use a Result,
    // or, could defining a class failed?
    pub fn def_nested_class(&mut self, name: String, superclass: ObjectId) -> Option<ObjectId> {
        // TODO assert superclass is actually a Class

        let class_obj_value = Value::Class( ClassValue::new( superclass ) );
        let class_obj_id = self.add(class_obj_value);

        self.add_const_in_superclass(superclass, name, class_obj_id);

        Some(class_obj_id)
    }

    fn add_const_in_superclass(&mut self, superclass: ObjectId, name: String, r#const: ObjectId) {
        if let Value::Class(ref mut class) = (*self.get(superclass)).value {
            class.consts_add(name, r#const);
        } else { unreachable!(); }
    }
}
