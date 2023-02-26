use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

use crate::ast::{ASTNode, Operator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericObject {
    Declared,
    I32(i32),
    F32(f32),
}

impl fmt::Display for NumericObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericObject::F32(value) => write!(f, "{}", value),
            NumericObject::I32(value) => write!(f, "{}", value),
            NumericObject::Declared => Ok(()),
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
            _ => return Err(EvaluateError::VariableDoesNotHaveAValue)
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
            _ => return Err(EvaluateError::VariableDoesNotHaveAValue)
        }
    };
}

#[derive(Debug)]
pub enum EvaluateError {
    VariableDoesNotHaveAValue,
    UnexpectedUnaryOperator,
    UnexpectedBinaryOperator,
    AttemptToDivideByZero,
    ExpectedIdentifier,
    ErrUninitializedVariable,
}

pub fn evaluate(expression_tree: ASTNode, evaluation_env: &mut HashMap<String, NumericObject>) {
    let result = evaluate_expression(expression_tree, evaluation_env);
    match result {
        Ok(numeric_object_value) => match numeric_object_value {
            NumericObject::Declared => {}
            _ => println!("{}", numeric_object_value),
        },
        Err(err) => println!("Evaluation error: {:?}", err),
    }
}

fn evaluate_expression(
    expression_tree: ASTNode,
    evaluation_env: &mut HashMap<String, NumericObject>,
) -> Result<NumericObject, EvaluateError> {
    match expression_tree {
        ASTNode::Ident(identifier) => match evaluation_env.get(&identifier) {
            Some(value) => Ok(value.to_owned()),
            None => Err(EvaluateError::ErrUninitializedVariable),
        },
        ASTNode::I32(value) => Ok(NumericObject::I32(value)),
        ASTNode::F32(value) => Ok(NumericObject::F32(value)),
        ASTNode::UnaryExpr { op, inner } => match op {
            Operator::Negative => {
                let evaluated_object = evaluate_expression(*inner, evaluation_env).unwrap();
                let negative = match evaluated_object {
                    NumericObject::F32(value) => NumericObject::F32(-value),
                    NumericObject::I32(value) => NumericObject::I32(-value),
                    NumericObject::Declared => {
                        return Err(EvaluateError::VariableDoesNotHaveAValue)
                    }
                };
                Ok(negative)
            }
            _ => Err(EvaluateError::UnexpectedUnaryOperator),
        },
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            match op {
                Operator::Assign => match lhs.as_ref() {
                    ASTNode::Ident(value) => {
                        let rhs = evaluate_expression(*rhs, evaluation_env).unwrap();
                        evaluation_env.insert(value.clone(), rhs);
                        return Ok(NumericObject::Declared);
                    }
                    _ => return Err(EvaluateError::ExpectedIdentifier),
                },
                _ => {}
            }

            let lhs = evaluate_expression(*lhs, evaluation_env).unwrap();
            let rhs = evaluate_expression(*rhs, evaluation_env).unwrap();

            match op {
                Operator::Plus => normalize_numeric_operation!(lhs + rhs),
                Operator::Minus => normalize_numeric_operation!(lhs - rhs),
                Operator::Multiplication => normalize_numeric_operation!(lhs * rhs),
                Operator::Division => normalize_numeric_operation!(lhs / rhs),
                Operator::Exponential => match (lhs, rhs) {
                    (NumericObject::Declared, _) | (_, NumericObject::Declared) => {
                        return Err(EvaluateError::VariableDoesNotHaveAValue)
                    }

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
