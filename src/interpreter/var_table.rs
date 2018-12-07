use std::collections::HashMap;
use crate::interpreter::object::ObjectId;

#[derive(PartialEq, Eq, Hash)]
enum VarType { Global, Constant, Class, Instance, Local }

#[derive(PartialEq, Eq, Hash)]
pub struct Var {
    r#type: VarType,
    name: String,
}

pub type VarTable = HashMap<Var, ObjectId>;
