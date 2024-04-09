use core::fmt;
use std::any::{self, Any};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum token_type {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    //Box<dyn Any>
    pub literal: String,
    pub line: u32,
    pub token_type: token_type,
    pub lexeme: String,
}

impl Token {
    pub fn new(literal: String, line: u32, token_type: token_type, lexeme: String) -> Self {
        return Self {
            literal,
            line,
            token_type,
            lexeme,
        };
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
