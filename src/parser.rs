use crate::lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEnd,
    ExpectedClosingParent,
    UnexpectedToken(Token),
    TokenStreamNotEmpty,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEnd => write!(f, "unexpected end"),
            ParserError::ExpectedClosingParent => write!(f, "expected closing parent"),
            ParserError::UnexpectedToken(t) => write!(f, "unexpected token: {:?}", t),
            ParserError::TokenStreamNotEmpty => write!(f, "token stream not empty"),
        }
    }
}

/**
 * The grammar
 *
 * E --> P {B P}
 * P --> v | "(" E ")" | U P
 * B --> "+" | "-" | "*" | "/"
 * U --> "-"
 *
 * where `v` is a terminal
 */

pub fn parse(token_stream: Vec<Token>) -> Result<(), ParserError> {
    let mut token_stream = token_stream.clone();

    token_stream.reverse();
    parse_expression(&mut token_stream)?;

    if !token_stream.is_empty() {
        return Err(ParserError::TokenStreamNotEmpty);
    }

    Ok(())
}

fn parse_expression(token_stream: &mut Vec<Token>) -> Result<(), ParserError> {
    parse_stmt(token_stream)?;

    loop {
        if let Some(next_tok) = token_stream.last() {
            return match next_tok {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    token_stream.pop();
                    parse_stmt(token_stream)
                }
                _ => break,
            };
        }
    }

    Ok(())
}

fn parse_stmt(token_stream: &mut Vec<Token>) -> Result<(), ParserError> {
    match token_stream.last() {
        Some(current_tok) => match current_tok {
            Token::I32(_) => {
                token_stream.pop();
                Ok(())
            }
            Token::OpenParen => {
                token_stream.pop();
                parse_expression(token_stream)?;
                match token_stream.pop() {
                    Some(end_tok) => {
                        if end_tok != Token::CloseParen {
                            return Err(ParserError::ExpectedClosingParent);
                        }
                        Ok(())
                    }
                    None => Err(ParserError::UnexpectedEnd),
                }
            }
            Token::Minus => {
                token_stream.pop();
                parse_stmt(token_stream)?;
                Ok(())
            }
            _ => Err(ParserError::UnexpectedToken(*current_tok)),
        },
        None => Err(ParserError::UnexpectedEnd),
    }
}
