use std::vec::IntoIter;
use crate::error::ParseError;
use crate::object::Object;

pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

fn scan(input: &str) -> Vec<Token> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| match s {
            "(" => Token::LParen,
            ")" => Token::RParen,
            _ => {
                let i = s.parse::<i64>();
                if i.is_ok() {
                    Token::Integer(i.unwrap())
                } else {
                    Token::Symbol(s.to_string())
                }
            },
        })
        .collect()
}

pub fn parse(string: &str) -> Result<Object, ParseError> {
    let mut tokens = scan(string).into_iter();
    let mut first_token = tokens.next();
    if let Some(Token::LParen) = first_token {
        parse_list(&mut tokens)
    } else {
        Err(ParseError {
            message: "Expected (".to_string(),
        })
    }
}

fn parse_list(tokens: &mut IntoIter<Token>) -> Result<Object, ParseError> {
    let mut result = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Integer(i) => result.push(Object::Integer(i)),
            Token::Symbol(s) => result.push(Object::Symbol(s)),
            Token::LParen => result.push(parse_list(tokens)?),
            Token::RParen => break,
        }
    }
    Ok(Object::List(result))
}

#[cfg(test)]
mod tests {
    use crate::parser::{Object, parse};

    #[test]
    fn parses_simple_s_expression() {
        let input = "(+ 1 2)";
        let result = parse(input).unwrap();
        assert_eq!(result, Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::Integer(2),
        ]));
    }

    #[test]
    fn parses_nested_s_expressions() {
        let input = "(+ 1 (* 2 3))";
        let result = parse(input).unwrap();
        assert_eq!(result, Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::List(vec![
                Object::Symbol("*".to_string()),
                Object::Integer(2),
                Object::Integer(3),
            ]),
        ]));
    }

    #[test]
    fn parses_triply_nested_s_expressions() {
        let input = "(+ 1 (* 2 (+ 3 4)))";
        let result = parse(input).unwrap();
        assert_eq!(result, Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::List(vec![
                Object::Symbol("*".to_string()),
                Object::Integer(2),
                Object::List(vec![
                    Object::Symbol("+".to_string()),
                    Object::Integer(3),
                    Object::Integer(4),
                ]),
            ]),
        ]));
    }
}
