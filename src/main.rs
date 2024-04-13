use clap::Parser as ClapParser;
// TODO: Repl support ?

mod lexer;
mod parser;
mod runtime;

use chumsky::prelude::*;
use chumsky::Parser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path
    #[clap(value_hint = clap::ValueHint::AnyPath)]
    script: String,
}

fn main() {
    let args = Args::parse();
    let src = match std::fs::read_to_string(&args.script) {
        Ok(source) => source,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let (tokens, mut errs) = lexer::lexer().parse(src.as_str()).into_output_errors();


    let parse_errs = if let Some(tokens) = &tokens {

        let (ast, parse_errs) = parser::expr_parser()
            // .map_with(|ast, e| (ast, e.span()))
            .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
            .into_output_errors();

        println!("{:?}", ast);
        println!("{:?}", parse_errs);

        if let Some(ast) = ast {
            let mut funcs = Vec::new();
            let mut stack = Vec::new();
            let res = runtime::eval_expr(&ast, &mut funcs, &mut stack);
            println!("{:?}", res);
        }
    };


}
