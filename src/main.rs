mod ast;
mod chunk;
mod compiler;
mod debug;
mod interner;
mod scanner;
mod tensor;
mod value;
mod vm;

use crate::{ast::Parser, scanner::Lexer};
use ast::ast_to_ascii;
use clap::Parser as ClapParser;
use vm::Result;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path (optional)
    #[clap(value_hint = clap::ValueHint::AnyPath, default_value = "")]
    script: String,

    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Check if args.script is provided
    if args.script.is_empty() {
        // run as a repl
        // run_repl();

        panic!("REPL is not supported yet.");
    } else {
        // read file

        let src = match std::fs::read_to_string(&args.script) {
            Ok(source) => source,
            Err(e) => panic!("Error reading file: {}", e),
        };

        run_source(&src, args.debug);
    }
}

pub fn run_source(src: &str, debug: bool) -> Result {
    let mut lexer = Lexer::new(src.to_string());

    if debug {
        println!("============= Tokens =============");
        for token in lexer.tokens.iter().rev() {
            println!("{:?}", token);
        }
    };

    let out = Parser::new(&mut lexer).parse().unwrap();

    if debug {
        println!("============= AST =============");
        let mut ast_output = String::new();
        for stmt in out.iter() {
            ast_output.push_str(&ast_to_ascii(stmt, 0));
        }
        println!("{}", ast_output);
    }

    let mut compiler = compiler::Compiler::new();
    let (bytecode, interner) = compiler.compile(out);

    if debug {
        println!("============= Bytecode =============");
        println!("{:?}", bytecode);
    }

    let debugger = debug::Debug::new("test", bytecode.clone(), interner.clone());

    if debug {
        println!("{}", debugger.disassemble());
    }

    let mut vm = vm::VM::init(bytecode, interner);
    let result = vm.run();

    return result;
}

#[cfg(test)]
mod tests {
    use crate::{run_source, tensor::Tensor, value::ValueType, vm::Result};

    #[test]
    fn test_micrograd_example() {
        let src = r#"
                        let a = -4.0;
                        let b = 2.0;
                        let c = a + b;
                        let d = a * b + b**3;
                        c += c + 1;
                        c += 1 + c + (-a);
                        d += d * 2 + (b + a).relu();
                        d += 3 * d + (b - a).relu();
                        let e = c - d;
                        let f = e**2;
                        let g = f / 2.0;
                        g += 10.0 / f;        
                        print(g) // prints 24.7041, the outcome of this forward pass
                        "#;

        let out = run_source(&src, false);

        // assert_eq!(
        //     out,
        //     Result::Ok(vec![ValueType::Tensor(Tensor::from(24.70408163265306))])
        // );
    }

    // #[test]
    // fn test_scopes() {
    //     let src = r#"
    //     let a = 4;
    //     {
    //         let b = 5;
    //         print(b);
    //         {
    //             let c = 10;
    //             print(c);
    //             let b = 353;
    //             print(b);
    //         }
    //         print(b);
    //         b = 11;
    //         print(b);
    //         a = 12;
    //     }
    //     print(a);
    //     "#;

    //     let out = run_source(&src, false);

    //     assert_eq!(
    //         out,
    //         Result::Ok(vec![
    //             ValueType::Tensor(Tensor::from(5.0)),
    //             ValueType::Tensor(Tensor::from(10.0)),
    //             ValueType::Tensor(Tensor::from(353.0)),
    //             ValueType::Tensor(Tensor::from(5.0)),
    //             ValueType::Tensor(Tensor::from(11.0)),
    //             ValueType::Tensor(Tensor::from(12.0))
    //         ])
    //     );
    // }
}
