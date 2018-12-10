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
    // frozen literal
    // 

    Class {
        superclass: Option<ObjectId>,
    },

    // 
    // Object
    // 
    Object(ObjectId)
}

#[derive(Debug)]
pub struct Object {
    pub id: ObjectId,
    pub value: Value,
}

// 
// There's no Object::new, use ObjectSpace::add
// 
impl Object {
    // TODO refinen with a to_bool trait/derive
    pub fn test_bool(&self) -> bool {
        match self.value {
            Value::Nil | Value::False => false,
            _ => true
        }
    }
}
