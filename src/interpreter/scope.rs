use std::collections::HashMap;

use interpreter::memory::{Refer};

#[derive(Debug)]
pub struct LocalVar {
    pub refer: Refer
}

pub struct Scope {
    pub local_vars: HashMap<String, LocalVar>,
}

// TODO
// assigning and fetching local vars should be a scope's job
impl Scope {
    pub fn new() -> Scope {
        Scope {
            local_vars: HashMap::new()
        }
    }
}
