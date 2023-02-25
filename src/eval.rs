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

macro_rules! normalize_numeric_operation {
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
}

pub fn evaluate(expression_tree: ASTNode) {
    let result = evaluate_expression(expression_tree).unwrap();
    println!("{}", result);
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
        ASTNode::BinaryExpr { op, lhs, rhs } => match op {
            Operator::Plus => {
                let eval_lhs = evaluate_expression(*lhs).unwrap();
                let eval_rhs = evaluate_expression(*rhs).unwrap();

                normalize_numeric_operation!(eval_lhs + eval_rhs)
            }
            Operator::Minus => {
                let eval_lhs = evaluate_expression(*lhs).unwrap();
                let eval_rhs = evaluate_expression(*rhs).unwrap();

                normalize_numeric_operation!(eval_lhs - eval_rhs)
            }
            Operator::Multiplication => {
                let eval_lhs = evaluate_expression(*lhs).unwrap();
                let eval_rhs = evaluate_expression(*rhs).unwrap();

                normalize_numeric_operation!(eval_lhs * eval_rhs)
            }
            Operator::Division => {
                let eval_lhs = evaluate_expression(*lhs).unwrap();
                let eval_rhs = evaluate_expression(*rhs).unwrap();

                normalize_numeric_operation!(eval_lhs / eval_rhs)
            }
            Operator::Exponential => {
                let eval_lhs = evaluate_expression(*lhs).unwrap();
                let eval_rhs = evaluate_expression(*rhs).unwrap();

                match (eval_lhs, eval_rhs) {
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
                }
            }
            _ => Err(EvaluateError::UnexpectedBinaryOperator),
        },
    }
}
