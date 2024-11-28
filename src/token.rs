use crate::intorfloat::IntOrFloat;

#[derive(Debug)]
pub enum Token {
    Number{num: IntOrFloat},

    Plus,
    Minus,
    Star,
    Slash,

    LeftParen,
    RightParen,
}
