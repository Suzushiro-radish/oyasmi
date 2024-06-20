
#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Int(i32),
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
            _ => {}
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
    fn test_skip_whitespace() {
        let input = " 123 + 456 ".to_string();
        let expected = vec![Token::Int(123), Token::Add, Token::Int(456)];
        let actual = tokenize(&input);

        assert_eq!(expected, actual);
    }
}
