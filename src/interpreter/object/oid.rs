use uuid::Uuid;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Oid(Uuid);

impl Oid {
    pub fn new() -> Oid { Oid(Uuid::new_v4()) }
}
