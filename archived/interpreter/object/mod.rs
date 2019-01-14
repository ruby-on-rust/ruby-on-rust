use std::collections::HashMap;

pub mod object_id;
pub mod name_tables;
pub mod class_value;

use crate::interpreter::{
    interpreter::Interpreter,
    object::{
        object_id::ObjectId,
        class_value::ClassValue,
    }
};

#[derive(Debug)]
pub enum Value {
    // 
    // primitive
    // 
    Nil,
    True,
    False,

    // 
    // frozen literal
    // 

    Class(ClassValue),

    // 
    // Object
    // 
    Object(ObjectId)
}

#[derive(Debug)]
pub struct Object {
    pub id: ObjectId,
    pub class: ObjectId,
    pub value: Value,
}

// 
// There's no Object::new, use ObjectSpace::add
// 
impl Object {
    // TODO refine with a to_bool trait/derive
    pub fn test_bool(&self) -> bool {
        match self.value {
            Value::Nil | Value::False => false,
            _ => true
        }
    }

    // 
    // class of object
    // 
    // TODO `rb_class_of` in cruby
    pub fn class(&self) -> ObjectId {
        match self.value {
            // Value::Nil => , TODO NilClass
            // Value::True => , TODO TrueClass
            // Value::False => , TODO FalseClass
            _ => { self.class }
        }
    }

    pub fn search_method(&self) {
        // TODO super
    }
}
