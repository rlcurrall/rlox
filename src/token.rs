#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub value: TokenValue,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(value: TokenValue, lexeme: String, line: usize) -> Self {
        Self {
            value,
            lexeme,
            line,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TokenValue {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
