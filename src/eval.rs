use crate::ast::{Node, Operator};

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedUnaryOperator,
    UnexpectedBinaryOperator,
}

pub fn evaluate(expression_tree: Node) {
    let result = evaluate_expression(expression_tree).unwrap();
    println!("{:?}", result);
}

fn evaluate_expression(expression_tree: Node) -> Result<i32, EvaluateError> {
    match expression_tree {
        Node::I32(value) => Ok(value),
        Node::UnaryExpr { op, inner } => match op {
            Operator::Negative => Ok(-(evaluate_expression(*inner).unwrap())),
            _ => Err(EvaluateError::UnexpectedUnaryOperator),
        },
        Node::BinaryExpr { op, lhs, rhs } => match op {
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
            _ => Err(EvaluateError::UnexpectedBinaryOperator),
        },
    }
}
