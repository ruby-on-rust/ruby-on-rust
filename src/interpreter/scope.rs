use parser::ast;

use interpreter::interpreter::{Interpreter};
use interpreter::memory::{Refer};

#[derive(Debug)]
pub struct Var {
    refer: Refer
}

impl Interpreter {
    // TODO return a Result
    pub fn get_var_refer(&self, var_name: String) -> Refer {
        if let Some(var) = self.vars.get(&var_name) {
            return var.refer
        } else {
            return None
        }
    }

    // TODO return a Result
    pub fn assign_var(&mut self, var_name: String, refer: Refer) {
        // TODO REVISIT
        // should be easier after NNL is available
        // ```
        // if let Some(original_var) = self.vars.get_mut(&var_name) {
        // if let ori_var = self.vars.get_mut() { balah } else { balah }
        // ```
    
        if self.vars.contains_key(&var_name) {
            let original_var = self.vars.get_mut(&var_name).unwrap();
            original_var.refer = refer;
        } else {
            self.vars.insert(var_name, Var{ refer } );
        }

        // TODO CLEANUP
        println!("var assigned, current vars:");
        println!("{:?}", self.vars);
    }
}
