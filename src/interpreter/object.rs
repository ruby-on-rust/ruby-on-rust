use uuid::Uuid;

use std::collections::HashMap;

use crate::interpreter::interpreter::Interpreter;

#[derive(Debug)]
enum Value {
    // 
    // primitive
    // 
    Nil,
    True,
    False,

    // 
    // frozon literal
    // 

    // 
    // Object
    // 
    Object(Box<Object>)
}

#[derive(Debug)]
pub struct Object {
    id: Uuid,
    value: Value,
}

impl Object {
    // pub fn new() -> Object {
    //     Object {
    //         id: Uuid::new_v4(),
    //         value: 
    //     }
    // }

    // 
    // primitive values
    // 
    pub fn new_nil() -> Object {
        Object {
            id: Uuid::new_v4(),
            value: Value::Nil,
        }
    }

    pub fn new_true() -> Object {
        Object {
            id: Uuid::new_v4(),
            value: Value::True,
        }
    }
}

pub struct ObjectSpace {
    objects: HashMap<Uuid, Object>
}

impl ObjectSpace {
    pub fn new() -> ObjectSpace {
        ObjectSpace {
            objects: HashMap::new()
        }
    }

    // pub fn add_object(&mut self) {
    //     let object = Object::new();

    //     self.objects.insert(object.id, object);
    // }
}
