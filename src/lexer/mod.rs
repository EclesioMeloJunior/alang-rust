pub mod token;

use core::num;
use std::{fmt, iter::Peekable, str::Chars};
use token::Token;

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

fn read_numeric(current: char, source: &mut Peekable<Chars>) -> String {
    let mut numbers_in_seq = vec![current.to_string()];

    let while_numbers_push = |sequence: &mut Vec<String>, source: &mut Peekable<Chars>| {
        while let Some(current) = source.peek() {
            if !current.is_numeric() {
                break;
            }

            sequence.push(current.to_string());
            source.next();
        }
    };

    while_numbers_push(&mut numbers_in_seq, source);

    if let Some(current) = source.peek() {
        match current {
            '.' => {
                // adds the '.' to the sequence
                numbers_in_seq.push(source.next().unwrap().to_string());
                while_numbers_push(&mut numbers_in_seq, source);
            }
            _ => {}
        }
    }

    numbers_in_seq.join("")
}

fn read_keyword_or_identifier(current: char, source: &mut Peekable<Chars>) -> Option<String> {
    let mut sequence = vec![current];
    while let Some(current) = source.peek() {
        if !current.is_alphabetic() {
            break;
        }

        sequence.push(source.next().unwrap());
    }

    Some(sequence.iter().collect())
}

pub fn extract_token_stream(line: String) -> Result<Vec<Token>, LexerError> {
    let mut source_as_chars = line.chars().peekable();

    let mut tokens: Vec<Token> = vec![];

    while let Some(current) = source_as_chars.next() {
        match current {
            ' ' | '\n' | '\r' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '^' => tokens.push(Token::Caret),
            '=' => tokens.push(Token::Assign),
            _ => {
                if current.is_numeric() {
                    let numeric_sequence = read_numeric(current, &mut source_as_chars);
                    match numeric_sequence.contains(".") {
                        true => tokens.push(Token::F32(numeric_sequence.parse::<f32>().unwrap())),
                        false => tokens.push(Token::I32(numeric_sequence.parse::<i32>().unwrap())),
                    }
                    continue;
                }

                match read_keyword_or_identifier(current, &mut source_as_chars) {
                    Some(value) => match value.as_ref() {
                        "let" => tokens.push(Token::Let),
                        _ => tokens.push(Token::Ident(value.clone())),
                    },
                    None => return Err(LexerError::InvalidInputChar(current.to_string())),
                }
            }
        };
    }

    Ok(tokens)
}

mod tests {
    #[allow(unused_imports)]
    use super::{extract_token_stream, Token};

    #[test]
    fn test_extract_token_stream() {
        let tests: Vec<&'static str> = vec![
            "1 + 1",
            "-1 + 1",
            "5 * 1 + 1",
            "5 * 1 / 1",
            "(5 + 5)",
            "2 ^ 2",
            "10.0",
            "let a = 1",
        ];
        let expectations: Vec<Vec<Token>> = vec![
            vec![Token::I32(1), Token::Plus, Token::I32(1)],
            vec![Token::Minus, Token::I32(1), Token::Plus, Token::I32(1)],
            vec![
                Token::I32(5),
                Token::Star,
                Token::I32(1),
                Token::Plus,
                Token::I32(1),
            ],
            vec![
                Token::I32(5),
                Token::Star,
                Token::I32(1),
                Token::Slash,
                Token::I32(1),
            ],
            vec![
                Token::OpenParen,
                Token::I32(5),
                Token::Plus,
                Token::I32(5),
                Token::CloseParen,
            ],
            vec![Token::I32(2), Token::Caret, Token::I32(2)],
            vec![Token::F32(10.0)],
            vec![
                Token::Let,
                Token::Ident("a".into()),
                Token::Assign,
                Token::I32(1),
            ],
        ];

        for idx in 0..tests.len() {
            let input = tests[idx];
            let expected = expectations[idx].clone();

            let output = extract_token_stream(input.to_string()).unwrap();
            assert_eq!(output, expected);
        }
    }
}
