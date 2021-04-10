#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenType, literal: String) -> Self {
        Self { kind, literal }
    }
}
