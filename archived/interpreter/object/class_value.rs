// TODO maybe rename to ClassObject

use crate::{
    ast::node::Node,
    interpreter::{
        object::{
            Value, Object, ObjectId,
            name_tables::{
                ConstTable, ConstsOwning, MethodTable, MethodsOwning
            }
        },
    }
};

#[derive(Debug)]
pub struct ClassValue {
    pub superclass: Option<ObjectId>,
    pub singleton: bool,
    pub consts: ConstTable,
    pub methods: MethodTable,
}

impl ClassValue {
    // 
    // creates a normal Class
    // 
    pub fn new(superclass: ObjectId) -> ClassValue {
        ClassValue {
            superclass: Some(superclass),
            singleton: false,
            consts: ConstTable::new(),
            methods: MethodTable::new(),
        }
    }

    pub fn new_singleton(superclass: ObjectId) -> ClassValue {
        ClassValue {
            superclass: Some(superclass),
            singleton: true,
            consts: ConstTable::new(),
            methods: MethodTable::new(),
        }
    }

    // TODO cleanup after we remove Option for superclass
    pub fn new_basic_object() -> ClassValue {
        ClassValue { superclass: None, singleton: false, consts: ConstTable::new(), methods: MethodTable::new() }
    }
}

impl ConstsOwning for ClassValue {
    fn consts_add(&mut self, name: String, object: ObjectId) {
        self.consts.insert(name, object);
    }
}

impl MethodsOwning for ClassValue {
    fn methods_add(&mut self, name: String, method: ObjectId) {
        // TODO assert method is a Method

        self.methods.insert(name, method);
    }
}
