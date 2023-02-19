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

    Caret,
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
            '^' => tokens.push(Token::Caret),
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
        ];

        for idx in 0..tests.len() {
            let input = tests[idx];
            let expected = expectations[idx].clone();

            let output = extract_token_stream(input.to_string()).unwrap();
            assert_eq!(output, expected);
        }
    }
}
