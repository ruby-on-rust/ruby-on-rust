use parser::ast;

use super::interpreter::{Interpreter};

pub type Refer = Option<u64>;

// TODO maybe make Value a struct
// TODO implement `primitive`
// TODO implement objects
#[derive(Debug)]
pub enum Value {
    Number(i64)
}

impl Interpreter {
    // allocate a new slot in memory, save the value, and return the assigned address
    // TODO return a Result
    pub fn memory_allocate(&mut self, value: Value) -> Refer {
        self.memory.push(value);

        // TODO CLEANUP, or at least use that logging crate
        println!("allocated memory, current:");
        println!("{:?}", self.memory);

        return Some(( self.memory.len() - 1 ) as u64)
    }

    // TODO separate to Ref
    // TODO handle nil
    // TODO return a Result<Option<>>
    pub fn get_value_from_refer(&self, refer: Refer) -> &Value {
        // TODO CLEANUP, or at least use that logging crate
        println!("getting value of refer:");
        println!("{:?}", refer);

        if let Some(addr) = refer {
            return &self.memory[addr as usize]
        } else {
            unimplemented!()
        }
    }
}
