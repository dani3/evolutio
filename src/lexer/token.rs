const KEYWORDS: [(&str, TokenType); 2] = [("fn", TokenType::Function), ("let", TokenType::Let)];

#[derive(PartialEq, Debug, Clone)]
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

pub struct Token<'a> {
    pub kind: TokenType,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, literal: &'a str) -> Self {
        Self { kind, literal }
    }

    pub fn lookup_ident(literal: &str) -> TokenType {
        for keyword in KEYWORDS.iter() {
            if keyword.0 == literal {
                return keyword.1.clone();
            }
        }

        TokenType::Ident
    }
}
