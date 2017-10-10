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

    // TODO
    // distinct localvar, global var, etc
    // return a Result
    pub fn fetch_local_var_refer(&mut self, var_name: String) -> Refer {
        if let Some(var) = self.local_vars.get(&var_name) {
            return var.refer
        } else {
            return None
        }
    }

    // TODO
    // distinct localvar, global var, etc
    // return a Result
    pub fn assign_local_var(&mut self, var_name: String, refer: Refer) {
        // TODO REVISIT
        // should be easier after NNL is available
        // ```
        // if let Some(original_var) = self.local_vars.get_mut(&var_name) {
        // if let ori_var = self.local_vars.get_mut() { balah } else { balah }
        // ```

        if self.local_vars.contains_key(&var_name) {
            let original_var = self.local_vars.get_mut(&var_name).unwrap();
            original_var.refer = refer;
        } else {
            self.local_vars.insert(var_name, LocalVar{ refer } );
        }

        // TODO CLEANUP
        println!("var assigned, current vars:");
        println!("{:?}", self.local_vars);
    }
}
