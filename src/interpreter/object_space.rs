use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{
            new_obj_id, Value, Object, ObjectId,
            name_tables::ConstTable,
            class_value::ClassValue
        },
    }
};

pub struct ObjectSpace {
    objects: HashMap<ObjectId, Object>,

    // TODO PERFORMANCE we're using ObjectId instead of &Object everywhere, don't think it's a good idea
    // TODO NilClass TrueClass FalseClass
    primitive_nil: ObjectId,
    primitive_true: ObjectId,
    primitive_false: ObjectId,

    pub root_object_id: ObjectId,
}

impl ObjectSpace {
    pub fn new() -> ObjectSpace {
        // BasicObject
        let basic_object_id = new_obj_id();
        // TODO ClassValue::new
        let basic_object = Object {
            id: basic_object_id,
            value: Value::Class( ClassValue { superclass: None, consts: ConstTable::new() } ),
        };

        let object_id = new_obj_id();
        let object = Object {
            id: object_id,
            value: Value::Class( ClassValue { superclass: Some(basic_object_id), consts: ConstTable::new() } ),
        };

        // primitives
        // TODO
        let nil_obj_id = new_obj_id(); let nil_obj = Object { id: nil_obj_id, value: Value::Nil };
        let true_obj_id = new_obj_id(); let true_obj = Object { id: true_obj_id, value: Value::True };
        let false_obj_id = new_obj_id(); let false_obj = Object { id: false_obj_id, value: Value::False };

        let mut space = ObjectSpace {
            objects: hashmap! {
                basic_object_id => basic_object,
                object_id => object,
                nil_obj_id => nil_obj,
                true_obj_id => true_obj,
                false_obj_id => false_obj,
            },
            primitive_nil: nil_obj_id,
            primitive_true: true_obj_id,
            primitive_false: false_obj_id,
            root_object_id: object_id,
        };

        space.predefine_classes();

        space
    }

    // 
    // generata an object with arbitrary value
    // 
    pub fn add(&mut self, value: Value) -> ObjectId {
        let id = new_obj_id();
        let object = Object { id, value };
        self.objects.insert(id, object);
        id
    }

    // 
    // get object
    // 
    pub fn get(&mut self, object_id: ObjectId) -> &mut Object {
        self.objects.get_mut(&object_id).unwrap()
    }

    pub fn get_root_obj(&mut self) -> &mut Object {
        self.objects.get_mut(&self.root_object_id).unwrap()
    }

    pub fn get_primitive_nil(&mut self) -> &mut Object { self.get(self.primitive_nil) }
    pub fn get_primitive_true(&mut self) -> &mut Object { self.get(self.primitive_true) }
    pub fn get_primitive_false(&mut self) -> &mut Object { self.get(self.primitive_false) }
}
