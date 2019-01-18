use std::fmt;
use std::rc::{Rc};
use std::cell::{RefCell};

use crate::interpreter::object::Object;

#[derive(Clone)]
pub struct ObjCell(
    pub Rc<RefCell<Option<Object>>>
);

impl fmt::Display for ObjCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // panic on placeholder
        assert!(self.0.borrow().is_some());

        write!(f, "ObjCell -> {}", self.0.borrow().as_ref().unwrap().id)
    }
}

impl ObjCell {
    pub fn new(obj: Object) -> ObjCell {
        ObjCell(Rc::new(RefCell::new(Some(obj))))
    }

    // 
    // placeholder mechanics is used for bootstrapping
    // 
    pub fn new_placeholder() -> ObjCell {
        ObjCell(Rc::new(RefCell::new(None)))
    }

    // 
    // replace the placeholder with a real value
    // 
    // panics unless is placeholder
    // 
    pub fn replace_placeholder_with(&mut self, obj: Object) {
        assert!(self.0.borrow().is_none());
        (*self.0).replace(Some(obj));
    }
}
