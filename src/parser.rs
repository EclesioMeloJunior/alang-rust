use crate::ast::{ASTNode, Operator};
use crate::lexer::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    ExpectedClosingParent,
    UnexpectedToken(Token),
    TokenStreamNotEmpty,
    FailedToParseAllOperators,
    ExpectedIdentifier,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ExpectedClosingParent => write!(f, "expected closing parent"),
            ParserError::UnexpectedToken(t) => write!(f, "unexpected token: {:?}", t),
            ParserError::TokenStreamNotEmpty => write!(f, "token stream not empty"),
            ParserError::FailedToParseAllOperators => write!(f, "failed to parse all operators"),
            ParserError::ExpectedIdentifier => write!(f, "expected variable identifier"),
        }
    }
}

/**
 * The grammar
 *
 * E --> P {B P}
 * P --> v | "=" P | "(" E ")" | U P
 * B --> "+" | "-" | "*" | "/"
 * U --> "-"
 *
 * where `v` is a terminal
 */

pub fn parse(token_stream: Vec<Token>) -> Result<ASTNode, ParserError> {
    let mut token_stream = token_stream.clone();

    // reverse the token stream so I can use pop to drain
    // tokens from the stream (like a stack)
    token_stream.reverse();

    let mut operators_stack: Vec<Operator> = vec![Operator::Sentinel];
    let mut operands_stack: Vec<ASTNode> = vec![];

    parse_expression(&mut token_stream, &mut operators_stack, &mut operands_stack)?;

    if !token_stream.is_empty() {
        return Err(ParserError::TokenStreamNotEmpty);
    }

    if operands_stack.len() != 1 {
        return Err(ParserError::FailedToParseAllOperators);
    }

    Ok(operands_stack.pop().unwrap())
}

