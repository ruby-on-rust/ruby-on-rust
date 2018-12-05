use std::collections::HashMap;
use crate::{
    ast::node::{Node, SomeNode},
    interpreter::object::{Value, Object, ObjectId},
};

struct ObjectSpace {
    objects: HashMap<ObjectId, Object>,

    // TODO PERFORMANCE well this is soooo bad
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

        ObjectSpace {
            objects,
            primitive_nil, primitive_true, primitive_false,
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.insert(object.id, object);
    }

    pub fn get_obj_by_id(&self, object_id: &ObjectId) -> &Object {
        self.objects.get(object_id).unwrap()
    }

    pub fn get_primitive_nil(&self) -> &Object { self.get_obj_by_id(&self.primitive_nil) }
    pub fn get_primitive_true(&self) -> &Object { self.get_obj_by_id(&self.primitive_true) }
    pub fn get_primitive_false(&self) -> &Object { self.get_obj_by_id(&self.primitive_false) }
}

pub struct Interpreter {
    objects: ObjectSpace
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            objects: ObjectSpace::new(),
        };

        // init core objects

        interpreter
    }

    pub fn eval(&mut self, node: SomeNode) -> &Object {
        if node.is_none() { return self.objects.get_primitive_nil(); }

        let node = node.unwrap();

        match node {
            Node::Nil => { self.objects.get_primitive_nil() },
            Node::True => { self.objects.get_primitive_true() },
            Node::False => { self.objects.get_primitive_false() },

            Node::If {condition, then_body, else_body} => {
                if self.eval_and_test_bool(Some(*condition)) { 
                    self.eval(*then_body)
                } else {
                    self.eval(*else_body)
                }
            },

            _ => { panic!("eval: don't know how to handle node: {:?}", node); }
        }
    }

    fn eval_and_test_bool(&mut self, node: Option<Node>) -> bool {
        match self.eval(node).value {
            Value::Nil | Value::False => false,
            _ => true
        }
    }
}
