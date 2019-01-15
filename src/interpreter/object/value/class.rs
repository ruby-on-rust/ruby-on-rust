use crate::interpreter::object::oid::Oid;

pub struct Class {
    pub superclass: Option<Oid>,
}
