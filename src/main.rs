mod chunk;
// mod compiler;
// mod debug;
mod scanner;
mod value;
// mod vm;
mod interner;
mod ast;

use ast::expr;
use clap::Parser as ClapParser;
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

        run_repl();
    } else {
        // read file

        let src = match std::fs::read_to_string(&args.script) {
            Ok(source) => source,
            Err(e) => panic!("Error reading file: {}", e),
        };

        run_source(&src);
    }
}

fn run_repl() {
    // let mut vm = vm::VM::init();

    // loop {
    //     // print prompt
    //     print!("> ");
    //     let _ = std::io::stdout().flush();

    //     // read input
    //     let mut input = String::new();
    //     input = match std::io::stdin().read_line(&mut input) {
    //         Ok(_) => input,
    //         Err(e) => panic!("Error reading input: {}", e),
    //     };

    //     // exit if input is "exit"
    //     if input.trim() == "exit" || input.trim() == "" {
    //         break;
    //     }

    //     // run source
    //     vm.interpret(&input);
    // }
}

fn run_source(src: &str) {

    // let mut lexer = Lexer::new(source.to_string());
    let out = expr(src);
    println!("{}", out);
    println!("{:?}", out);

    // let mut vm = vm::VM::init();
    // let result = vm.interpret(src);

 
 
}
