use crate::lexer::{Lexer, Token};

struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Option<Token<'a>>,
    peek_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser: Parser = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        parser.next_token();

        parser
    }

    pub fn next_token(&'a mut self) {
        self.peek_token = Some(self.lexer.next_token());
    }
}
