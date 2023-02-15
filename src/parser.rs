use crate::ast::{Node, Operator};
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

pub fn parse(token_stream: Vec<Token>) -> Result<Node, ParserError> {
    let mut token_stream = token_stream.clone();

    // reverse the token stream so I can use pop to drain
    // tokens from the stream (like a stack)
    token_stream.reverse();

    let mut operators_stack: Vec<Operator> = vec![Operator::Sentinel];
    let mut operands_stack: Vec<Node> = vec![];

    parse_expression(&mut token_stream, &mut operators_stack, &mut operands_stack)?;

    if !token_stream.is_empty() {
        return Err(ParserError::TokenStreamNotEmpty);
    }

    println!("operands stack len: {}", operands_stack.len());

    Ok(operands_stack.pop().unwrap())
}

fn parse_expression(
    token_stream: &mut Vec<Token>,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<Node>,
) -> Result<(), ParserError> {
    parse_stmt(token_stream, operators_stack, operands_stack)?;

    loop {
        if let Some(next_tok) = token_stream.last() {
            match next_tok {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    let operator = match Operator::try_from(*next_tok) {
                        Ok(operator) => operator,
                        _ => return Err(ParserError::UnexpectedToken(*next_tok)),
                    };

                    push_operator(operator, operators_stack, operands_stack);
                    token_stream.pop();

                    parse_stmt(token_stream, operators_stack, operands_stack)?;
                }
                _ => todo!("maybe an error?"),
            }
        } else {
            break;
        }
    }

    loop {
        let top_stack_operator = operators_stack.last().unwrap();
        if *top_stack_operator == Operator::Sentinel {
            break;
        }

        pop_operator(operators_stack, operands_stack);
    }

    Ok(())
}

fn parse_stmt(
    token_stream: &mut Vec<Token>,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<Node>,
) -> Result<(), ParserError> {
    match token_stream.last() {
        Some(current_tok) => match current_tok {
            Token::I32(terminal_value) => {
                operands_stack.push(Node::I32(*terminal_value));
                token_stream.pop().unwrap();
                Ok(())
            }
            Token::OpenParen => {
                token_stream.pop();
                operators_stack.push(Operator::Sentinel);
                parse_expression(token_stream, operators_stack, operands_stack)?;

                // expect we end with a closing parenthesis
                match token_stream.last() {
                    Some(end_tok) => {
                        if *end_tok != Token::CloseParen {
                            return Err(ParserError::ExpectedClosingParent);
                        }

                        token_stream.pop();
                        operators_stack.pop();
                        Ok(())
                    }
                    None => Err(ParserError::UnexpectedEnd),
                }
            }
            Token::Minus => {
                todo!("should implement unary with the negative Token instead of Minus Token");
                token_stream.pop();
                parse_stmt(token_stream, operators_stack, operands_stack)?;
                Ok(())
            }
            _ => Err(ParserError::UnexpectedToken(*current_tok)),
        },
        None => Err(ParserError::UnexpectedEnd),
    }
}

fn push_operator(
    op: Operator,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<Node>,
) {
    loop {
        // retrieve the top operator in the stack and check
        // if it is greater than the given argument operator
        if let Some(top_stack_operator) = operators_stack.last() {
            if *top_stack_operator > op {
                pop_operator(operators_stack, operands_stack)
            } else {
                println!(
                    "top operator: {:?} is not greater than: {:?}",
                    *top_stack_operator, op
                );
                break;
            }
        } else {
            todo!("implement error here")
        }
    }

    operators_stack.push(op);
}

fn pop_operator(operators_stack: &mut Vec<Operator>, operands_stack: &mut Vec<Node>) {
    match operators_stack.last() {
        Some(value) => match *value {
            Operator::Plus | Operator::Minus | Operator::Multiplication | Operator::Division => {
                let rhs = operands_stack.pop().unwrap();
                let lhs = operands_stack.pop().unwrap();

                operands_stack.push(Node::BinaryExpr {
                    op: operators_stack.pop().unwrap(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            _ => todo!("should implement unary operations"),
        },
        None => {}
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse() {}
}
