use crate::ast::{Node, Operator};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    I32(i32),

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidInputChar(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidInputChar(c) => write!(f, "invalid input char: {}", c),
        }
    }
}

pub fn extract_token_stream(line: String) -> Result<Vec<Token>, LexerError> {
    let mut source_as_chars = line.chars().filter(|c| !c.is_whitespace()).peekable();

    let mut tokens: Vec<Token> = vec![];

    while let Some(current) = source_as_chars.next() {
        match current {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            _ => {
                if current.is_numeric() {
                    let mut numbers_in_seq = vec![current.to_string()];
                    while let Some(current) = source_as_chars.peek() {
                        if current.is_numeric() {
                            numbers_in_seq.push(current.to_string());
                            source_as_chars.next();
                        } else {
                            break;
                        }
                    }

                    let numbers_in_seq = numbers_in_seq.join("");
                    tokens.push(Token::I32(numbers_in_seq.parse::<i32>().unwrap()));
                } else {
                    return Err(LexerError::InvalidInputChar(current.to_string()));
                }
            }
        };
    }

    Ok(tokens)
}

mod tests {
    use super::*;

    #[test]
    fn test_extract_token_stream() {
        let input = "1 + 1";
        let output = extract_token_stream(input.to_string()).unwrap();

        let expected = vec![Token::I32(1), Token::Plus, Token::I32(1)];
        assert_eq!(output, expected);
    }
}
