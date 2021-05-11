//! # Lexer
//!
//! In order to work with source code, it needs to be turned into a more accessible form. The first transformation, from source code to tokens, is called _lexical analysis_, or _lexing_. This is done by the `lexer`.

use super::token::{Token, TokenType};

/// Struct that represent the Lexer.
pub struct Lexer<'a> {
    /// Input source code.
    input: &'a str,
    /// Current position in the source code.
    current_pos: usize,
    /// Next position to be read from the source code.
    next_pos: usize,
    /// Current char read from the source code stored as a `u8`.
    ch: u8,
}

fn is_letter(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

impl<'a> Lexer<'a> {
    /// Constructs a new `Lexer` object.
    ///
    /// # Arguments
    ///
    /// * `input` - reference to the input source code to be lexed.
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

    /// Reads the following char.
    fn read_char(&mut self) {
        match self.input.chars().nth(self.next_pos) {
            Some(c) => self.ch = c as u8,
            None => self.ch = 0,
        };

        self.current_pos = self.next_pos;
        self.next_pos += 1;
    }

    /// Returns a reference to an identifer that starts with the current char.
    fn read_ident(&mut self) -> &str {
        let current = self.current_pos;
        while is_letter(self.ch as char) {
            self.read_char();
        }

        &self.input[current..self.current_pos]
    }

    /// Returns a reference to the number that starts with the current char.
    fn read_number(&mut self) -> &str {
        let current = self.current_pos;
        while (self.ch as char).is_numeric() {
            self.read_char();
        }

        &self.input[current..self.current_pos]
    }

    /// Skips whitespaces and empty lines.
    fn skip_whitespace(&mut self) {
        while self.ch as char == '\t'
            || self.ch as char == '\n'
            || self.ch as char == '\r'
            || self.ch as char == ' '
        {
            self.read_char();
        }
    }

    /// Returns the following char without moving the current pointer.
    fn peek_char(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.next_pos).unwrap())
        }
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Token {
        let token: Token;
        if self.ch == 0 {
            token = Token::new(TokenType::Eof, "");
        } else {
            self.skip_whitespace();

            token = match self.ch as char {
                '=' => {
                    if let Some(i) = self.peek_char() {
                        if i == '=' {
                            self.read_char();
                            Token::new(
                                TokenType::Eq,
                                &self.input[(self.current_pos - 1)..self.next_pos],
                            )
                        } else {
                            Token::new(
                                TokenType::Assign,
                                &self.input[self.current_pos..self.next_pos],
                            )
                        }
                    } else {
                        Token::new(
                            TokenType::Assign,
                            &self.input[self.current_pos..self.next_pos],
                        )
                    }
                }
                ';' => Token::new(
                    TokenType::Semicolon,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '+' => Token::new(
                    TokenType::Plus,
                    &self.input[self.current_pos..self.next_pos],
                ),
                ',' => Token::new(
                    TokenType::Comma,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '(' => Token::new(
                    TokenType::LParen,
                    &self.input[self.current_pos..self.next_pos],
                ),
                ')' => Token::new(
                    TokenType::RParen,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '{' => Token::new(
                    TokenType::LBrace,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '}' => Token::new(
                    TokenType::RBrace,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '!' => {
                    if let Some(i) = self.peek_char() {
                        if i == '=' {
                            self.read_char();
                            Token::new(
                                TokenType::Neq,
                                &self.input[(self.current_pos - 1)..self.next_pos],
                            )
                        } else {
                            Token::new(TokenType::Not, &self.input[self.current_pos..self.next_pos])
                        }
                    } else {
                        Token::new(TokenType::Not, &self.input[self.current_pos..self.next_pos])
                    }
                }
                '-' => Token::new(
                    TokenType::Minus,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '>' => Token::new(TokenType::Gt, &self.input[self.current_pos..self.next_pos]),
                '<' => Token::new(TokenType::Lt, &self.input[self.current_pos..self.next_pos]),
                '*' => Token::new(
                    TokenType::Asterisk,
                    &self.input[self.current_pos..self.next_pos],
                ),
                '/' => Token::new(
                    TokenType::Slash,
                    &self.input[self.current_pos..self.next_pos],
                ),
                _ => {
                    if is_letter(self.ch as char) {
                        let literal: &str = self.read_ident();
                        let kind: TokenType = Token::lookup_ident(literal);

                        return Token::new(kind, literal);
                    } else if (self.ch as char).is_numeric() {
                        let literal: &str = self.read_number();

                        return Token::new(TokenType::Int, literal);
                    } else {
                        Token::new(
                            TokenType::Illegal,
                            &self.input[self.current_pos..self.next_pos],
                        )
                    }
                }
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

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;",
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
            (TokenType::Not, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::Neq, "!="),
            (TokenType::Int, "9"),
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
