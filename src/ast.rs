#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: Operator,
}

impl Expression {
    pub fn new( op: Operator, lhs: Node, rhs: Node,) -> Self {
        Expression {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
