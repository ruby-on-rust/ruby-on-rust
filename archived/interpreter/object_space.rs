use std::cell::RefCell;
use std::collections::HashMap;
use crate::{
    interpreter::{
        object::{
            object_id::{ ObjectId, new_obj_id },
            Value, Object,
            name_tables::{ ConstTable, ConstsOwning, MethodTable },
            class_value::ClassValue
        },
    }
};

pub struct ObjectSpace {
    // TODO make this an Arena/Zone/Region struct
    objects: RefCell<HashMap<ObjectId, Object>>,

    primitive_nil: ObjectId,
    primitive_true: ObjectId,
    primitive_false: ObjectId,

    // primitive_basic_object: ObjectId,
    pub primitive_object: ObjectId,
    primitive_class: ObjectId,
    primitive_module: ObjectId,
}

impl ObjectSpace {
    pub fn new() -> ObjectSpace {
        // 
        // bootstrap with BasicObject Object Module Class
        // 

        let (basic_object_id, object_id, module_id, class_id) = (new_obj_id(), new_obj_id(), new_obj_id(), new_obj_id());

        let basic_object = Object {
            id: basic_object_id,
            class: class_id,
            value: Value::Class( ClassValue::new_basic_object() ),
        };

        let mut object = Object {
            id: object_id,
            class: class_id,
            value: Value::Class( ClassValue::new(basic_object_id) ),
        };

        let module = Object {
            id: module_id,
            class: class_id,
            value: Value::Class( ClassValue::new(basic_object_id) ),
        };

        let mut class = Object {
            id: class_id,
            class: class_id,
            value: Value::Class( ClassValue::new(basic_object_id) ),
        };

        // 
        // predefine singleton classes and values: NilClass, TrueClass, FalseClass
        // 

        let (nil_class_id, nil_id, true_class_id, true_id, false_class_id, false_id) = (new_obj_id(), new_obj_id(), new_obj_id(), new_obj_id(), new_obj_id(), new_obj_id());

        let nil_class = Object {
            id: nil_class_id,
            class: class_id,
            value: Value::Class( ClassValue::new_singleton(basic_object_id) ),
        };
        let nil = Object {
            id: nil_id,
            class: nil_class_id,
            value: Value::Nil
        };

        let true_class = Object {
            id: true_class_id,
            class: class_id,
            value: Value::Class( ClassValue::new_singleton(basic_object_id) ),
        };
        let r#true = Object {
            id: nil_id,
            class: true_class_id,
            value: Value::True
        };

        let false_class = Object {
            id: false_class_id,
            class: class_id,
            value: Value::Class( ClassValue::new_singleton(basic_object_id) ),
        };
        let r#false = Object {
            id: nil_id,
            class: false_class_id,
            value: Value::False
        };

        // add top-level consts
        if let Value::Class(ref mut object) = class.value {
            object.consts_add("BasicObject".to_string(), basic_object_id);
            object.consts_add("Object".to_string(), object_id);
            object.consts_add("Module".to_string(), module_id);
            object.consts_add("Class".to_string(), class_id);
            object.consts_add("NilClass".to_string(), nil_class_id);
            object.consts_add("TrueClass".to_string(), true_class_id);
            object.consts_add("FalseClass".to_string(), false_class_id);
        }

        let mut space = ObjectSpace {
            objects: RefCell::new(hashmap! {
                basic_object_id => basic_object,
                object_id => object,
                module_id => module,
                class_id => class,
                nil_class_id => nil_class,
                nil_id => nil,
                true_class_id => true_class,
                true_id => r#true,
                false_class_id => false_class,
                false_id => r#false,
            }),
            primitive_nil: nil_id,
            primitive_true: true_id,
            primitive_false: false_id,
            primitive_object: object_id,
            primitive_class: class_id,
            primitive_module: module_id,
        };

        space
    }

    // 
    // generata an object with arbitrary value
    // 
    pub fn add(&mut self, value: Value) -> ObjectId {
        let id = new_obj_id();

        // TODO
        assert_eq!(self.objects.borrow().contains_key(&id), false);

        // TODO find top-level `Class`
        let class = new_obj_id();

        let object = Object { id, value, class };
        self.objects.borrow_mut().insert(id, object);
        id
    }

    // 
    // get object
    // 
    pub fn get(&mut self, object_id: ObjectId) -> &mut Object {
        self.objects.borrow_mut().get_mut(&object_id).unwrap()
    }

    // TODO nil! true! false!, etc.
    pub fn get_primitive_nil(&mut self) -> &mut Object { self.get(self.primitive_nil) }
    pub fn get_primitive_true(&mut self) -> &mut Object { self.get(self.primitive_true) }
    pub fn get_primitive_false(&mut self) -> &mut Object { self.get(self.primitive_false) }
    pub fn get_primitive_object(&mut self) -> &mut Object { self.get(self.primitive_object) }
}
