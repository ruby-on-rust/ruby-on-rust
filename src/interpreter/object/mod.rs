use uuid::Uuid;

use std::collections::HashMap;

pub mod name_tables;
pub mod class_value;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::object::class_value::ClassValue;

// 
// object id and utility
// 
pub type ObjectId = Uuid;
pub fn new_obj_id() -> ObjectId { Uuid::new_v4() }

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
