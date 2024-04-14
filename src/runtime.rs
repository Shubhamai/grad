use crate::lexer;
use logos::Logos;

pub fn run(src: &str) {
    // let mut lexer = Lexer::new(src);
    // let tokens = lexer.scan_tokens();

    let mut lexer = lexer::TokenType::lexer(&src);

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next();

        match token {
            None => break,
            _ => tokens.push(token.unwrap()),
        }
    }

    println!("{:?}", tokens);
}
