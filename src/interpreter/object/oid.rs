use uuid::Uuid;
pub type Oid = Uuid;
pub fn new_oid() -> Oid { Uuid::new_v4() }
