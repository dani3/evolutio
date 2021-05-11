mod lexer;
mod repl;

use lexer::Lexer;

fn main() {
    repl::start(&mut std::io::stdin().lock(), &mut std::io::stdout().lock()).unwrap();
}
