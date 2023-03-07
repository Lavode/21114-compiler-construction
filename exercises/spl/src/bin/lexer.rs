use spl::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("Test");

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokenization successful. Tokens:");
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(errors) => {
            eprintln!("Tokenization failed. Tokenization errors:");
            for e in errors {
                println!("{}", e);
            }
        }
    }
}
