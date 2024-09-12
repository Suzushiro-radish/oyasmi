#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Node(Node),
    Assign(String, Node),
}
