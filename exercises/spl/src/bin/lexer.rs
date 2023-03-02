use spl::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("Test");

    lexer.tokenize();
}
