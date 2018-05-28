#[derive(Debug)]
pub enum Node {
    Dummy

    // Literal(i32),

    // Binary {
    //     op: &'static str,
    //     left: Box<Node>,
    //     right: Box<Node>,
    // },
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        match ( self, other ) {
            ( Node::Dummy, Node::Dummy ) => true,
            _ => false
        }
    }
}
