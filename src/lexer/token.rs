#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    F32(f32),
    I32(i32),

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,

    Caret,

    Let,
    Assign,
}
