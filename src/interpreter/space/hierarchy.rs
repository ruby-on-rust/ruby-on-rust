// 
// class defining related functions for Space
// 

use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::{
    object::{
        Object,
        obj_cell::ObjCell,
        value, value::Value,
    },
    space::Space
};

impl Space {
    // 
    // primitive classes: BasicObject, Object, Class, Module
    // 
    // after this, we could init other classes, like NilClass, like normal
    // 
    pub fn init_hierarchy(&mut self) {
        let basic_object = Object::new(
            self.reserved_class.clone(),
            Value::Class(
                value::class::Class {
                    superclass: None,
                    consts: HashMap::new()
                }
            )
        );
        self.reserved_basic_object.replace_placeholder_with(basic_object);
        // TODO add top-level const

        let object = Object::new(
            self.reserved_class.clone(),
            value::new_class_value(self.reserved_basic_object.clone())
        );
        self.reserved_object.replace_placeholder_with(object);
        // TODO add top-level const

        // TODO use self.define_nested_class
        let module = Object::new(
            self.reserved_class.clone(),
            value::new_class_value(self.reserved_object.clone())
        );
        self.reserved_module.replace_placeholder_with(module);
        // TODO add top-level const

        // TODO use self.define_nested_class
        let class = Object::new(
            self.reserved_class.clone(),
            value::new_class_value(self.reserved_module.clone())
        );
        self.reserved_class.replace_placeholder_with(class);
        // TODO add top-level const
    }

    // 
    // defines a normal top-level class
    // 
    // TODO DOC
    // 
    pub fn define_class(&mut self, name: &str) -> ObjCell {
        let new_class_cell = {
            let new_class = Object::new(
                self.reserved_class.clone(),
                value::new_class_value(self.reserved_object.clone())
            );
            self.arena.add(new_class)
        };

        // TODO separate
        // object.as_class_add_const
        let mut object_cell = self.reserved_object.0.borrow_mut();
        let mut object = object_cell.as_mut().unwrap();
        if let Value::Class(class_value) = &mut object.value {
            class_value.add_const(name.to_string(), new_class_cell.clone());
        }

        new_class_cell
    }
}
