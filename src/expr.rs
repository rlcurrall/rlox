use crate::token::{Token, TokenLiteral};

pub(crate) struct Literal {
    value: TokenLiteral,
}

pub(crate) struct Logical {
    left: Literal,
    operator: Token,
    right: Literal,
}

pub(crate) enum Expr {
    Literal,
    Logical,
}
