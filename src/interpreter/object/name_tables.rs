use std::collections::HashMap;
use crate::interpreter::object::ObjectId;

pub type ConstTable = HashMap<String, ObjectId>;

pub trait ConstsOwning {
    fn consts_add(&mut self, name: String, object: ObjectId);
}
