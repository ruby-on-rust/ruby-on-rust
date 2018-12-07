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
        let nil = Object::new(Value::Nil);       let primitive_nil = nil.id;
        let r#true = Object::new(Value::True);   let primitive_true = r#true.id;
        let r#false = Object::new(Value::False); let primitive_false = r#false.id;

        let objects = hashmap!{
            nil.id => nil,
            r#true.id => r#true,
            r#false.id => r#false,
        };

        let mut space = ObjectSpace {
            objects,
            primitive_nil, primitive_true, primitive_false,
        };

        // TODO predefine_primitive

        space.predefine_classes();

        space
    }

    // 
    // add arbitrary object
    // 
    pub fn add(&mut self, object: Object) {
        self.objects.insert(object.id, object);
    }

    // 
    // get object
    // 
    pub fn get_obj_by_id(&self, object_id: &ObjectId) -> &Object {
        self.objects.get(object_id).unwrap()
    }

    pub fn get_primitive_nil(&self) -> &Object { self.get_obj_by_id(&self.primitive_nil) }
    pub fn get_primitive_true(&self) -> &Object { self.get_obj_by_id(&self.primitive_true) }
    pub fn get_primitive_false(&self) -> &Object { self.get_obj_by_id(&self.primitive_false) }
}
