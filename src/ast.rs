use std::cmp::{Ordering, PartialOrd};

use crate::lexer::Token;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Sentinel,
    Plus,
    Minus,
    Multiplication,
    Division,

    Negative,
}

pub struct NotAnOperatorError(Token);
impl TryFrom<Token> for Operator {
    type Error = NotAnOperatorError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Operator::Plus),
            Token::Minus => Ok(Operator::Minus),
            Token::Star => Ok(Operator::Multiplication),
            Token::Slash => Ok(Operator::Division),
            _ => Err(NotAnOperatorError(value)),
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = match self {
            Operator::Plus | Operator::Minus => match other {
                Operator::Sentinel => Ordering::Greater,
                Operator::Multiplication | Operator::Division | Operator::Negative => {
                    Ordering::Less
                }
                _ => Ordering::Equal,
            },
            Operator::Multiplication | Operator::Division => match other {
                Operator::Sentinel | Operator::Minus | Operator::Plus => Ordering::Greater,
                Operator::Negative => Ordering::Less,
                _ => Ordering::Equal,
            },
            Operator::Negative => Ordering::Greater,
            Operator::Sentinel => Ordering::Less,
        };

        Some(ordering)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    I32(i32),
    UnaryExpr {
        op: Operator,
        inner: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
