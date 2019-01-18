use std::rc::{Rc};
use std::cell::{RefCell};

use crate::interpreter::object::Object;

#[derive(Clone)]
pub struct ObjCell(
    pub Rc<RefCell<Option<Object>>>
);

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
    pub fn replace_placeholder_with(&mut self, obj: Object) {
        assert!(self.0.borrow().is_none());
        (*self.0).replace(Some(obj));
    }

    // // 
    // // panic on placeholder
    // // 
    // pub fn borrow_mut(&mut self) -> &mut Object {
    //     &mut self.0.get_mut().unwrap()
    // }
}