fn parse_expression(
    token_stream: &mut Vec<Token>,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<ASTNode>,
) -> Result<(), ParserError> {
    parse_stmt(token_stream, operators_stack, operands_stack)?;

    loop {
        if let Some(next_tok) = token_stream.last() {
            match next_tok {
                // when we reach a closing parent we just return since
                // it is possible we are inside an open paren iteration
                Token::CloseParen => break,
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::Caret
                | Token::Assign => {
                    let operator = match Operator::try_from(next_tok) {
                        Ok(operator) => operator,
                        _ => return Err(ParserError::UnexpectedToken(next_tok.clone())),
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

    rewind_operands(operators_stack, operands_stack);
    Ok(())
}

fn rewind_operands(operators_stack: &mut Vec<Operator>, operands_stack: &mut Vec<ASTNode>) {
    loop {
        let top_stack_operator = operators_stack.last().unwrap();
        if *top_stack_operator == Operator::Sentinel {
            break;
        }

        pop_operator(operators_stack, operands_stack);
    }
}

fn parse_stmt(
    token_stream: &mut Vec<Token>,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<ASTNode>,
) -> Result<(), ParserError> {
    match token_stream.last() {
        Some(current_tok) => match current_tok {
            Token::Let => {
                // remove Token::Let from the token stream
                token_stream.pop().unwrap();

                // the next token should be an identifier otherwise raise an error
                match token_stream.pop().unwrap() {
                    Token::Ident(identifier) => {
                        operands_stack.push(ASTNode::Ident(identifier.clone()));
                    }
                    _ => return Err(ParserError::ExpectedIdentifier),
                }
                Ok(())
            }

            Token::Ident(identifier) => {
                operands_stack.push(ASTNode::Ident(identifier.clone()));
                token_stream.pop().unwrap();

                Ok(())
            }
            Token::F32(value) => {
                operands_stack.push(ASTNode::F32(*value));
                token_stream.pop().unwrap();
                Ok(())
            }
            Token::I32(value) => {
                operands_stack.push(ASTNode::I32(*value));
                token_stream.pop().unwrap();
                Ok(())
            }
            Token::OpenParen => {
                token_stream.pop();
                operators_stack.push(Operator::Sentinel);
                parse_expression(token_stream, operators_stack, operands_stack)?;

                // expect we end with a closing parenthesis
                if let Some(end_token) = token_stream.last() {
                    if *end_token != Token::CloseParen {
                        return Err(ParserError::ExpectedClosingParent);
                    }

                    token_stream.pop();
                    operators_stack.pop();
                    return Ok(());
                } else {
                    return Err(ParserError::ExpectedClosingParent);
                }
            }
            Token::Minus => {
                token_stream.pop();
                operators_stack.push(Operator::Negative);
                parse_stmt(token_stream, operators_stack, operands_stack)?;
                Ok(())
            }
            _ => Err(ParserError::UnexpectedToken(current_tok.clone())),
        },
        None => Ok(()),
    }
}

fn push_operator(
    op: Operator,
    operators_stack: &mut Vec<Operator>,
    operands_stack: &mut Vec<ASTNode>,
) {
    loop {
        // retrieve the top operator in the stack and check
        // if it is greater than the given argument operator
        if let Some(top_stack_operator) = operators_stack.last() {
            if *top_stack_operator > op {
                pop_operator(operators_stack, operands_stack)
            } else {
                break;
            }
        } else {
            todo!("implement error here")
        }
    }

    operators_stack.push(op);
}

fn pop_operator(operators_stack: &mut Vec<Operator>, operands_stack: &mut Vec<ASTNode>) {
    match operators_stack.last() {
        Some(value) => match *value {
            Operator::Plus
            | Operator::Minus
            | Operator::Multiplication
            | Operator::Division
            | Operator::Exponential
            | Operator::Assign => {
                let rhs = operands_stack.pop().unwrap();
                let lhs = operands_stack.pop().unwrap();

                operands_stack.push(ASTNode::BinaryExpr {
                    op: operators_stack.pop().unwrap(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Negative => {
                let unary_expression = ASTNode::UnaryExpr {
                    op: operators_stack.pop().unwrap(),
                    inner: Box::new(operands_stack.pop().unwrap()),
                };

                operands_stack.push(unary_expression);
            }
            _ => todo!("return an error if the popped operator is not supported"),
        },
        None => {}
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{
        ast::{ASTNode, Operator},
        parser::parse,
    };

    #[test]
    fn test_parser() {
        use crate::lexer::token::Token;
        let tokens_tests: Vec<Vec<Token>> = vec![
            vec![Token::I32(1), Token::Plus, Token::I32(1)],
            vec![
                Token::I32(1),
                Token::Plus,
                Token::I32(1),
                Token::Star,
                Token::I32(2),
            ],
            // should parse the unary expression
            vec![
                Token::Minus,
                Token::I32(1),
                Token::Plus,
                Token::I32(1),
                Token::Slash,
                Token::I32(2),
            ],
            vec![Token::Minus, Token::I32(1), Token::Minus, Token::I32(1)],
            vec![
                Token::I32(10),
                Token::Slash,
                Token::OpenParen,
                Token::I32(90),
                Token::Plus,
                Token::I32(8),
                Token::CloseParen,
            ],
            vec![Token::I32(2), Token::Caret, Token::I32(2)],
        ];

        let expected_outputs: Vec<ASTNode> = vec![
            ASTNode::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(ASTNode::I32(1)),
                rhs: Box::new(ASTNode::I32(1)),
            },
            ASTNode::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(ASTNode::I32(1)),
                rhs: Box::new(ASTNode::BinaryExpr {
                    op: Operator::Multiplication,
                    lhs: Box::new(ASTNode::I32(1)),
                    rhs: Box::new(ASTNode::I32(2)),
                }),
            },
            ASTNode::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(ASTNode::UnaryExpr {
                    op: Operator::Negative,
                    inner: Box::new(ASTNode::I32(1)),
                }),
                rhs: Box::new(ASTNode::BinaryExpr {
                    op: Operator::Division,
                    lhs: Box::new(ASTNode::I32(1)),
                    rhs: Box::new(ASTNode::I32(2)),
                }),
            },
            ASTNode::BinaryExpr {
                op: Operator::Minus,
                lhs: Box::new(ASTNode::UnaryExpr {
                    op: Operator::Negative,
                    inner: Box::new(ASTNode::I32(1)),
                }),
                rhs: Box::new(ASTNode::I32(1)),
            },
            ASTNode::BinaryExpr {
                op: Operator::Division,
                lhs: Box::new(ASTNode::I32(10)),
                rhs: Box::new(ASTNode::BinaryExpr {
                    op: Operator::Plus,
                    lhs: Box::new(ASTNode::I32(90)),
                    rhs: Box::new(ASTNode::I32(8)),
                }),
            },
            ASTNode::BinaryExpr {
                op: Operator::Exponential,
                lhs: Box::new(ASTNode::I32(2)),
                rhs: Box::new(ASTNode::I32(2)),
            },
        ];

        for idx in 0..tokens_tests.len() {
            let tokens_to_test = tokens_tests[idx].clone();
            let expected_ast = expected_outputs[idx].clone();

            let output_ast = parse(tokens_to_test).unwrap();
            assert_eq!(output_ast, expected_ast);
        }
    }
}
