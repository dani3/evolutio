mod ast;
mod lexer;
mod repl;

fn main() {
    repl::start(&mut std::io::stdin().lock(), &mut std::io::stdout().lock()).unwrap();
}
