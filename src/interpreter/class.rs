use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{Value, Object, ObjectId},
        object_space::{ObjectSpace},
    }
};

impl ObjectSpace {
    fn add_class_to_name_table() {
        // TODO
    }

    pub fn predefine_classes(&mut self) {
        // BasicObject
        let basic_object = Object::new(Value::Class {
            superclass: None,
        });

        self.add(basic_object);

        // TODO add to name table

        // TODO Object < BasicObejct
        // TODO Module < Object
        // TODO Class < Module
    }

    pub fn define_class(&mut self, name: String, superclass: Object) -> Option<ObjectId> { // TODO use a Result
        // TODO assert superclass is actually a Class

        // create class object
        let class_obj = Object::new(Value::Class { superclass: Some(superclass.id) });

        // TODO add to name table

        Some(class_obj.id)
    }
}
