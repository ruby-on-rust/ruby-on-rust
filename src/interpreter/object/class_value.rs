use crate::{
    interpreter::{
        object::{
            Value, Object, ObjectId,
            name_tables::{
                ConstTable, ConstsOwning
            }
        },
    }
};

#[derive(Debug)]
pub struct ClassValue {
    pub superclass: Option<ObjectId>,
    pub consts: ConstTable,
}

impl ClassValue {
    pub fn new(superclass: ObjectId) -> ClassValue {
        ClassValue {
            superclass: Some(superclass),
            consts: ConstTable::new(),
        }
    }
}

impl ConstsOwning for ClassValue {
    fn consts_add(&mut self, name: String, object: ObjectId) {
        self.consts.insert(name, object);
    }
}
