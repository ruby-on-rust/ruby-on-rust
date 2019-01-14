// TODO https://doc.rust-lang.org/rust-by-example/generics/new_types.html
use uuid::Uuid;
pub type Oid = Uuid;
pub fn new_oid() -> Oid { Uuid::new_v4() }
