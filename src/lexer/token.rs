//! # Token
//!
//! Data structure that holds a first representation of the source code.

// Array containing all the language keywords.
const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", TokenType::Function),
    ("let", TokenType::Let),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("return", TokenType::Return),
    ("true", TokenType::True),
    ("false", TokenType::False),
];

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Not,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Neq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    If,
    Else,
    True,
    False,
    Return,
}

/// Data structure that holds the type of token and the literal associated.
/// This tokens are later fed to the parser.
pub struct Token<'a> {
    pub kind: TokenType,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    /// Constructs a new `Token`.
    pub fn new(kind: TokenType, literal: &'a str) -> Self {
        Self { kind, literal }
    }

    /// Returns whether is a _keyword_ or an _identifier_ given a literal.
    pub fn lookup_ident(literal: &str) -> TokenType {
        for keyword in KEYWORDS.iter() {
            if keyword.0 == literal {
                return keyword.1.clone();
            }
        }

        TokenType::Ident
    }
}
