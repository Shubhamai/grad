mod expr;
mod interpreter;
mod lexer;
mod parser;

use crate::{expr::ValueType, lexer::Token};

use clap::Parser as ClapParser;
use logos::Logos;
use std::io::Write;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path (optional)
    #[clap(value_hint = clap::ValueHint::AnyPath, default_value = "")]
    script: String,
}

fn main() {
    let args = Args::parse();

    // Check if args.script is provided
    if args.script.is_empty() {
        // run as a repl
        loop {
            // print prompt
            print!("> ");
            let _ = std::io::stdout().flush();

            // read input
            let mut input = String::new();
            input = match std::io::stdin().read_line(&mut input) {
                Ok(_) => input,
                Err(e) => panic!("Error reading input: {}", e),
            };

            // exit if input is "exit"
            if input.trim() == "exit" || input.trim() == "" {
                break;
            }

            run(&input);
        }
    } else {
        // read file

        let src = match std::fs::read_to_string(&args.script) {
            Ok(source) => source,
            Err(e) => panic!("Error reading file: {}", e),
        };

        run(&src);
    }
}

fn run(src: &str) {
    let mut lex = lexer::TokenType::lexer(&src);

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lex.next();

        match token {
            None => break,
            _ => {
                tokens.push(Token {
                    token_type: match token {
                        Some(_) => match token.clone().unwrap() {
                            Ok(token) => token,
                            Err(_) => panic!("Error parsing token"),
                        },
                        None => panic!("Error parsing token"),
                    },
                    literal: {
                        match token {
                            Some(Ok(lexer::TokenType::Number)) => {
                                Some(ValueType::Number(lex.slice().parse::<f64>().unwrap()))
                            }
                            Some(Ok(lexer::TokenType::String)) => {
                                Some(ValueType::String(lex.slice().to_owned()))
                            }
                            Some(Ok(lexer::TokenType::True)) => Some(ValueType::Boolean(true)),
                            Some(Ok(lexer::TokenType::False)) => Some(ValueType::Boolean(false)),
                            Some(Ok(lexer::TokenType::NIL)) => Some(ValueType::Nil),
                            _ => None,
                        }
                    },
                    lexeme: lex.slice().to_string(),
                    span: lex.span(),
                });
            }
        }
    }

    let mut parser = parser::Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(expr) => expr,
        Err(e) => panic!("Error parsing expression: {}", e),
    };

    let interpreter = interpreter::Interpreter::new();
    let result = interpreter.visit_expr(&expr);

    println!("{:?}", result);
}
