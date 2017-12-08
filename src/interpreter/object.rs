#[derive(Debug)]
pub enum Primitive {
    Number(i64)
}

#[derive(Debug)]
// TODO
pub struct Object {
    pub dummy_value: i64
}

impl Object {
    pub fn new() -> Object {
        // TODO
        Object { dummy_value: 0 }
    }
}

// TODO `impl Object::from_primitive`?

// pub struct Method<'a> {
//     pub obj: &'a Object,
pub struct Method {
    pub name: String,
    // pub body: TODO vec<ast:stmtwithinmethod>
}

impl Method {
    pub fn new(name: String) -> Method {
        Method {
            name
        }
    }
}
