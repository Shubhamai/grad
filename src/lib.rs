pub mod ast;
pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod interner;
pub mod scanner;
pub mod tensor;
pub mod value;
pub mod vm;

use crate::vm::Result::{CompileErr, Ok, RuntimeErr};
use crate::{ast::Parser, scanner::Lexer};

use ast::{ast_to_ascii, ASTNode};
use wasm_bindgen::prelude::*;


/// `wasm-pack build -t web`
#[wasm_bindgen]
pub fn run_source(src: &str) -> Vec<String> {
    let mut lexer = Lexer::new(src.to_string());

    let out = Parser::new(&mut lexer).parse().unwrap();
    // for stmt in out.iter() {
    //     println!("{:?}", stmt);
    // }
    // println!("-------------");

    let mut compiler = compiler::Compiler::new();
    let (bytecode, interner) = compiler.compile(out.clone());
    // println!("{:?}", bytecode);

    let debug = debug::Debug::new("test", bytecode.clone(), interner.clone());
    let disassemble_output = debug.disassemble();

    let mut vm = vm::VM::init(bytecode, interner);
    let result = vm.run();

    match result {
        Ok(v) => {
            // join string to /n
            let mut result = String::new();
            for i in v.iter() {
                result.push_str(&format!("{:?}\n", i));
            }

            // AST to ascii
            let mut ast_output = String::new();
            for stmt in out.iter() {
                ast_output.push_str(&ast_to_ascii(stmt, 0));
            }

            vec![result, disassemble_output, ast_output]
        }
        // CompileErr(e) => format!("CompileError({:?})", e),
        CompileErr(e) => vec![format!("CompileError({:?})", e), disassemble_output],
        // RuntimeErr(e) => format!("RuntimeError({:?})", e),
        RuntimeErr(e) => vec![format!("RuntimeError({:?})", e), disassemble_output],
    }
}

// #[test]
// fn test_run_source() {
//     let src = "let a = 1 + 2 * 3;print(a);";
//     let result = run_source(src);
//     assert_eq!(result, "Ok([Tensor(7)])");
// }

