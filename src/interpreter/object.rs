use uuid::Uuid;
use std::collections::HashMap;
use crate::interpreter::interpreter::Interpreter;

pub type ObjectId = Uuid;

#[derive(Debug)]
pub enum Value {
    // 
    // primitive
    // 
    Nil,
    True,
    False,

    // 
    // frozon literal
    // 

    Class {
        super_class: Box<Object>,
    },

    // 
    // Object
    // 
    Object(Box<Object>)
}

#[derive(Debug)]
pub struct Object {
    pub id: ObjectId,
    pub value: Value,
}

impl Object {
    pub fn new(value: Value) -> Object {
        Object {
            id: Uuid::new_v4(),
            value
        }
    }

    pub fn test_bool(&self) -> bool {
        match self.value {
            Value::Nil | Value::False => false,
            _ => true
        }
    }
}
