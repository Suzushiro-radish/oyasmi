use std::iter::Peekable;

use crate::{ast::{Expression, Node, Operator}, tokenizer::Token};

pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Node {
    parse_expression(tokens)
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Node {
    let mut node = parse_mul_div(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Add => {
                tokens.next();
                let rhs = parse_mul_div(tokens);
                node = Node::Expression(Expression::new(Operator::Add, node, rhs));
            }
            Token::Sub => {
                tokens.next();
                let rhs = parse_mul_div(tokens);
                node = Node::Expression(Expression::new(Operator::Sub, node, rhs));
            }
            _ => {
                break;
            }
        }
    }

    node
}


fn parse_mul_div(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Node {
    let mut node = parse_primary(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Mul => {
                tokens.next();
                let rhs = parse_primary(tokens);
                node = Node::Expression(Expression::new(Operator::Mul, node, rhs));
            }
            Token::Div => {
                tokens.next();
                let rhs = parse_primary(tokens);
                node = Node::Expression(Expression::new(Operator::Div, node, rhs));
            }
            _ => {
                break;
            }
        }
    }

    node
}

fn parse_primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Node {
    if let Some(token) = tokens.next() {
        match token {
            Token::Int(n) => Node::Number(n),
            _ => panic!("Unexpected token: {:?}", token),
        }
    } else {
        panic!("Unexpected end of input");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Token;

    #[test]
    fn test_parse() {
        let mut tokens = vec![Token::Int(1), Token::Add, Token::Int(2)].into_iter().peekable();
        let node = parse(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Add, Node::Number(1), Node::Number(2)));
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_primary() {
        let mut tokens = vec![Token::Int(1)].into_iter().peekable();
        let node = parse_primary(&mut tokens);
        let expected = Node::Number(1);
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_mul_div_mul() {
        let mut tokens = vec![Token::Int(1), Token::Mul, Token::Int(2)].into_iter().peekable();
        let node = parse_mul_div(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Mul, Node::Number(1), Node::Number(2)));
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_mul_div_div() {
        let mut tokens = vec![Token::Int(1), Token::Div, Token::Int(2), Token::Mul, Token::Int(4)].into_iter().peekable();
        let node = parse_mul_div(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Mul, Node::Expression(Expression::new(Operator::Div, Node::Number(1), Node::Number(2))), Node::Number(4)));
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_mul_div_primary() {
        let mut tokens = vec![Token::Int(42)].into_iter().peekable();
        let node = parse_mul_div(&mut tokens);
        let expected = Node::Number(42);
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_primary() {
        let mut tokens = vec![Token::Int(42)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Number(42);
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_add() {
        let mut tokens = vec![Token::Int(1), Token::Add, Token::Int(2)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Add, Node::Number(1), Node::Number(2)));
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_sub() {
        let mut tokens = vec![Token::Int(1), Token::Sub, Token::Int(2)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Sub, Node::Number(1), Node::Number(2)));
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_mul() {
        let mut tokens = vec![Token::Int(1), Token::Mul, Token::Int(2)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Expression(Expression::new(Operator::Mul, Node::Number(1), Node::Number(2)));
        assert_eq!(expected, node);
    }
}
