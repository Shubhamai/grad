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
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_source(src: &str) -> String {
    let mut lexer = Lexer::new(src.to_string());

    let out = Parser::new(&mut lexer).parse();
    for stmt in out.iter() {
        println!("{:?}", stmt);
    }
    println!("-------------");

    let mut compiler = compiler::Compiler::new();
    let (bytecode, interner) = compiler.compile(out);
    println!("{:?}", bytecode);

    let debug = debug::Debug::new("test", bytecode.clone(), interner.clone());
    debug.disassemble();

    let mut vm = vm::VM::init(bytecode, interner);
    let result = vm.run();

    format!("{:?}", result)
}

#[test]
fn test_run_source() {
    let src = "let a = 1 + 2 * 3;print(a);";
    let result = run_source(src);
    assert_eq!(result, "Ok([Tensor(7)])");
}
