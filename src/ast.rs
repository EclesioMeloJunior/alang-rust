#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
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
