mod ast;
mod chunk;
mod compiler;
mod debug;
mod interner;
mod scanner;
mod value;
mod vm;

use clap::Parser as ClapParser;
use std::io::Write;
use vm::InterpretResult;

use crate::{ast::Parser, scanner::Lexer};

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

        // ======================== REPL ========================
        let mut lexer = Lexer::new(input.to_string());

        let out = Parser::new(&mut lexer).parse();
        println!("{:?}", out);

        let mut compiler = compiler::Compiler::new();
        let (bytecode, interner) = compiler.compile(out);

        println!("{:?}", bytecode);

        let debug = debug::Debug::new("test", bytecode.clone());
        debug.disassemble();

        let mut vm = vm::VM::init(bytecode, interner);
        let result = vm.run();
        // println!("{:?}", result);

        // ======================== REPL ========================
    }
}

pub fn run_source(src: &str) -> InterpretResult {
    let mut lexer = Lexer::new(src.to_string());

    let out = Parser::new(&mut lexer).parse();
    println!("{:?}", out);

    let mut compiler = compiler::Compiler::new();
    let (bytecode, interner) = compiler.compile(out);
    println!("{:?}", bytecode);

    let debug = debug::Debug::new("test", bytecode.clone());
    debug.disassemble();

    let mut vm = vm::VM::init(bytecode, interner);
    let result = vm.run();

    println!("{:?}", result);

    return result;
}

#[cfg(test)]
mod tests {
    use crate::{run_source, value::ValueType, vm::InterpretResult};

    #[test]
    fn test_micrograd_example() {
        let src = r#"
                        let a = -4.0;
                        let b = 2.0;
                        let c = a + b;
                        let d = a * b + b**3;
                        c += c + 1;
                        c += 1 + c + (-a);
                        print(c == -1);
                        "#;

        let out = run_source(&src);

        assert_eq!(
            out,
            InterpretResult::InterpretOk(vec![ValueType::Boolean(true)])
        );
    }
}
