// 
// space holds and manages objects for interpreter, including:
// - to remember a top-level Object
// - to allocate new object
// - to define class/const
// 

pub struct Space {
}

impl Space {
    pub fn new() -> Space {
        Space {}
    }
}
