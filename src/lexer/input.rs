// TODO NOTE
// So apparently ragel can't handle UTF8 directly...

use std::ops::{Index};

#[derive(Clone)]
pub struct Input {
    input: String
}

impl Input {
    pub fn new(input: String) -> Input { Input { input } }
}

impl Index<usize> for Input {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.input.as_bytes()[i]
    }
}
