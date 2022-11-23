#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,

    // Identifiers
    Identifier(String),
    Int(u32),
    String(String),

    // Operators
    Assign, // =
    Eq,
    NotEq,
    Bang,

    Plus,
    Minus,
    Asterisk,
    Slash,

    // Delimiters
    Comma,
    Semicolon,

    // Panrenthesis and Braces
    LParen,
    RParen,
    LBraces,
    RBraces,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
