use uuid::Uuid;

pub type ObjectId = Uuid;

// pub static dummy_obj_id: ObjectId = Uuid::parse_str("0").unwrap();

pub fn new_obj_id() -> ObjectId { Uuid::new_v4() }
