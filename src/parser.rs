use std::iter::Peekable;

use crate::{ast::{Node, Statement}, tokenizer::Token};

pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Vec<Statement> {
    let mut statements = Vec::new();

    while tokens.peek().is_some() {
        statements.push(parse_statement(tokens));
    }

    statements
}

fn parse_statement(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Statement {
    let node = parse_expression(tokens);

    if let Some(token) = tokens.peek() {
        match token {
            Token::Semicolon => {
                tokens.next();
                Statement::new(node)
            }
            _ => {
                panic!("Expected semicolon");
            }
        }
    } else {
        Statement::new(node)
    }
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Node {
    let mut node = parse_mul_div(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Add => {
                tokens.next();
                let rhs = parse_mul_div(tokens);
                node = Node::Add(node.into(), rhs.into());
            }
            Token::Sub => {
                tokens.next();
                let rhs = parse_mul_div(tokens);
                node = Node::Sub(node.into(), rhs.into());
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
                node = Node::Mul(node.into(), rhs.into());
            }
            Token::Div => {
                tokens.next();
                let rhs = parse_primary(tokens);
                node = Node::Div(node.into(), rhs.into());
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
            Token::Lparen => {
                let node = parse_expression(tokens);
                if let Some(Token::Rparen) = tokens.next() {
                    node
                } else {
                    panic!("Expected closing parenthesis");
                }
            }
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
        let expected = vec![Statement::new(Node::Add(Node::Number(1).into(), Node::Number(2).into()))];
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
        let expected = Node::Mul(Node::Number(1).into(), Node::Number(2).into());
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_mul_div_div() {
        let mut tokens = vec![Token::Int(1), Token::Div, Token::Int(2), Token::Mul, Token::Int(4)].into_iter().peekable();
        let node = parse_mul_div(&mut tokens);
        let expected = Node::Mul((Node::Div(Node::Number(1).into(), Node::Number(2).into())).into(), Node::Number(4).into());
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
        let expected = Node::Add(Node::Number(1).into(), Node::Number(2).into());
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_sub() {
        let mut tokens = vec![Token::Int(1), Token::Sub, Token::Int(2)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Sub(Node::Number(1).into(), Node::Number(2).into());
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_expression_mul() {
        let mut tokens = vec![Token::Int(1), Token::Mul, Token::Int(2)].into_iter().peekable();
        let node = parse_expression(&mut tokens);
        let expected = Node::Mul(Node::Number(1).into(), Node::Number(2).into());
        assert_eq!(expected, node);
    }
}
