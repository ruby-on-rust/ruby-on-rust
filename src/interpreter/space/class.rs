// 
// class defining related functions for Space
// 

use crate::interpreter::{
    object::oid::Oid,
    space::Space
};

impl Space {
    pub fn define_class(&mut self, name: String, superclass: Oid) -> Oid {
        panic!();
    }
}
