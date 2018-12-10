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
        let basic_object_id = self.add(Value::Class {
            superclass: None,
        });

        // TODO add to name table

        let object_id = self.define_class(String::from("Object"), basic_object_id).unwrap();
        let module_id = self.define_class(String::from("Module"), object_id).unwrap();
        self.define_class(String::from("Class"), module_id);
    }

    pub fn define_class(&mut self, name: String, superclass: ObjectId) -> Option<ObjectId> { // TODO use a Result
        // TODO assert superclass is actually a Class

        // create class object
        let class_obj_value = Value::Class { superclass: Some(superclass) };
        let class_obj_id = self.add(class_obj_value);

        // TODO add to name table

        Some(class_obj_id)
    }
}
