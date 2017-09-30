pub type Refer = Option<u64>;

// TODO maybe make Value a struct
// TODO implement `primitive`
// TODO implement objects
#[derive(Debug)]
pub enum Value {
    Number(i64)
}

#[derive(Default)]
pub struct Memory {
    space: Vec<Value>
}

impl Memory {
    // allocate a new slot in memory, save the value, and return the assigned address
    // TODO return a Result
    pub fn allocate_value(&mut self, value: Value) -> Refer {
        self.space.push(value);

        // TODO CLEANUP, or at least use that logging crate
        println!("allocated memory, current:");
        println!("{:?}", self.space);

        return Some(( self.space.len() - 1 ) as u64)
    }

    // TODO handle nil
    // TODO return a Result<Option<>>
    pub fn get_value_from_refer(&self, refer: Refer) -> &Value {
        // TODO CLEANUP, or at least use that logging crate
        println!("getting value of refer:");
        println!("{:?}", refer);

        if let Some(addr) = refer {
            return &self.space[addr as usize]
        } else {
            unimplemented!()
        }
    }
}
