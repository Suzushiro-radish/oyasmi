#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Lparen,
    Rparen,
    Int(i32),
    Semicolon,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '+' => {
                tokens.push(Token::Add);
            }
            '-' => {
                tokens.push(Token::Sub);
            }
            '*' => {
                tokens.push(Token::Mul);
            }
            '/' => {
                tokens.push(Token::Div);
            }
            '(' => {
                tokens.push(Token::Lparen);
            }
            ')' => {
                tokens.push(Token::Rparen);
            }
            ';' => {
                tokens.push(Token::Semicolon);
            }
            '0'..='9' => {
                let mut current_num = String::new();
                current_num.push(c);
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            current_num.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Int(current_num.parse().unwrap()));
            },

            _ => {
                panic!("Unknown character: {}", c);
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let input = "123".to_string();
        let expected = vec![Token::Int(123)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_add() {
        let input = "123+456".to_string();
        let expected = vec![Token::Int(123), Token::Add, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_sub() {
        let input = "123-456".to_string();
        let expected = vec![Token::Int(123), Token::Sub, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_mul() {
        let input = "123*456".to_string();
        let expected = vec![Token::Int(123), Token::Mul, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_div() {
        let input = "123/456".to_string();
        let expected = vec![Token::Int(123), Token::Div, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_skip_whitespace() {
        let input = " 123 + 456 ".to_string();
        let expected = vec![Token::Int(123), Token::Add, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }
}
