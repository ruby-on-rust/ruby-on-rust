// 
// space holds and manages objects for interpreter, including:
// - to remember a top-level Object
// - to allocate new object
// - to define class/const
// 
// TODO separate arena

mod arena;

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use crate::interpreter::{
    object::{
        Object,
        oid::Oid,
        value::Value,
    },
    space::arena::Arena,
};

pub struct Space {
    arena: Arena
}

impl Space {
    pub fn new() -> Space {
        let mut arena = Arena::new();

        // 
        // primitive classes: BasicObject, Object, Class, Module
        // 

        // pre-generate ids
        let (basic_object_id, object_id, module_id, class_id) = (
            Oid::new(), Oid::new(), Oid::new(), Oid::new());

        let basic_object = Object {
            id: basic_object_id,
            class: class_id,
            value: Value::Class {
                superclass: None
            }
        };

        let object = Object {
            id: object_id,
            class: class_id,
            value: Value::Class {
                superclass: Some(basic_object_id)
            }
        };

        let module = Object {
            id: module_id,
            class: class_id,
            value: Value::Class {
                superclass: Some(object_id)
            }
        };

        let class = Object {
            id: class_id,
            class: class_id,
            value: Value::Class {
                superclass: Some(module_id)
            }
        };

        arena.insert(basic_object);
        arena.insert(object);
        arena.insert(module);
        arena.insert(class);

        Space {
            arena
        }
    }
}
