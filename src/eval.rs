use crate::ast::{ASTNode, Operator};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericObject {
    I32(i32),
    F32(f32),
}

impl fmt::Display for NumericObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericObject::F32(value) => write!(f, "{}", value),
            NumericObject::I32(value) => write!(f, "{}", value),
        }
    }
}

macro_rules! check_div_or_err {
    ($lhs:ident, $rhs:ident) => {
        match $lhs.checked_div($rhs) {
            Some(value) => value,
            None => {
                return Err(EvaluateError::AttemptToDivideByZero);
            }
        }
    };
}

macro_rules! normalize_numeric_operation {
    ($lhs:tt / $rhs:ident) => {
        match ($lhs, $rhs) {
            (NumericObject::I32(lhs), NumericObject::I32(rhs)) => Ok(NumericObject::I32(check_div_or_err!(lhs, rhs))),
            (NumericObject::F32(lhs), NumericObject::F32(rhs)) => Ok(NumericObject::F32(lhs / rhs)),
            (NumericObject::F32(lhs), NumericObject::I32(rhs)) => {
                return Ok(NumericObject::F32(lhs / rhs as f32));
            }
            (NumericObject::I32(lhs), NumericObject::F32(rhs)) => {
                return Ok(NumericObject::F32(lhs as f32 / rhs));
            }
        }
    };

    ($lhs:ident $op:tt $rhs:ident) => {
        match ($lhs, $rhs) {
            (NumericObject::I32(lhs), NumericObject::I32(rhs)) => Ok(NumericObject::I32(lhs $op rhs)),
            (NumericObject::F32(lhs), NumericObject::F32(rhs)) => Ok(NumericObject::F32(lhs $op rhs)),
            (NumericObject::F32(lhs), NumericObject::I32(rhs)) => {
                return Ok(NumericObject::F32(lhs $op rhs as f32));
            }
            (NumericObject::I32(lhs), NumericObject::F32(rhs)) => {
                return Ok(NumericObject::F32(lhs as f32 $op rhs));
            }
        }
    };
}

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedUnaryOperator,
    UnexpectedBinaryOperator,
    AttemptToDivideByZero,
}

pub fn evaluate(expression_tree: ASTNode) {
    let result = evaluate_expression(expression_tree);
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Evaluation error: {:?}", err),
    }
}

fn evaluate_expression(expression_tree: ASTNode) -> Result<NumericObject, EvaluateError> {
    match expression_tree {
        ASTNode::I32(value) => Ok(NumericObject::I32(value)),
        ASTNode::F32(value) => Ok(NumericObject::F32(value)),
        ASTNode::UnaryExpr { op, inner } => match op {
            Operator::Negative => {
                let evaluated_object = evaluate_expression(*inner).unwrap();
                let negative = match evaluated_object {
                    NumericObject::F32(value) => NumericObject::F32(-value),
                    NumericObject::I32(value) => NumericObject::I32(-value),
                };
                Ok(negative)
            }
            _ => Err(EvaluateError::UnexpectedUnaryOperator),
        },
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            let lhs = evaluate_expression(*lhs).unwrap();
            let rhs = evaluate_expression(*rhs).unwrap();

            match op {
                Operator::Plus => normalize_numeric_operation!(lhs + rhs),
                Operator::Minus => normalize_numeric_operation!(lhs - rhs),
                Operator::Multiplication => normalize_numeric_operation!(lhs * rhs),
                Operator::Division => normalize_numeric_operation!(lhs / rhs),
                Operator::Exponential => match (lhs, rhs) {
                    (NumericObject::I32(lhs), NumericObject::I32(rhs)) => {
                        Ok(NumericObject::I32(lhs.pow(rhs as u32)))
                    }
                    (NumericObject::F32(lhs), NumericObject::F32(rhs)) => {
                        Ok(NumericObject::F32(lhs.powf(rhs)))
                    }
                    (NumericObject::I32(lhs), NumericObject::F32(rhs)) => {
                        Ok(NumericObject::F32((lhs as f32).powf(rhs)))
                    }
                    (NumericObject::F32(lhs), NumericObject::I32(rhs)) => {
                        Ok(NumericObject::F32(lhs.powi(rhs)))
                    }
                },
                _ => Err(EvaluateError::UnexpectedBinaryOperator),
            }
        }
    }
}
