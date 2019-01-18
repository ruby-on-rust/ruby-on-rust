use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Oid(Uuid);

impl Oid {
    pub fn new() -> Oid { Oid(Uuid::new_v4()) }
}

// TODO https://github.com/uuid-rs/uuid/issues/356
impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.0).as_bytes()[0])
    }
}
