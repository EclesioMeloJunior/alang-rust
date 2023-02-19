use crate::ast::{ASTNode, Operator};

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedUnaryOperator,
    UnexpectedBinaryOperator,
}

pub fn evaluate(expression_tree: ASTNode) {
    let result = evaluate_expression(expression_tree).unwrap();
    println!("{:?}", result);
}

fn evaluate_expression(expression_tree: ASTNode) -> Result<i32, EvaluateError> {
    match expression_tree {
        ASTNode::I32(value) => Ok(value),
        ASTNode::UnaryExpr { op, inner } => match op {
            Operator::Negative => Ok(-(evaluate_expression(*inner).unwrap())),
            _ => Err(EvaluateError::UnexpectedUnaryOperator),
        },
        ASTNode::BinaryExpr { op, lhs, rhs } => match op {
            Operator::Plus => {
                Ok(evaluate_expression(*lhs).unwrap() + evaluate_expression(*rhs).unwrap())
            }
            Operator::Minus => {
                Ok(evaluate_expression(*lhs).unwrap() - evaluate_expression(*rhs).unwrap())
            }
            Operator::Multiplication => {
                Ok(evaluate_expression(*lhs).unwrap() * evaluate_expression(*rhs).unwrap())
            }
            Operator::Division => {
                Ok(evaluate_expression(*lhs).unwrap() / evaluate_expression(*rhs).unwrap())
            }
            Operator::Exponential => {
                let lhs = evaluate_expression(*lhs).unwrap();
                let rhs = evaluate_expression(*rhs).unwrap();
                Ok(lhs.pow(rhs as u32))
            }
            _ => Err(EvaluateError::UnexpectedBinaryOperator),
        },
    }
}
