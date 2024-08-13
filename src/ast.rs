#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub node: Node,
}

impl Statement {
    pub fn new(node: Node) -> Self {
        Statement { node }
    }
}
