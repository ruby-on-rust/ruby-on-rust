use std::collections::HashMap;
use crate::interpreter::object::oid::Oid;

pub struct Class {
    pub superclass: Option<Oid>,
    pub consts: HashMap<String, Oid>,
}
