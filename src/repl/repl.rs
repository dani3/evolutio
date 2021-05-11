use std::io::{BufRead, Read, Write};

use crate::lexer::Lexer;
use crate::lexer::TokenType;

const PROMPT: &[u8] = b">> ";

pub fn start<T, K>(input: &mut T, output: &mut K) -> std::io::Result<()>
where
    T: Read + BufRead,
    K: Write,
{
    let mut buf: String = String::default();

    output.write_all(b"Evolutio, the Monkey interpreter:\n")?;
    output.flush()?;

    loop {
        buf.clear();

        output.write_all(PROMPT)?;
        output.flush()?;
        input.read_line(&mut buf)?;

        let mut lexer: Lexer = Lexer::new(&buf.trim());
        loop {
            let token = lexer.next_token();
            if token.kind == TokenType::Eof {
                break;
            } else {
            }
        }
    }
}
