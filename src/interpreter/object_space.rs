use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{Value, Object, ObjectId},
    }
};

pub struct ObjectSpace {
    objects: HashMap<ObjectId, Object>,

    // TODO PERFORMANCE we're using ObjectId instead of &Object everywhere, don't think it's a good idea
    primitive_nil: ObjectId,
    primitive_true: ObjectId,
    primitive_false: ObjectId,
}

impl ObjectSpace {
    pub fn new() -> ObjectSpace {
        // primitives
        let nil_obj_id = ObjectId::new_v4(); let nil_obj = Object { id: nil_obj_id, value: Value::Nil };
        let true_obj_id = ObjectId::new_v4(); let true_obj = Object { id: true_obj_id, value: Value::True };
        let false_obj_id = ObjectId::new_v4(); let false_obj = Object { id: false_obj_id, value: Value::False };

        let mut space = ObjectSpace {
            objects: hashmap!{
                nil_obj_id => nil_obj,
                true_obj_id => true_obj,
                false_obj_id => false_obj,
            },
            primitive_nil: nil_obj_id,
            primitive_true: true_obj_id,
            primitive_false: false_obj_id,
        };

        space.predefine_classes();

        space
    }

    // 
    // generata an object with arbitrary value
    // 
    pub fn add(&mut self, value: Value) -> ObjectId {
        let id = ObjectId::new_v4(); // TODO wrap ObjectId
        let object = Object {
            value, id
        };
        self.objects.insert(id, object);
        id
    }

    // 
    // get object
    // 
    pub fn get_obj(&self, object_id: &ObjectId) -> &Object {
        self.objects.get(object_id).unwrap()
    }

    pub fn get_primitive_nil(&self) -> &Object { self.get_obj(&self.primitive_nil) }
    pub fn get_primitive_true(&self) -> &Object { self.get_obj(&self.primitive_true) }
    pub fn get_primitive_false(&self) -> &Object { self.get_obj(&self.primitive_false) }
}
