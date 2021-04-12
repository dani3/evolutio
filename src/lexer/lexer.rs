use super::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    next_pos: usize,
    ch: u8,
}

fn is_letter(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
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

    fn read_ident(&mut self) -> &str {
        let current = self.current_pos;
        while is_letter(self.ch as char) {
            self.read_char();
        }

        &self.input[current..self.current_pos]
    }

    fn read_number(&mut self) -> &str {
        let current = self.current_pos;
        while (self.ch as char).is_numeric() {
            self.read_char();
        }

        &self.input[current..self.current_pos]
    }

    fn skip_whitespace(&mut self) {
        while self.ch as char == '\t'
            || self.ch as char == '\n'
            || self.ch as char == '\r'
            || self.ch as char == ' '
        {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        if self.ch == 0 {
            token = Token::new(TokenType::Eof, "");
        } else {
            self.skip_whitespace();

            token = match self.ch as char {
                '=' => Token::new(
                    TokenType::Assign,
                    &self.input[self.current_pos..self.next_pos],
                ),
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
                '!' => Token::new(TokenType::Not, &self.input[self.current_pos..self.next_pos]),
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
            5 < 10 > 5;",
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
