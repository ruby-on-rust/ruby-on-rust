use interpreter::object::{Object, Primitive};

pub type Refer = Option<u64>;

pub struct Memory {
    space: Vec<Object>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            space: Vec::new()
        }
    }

    // TODO CLEANUP
    // // allocate a new slot in memory, save the value, and return the assigned address
    // // TODO return a Result
    // pub fn allocate_value(&mut self, value: Value) -> Refer {
    //     self.space.push(value);

    //     // TODO CLEANUP, or at least use that logging crate
    //     println!("allocated memory, current:");
    //     println!("{:?}", self.space);

    //     return Some(( self.space.len() - 1 ) as u64)
    // }

    // // TODO handle nil
    // // TODO return a Result<Option<>>
    // pub fn get_value_from_refer(&self, refer: Refer) -> &Value {
    //     // TODO CLEANUP, or at least use that logging crate
    //     println!("getting value of refer:");
    //     println!("{:?}", refer);

    //     if let Some(addr) = refer {
    //         return &self.space[addr as usize]
    //     } else {
    //         unimplemented!()
    //     }
    // }

    // TODO
    // - return a Result
    pub fn allocate_primitive(&mut self, primitive: Primitive) -> Refer {
        // TODO DUMMY
        let Primitive::Number(dummy_value) = primitive;
        let new_obj = Object { dummy_value };

        self.space.push(new_obj);

        // TODO CLEANUP, or at least use that logging crate
        println!("allocated memory, current space:");
        println!("{:?}", self.space);

        return Some(( self.space.len() - 1 ) as u64)
    }

    // TODO
    // - handle nil
    // - return a Result<Option<>>
    pub fn get_obj_from_refer(&self, refer: Refer) -> &Object {
        // TODO CLEANUP, or at least use that logging crate
        println!("getting obj of refer: {:?}", refer);

        if let Some(addr) = refer {
            return &self.space[addr as usize]
        } else {
            // nil
            unimplemented!()
        }
    }
}
