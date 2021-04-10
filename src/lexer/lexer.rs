use std::cell::Cell;

use super::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut s = Self {
            input,
            current_pos: 0,
            next_pos: 0,
            ch: 0,
        };

        s.read_char();

        s
    }

    fn read_char(&mut self) {
        match self.input.chars().nth(self.next_pos) {
            Some(c) => self.ch = c as u8,
            None => self.ch = 0,
        };

        self.current_pos = self.next_pos;
        self.next_pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        if self.ch == 0 {
            token = Token::new(TokenType::Eof, String::new());
        } else {
            token = match self.ch as char {
                '=' => Token::new(TokenType::Assign, (self.ch as char).to_string()),
                ';' => Token::new(TokenType::Semicolon, (self.ch as char).to_string()),
                '+' => Token::new(TokenType::Plus, (self.ch as char).to_string()),
                ',' => Token::new(TokenType::Comma, (self.ch as char).to_string()),
                '(' => Token::new(TokenType::LParen, (self.ch as char).to_string()),
                ')' => Token::new(TokenType::RParen, (self.ch as char).to_string()),
                '{' => Token::new(TokenType::LBrace, (self.ch as char).to_string()),
                '}' => Token::new(TokenType::RBrace, (self.ch as char).to_string()),
                _ => todo!(),
            };
        }

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Token, TokenType};

    #[test]
    fn next_token() {
        let input: String = String::from(
            "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y
            };

            let result = add(five, ten);",
        );

        let tests: Vec<(TokenType, &str)> = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer: Lexer = Lexer::new(&input);

        for (k, l) in tests {
            let token: Token = lexer.next_token();

            assert_eq!(token.kind, k);
            assert_eq!(token.literal, l);
        }
    }
}
