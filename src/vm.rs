use std::collections::HashMap;

use crate::{
    chunk::{self, Chunk, VectorType},
    debug,
    interner::{Interner, StringObjIdx},
    value::ValueType,
};

const STACK_MAX: usize = 256;

pub(crate) struct VM {
    pub chunk: Chunk,

    // instruction pointer
    ip: usize,

    // TODO - implement JIT instead of stack perhaps ?
    // NOTE - using a fixed size array for the stack instead of a Vec
    stack: [ValueType; STACK_MAX],
    stack_top: usize,

    pub interner: Interner,

    globals: HashMap<StringObjIdx, ValueType>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    InterpretOk(Vec<ValueType>),
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    // pub(crate) fn init(chunk: Chunk) -> VM {
    pub(crate) fn init(chunk: Chunk, interner: Interner) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: core::array::from_fn(|i| ValueType::Nil),
            stack_top: 0,
            interner,
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        let mut print_outputs = Vec::new();

        loop {
            // debug
            // for i in 0..self.stack_top {
            //     print!("{} ", self.stack[i]);
            // }
            // let debug = debug::Debug::new(&format!("{}", self.ip), self.chunk.clone());
            // debug.disassemble_instruction(self.ip);

            let instruction = self.read_byte();

            match instruction {
                chunk::VectorType::Code(chunk::OpCode::OpReturn) => {
                    // println!("{}", self.pop());
                    return InterpretResult::InterpretOk(print_outputs);
                }
                chunk::VectorType::Code(chunk::OpCode::OpAdd) => {
                    if let ValueType::String(_) = self.peek(0) {
                        self.concatenate();
                    } else {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a + b);
                    }
                }
                chunk::VectorType::Code(chunk::OpCode::OpSubtract) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                chunk::VectorType::Code(chunk::OpCode::OpMultiply) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                chunk::VectorType::Code(chunk::OpCode::OpDivide) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
                chunk::VectorType::Code(chunk::OpCode::OpPower) => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (ValueType::Tensor(a), ValueType::Tensor(b)) => {
                            self.push(ValueType::Tensor(a.powf(b)));
                        }
                        _ => {
                            println!("Operands must be numbers.");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                }
                chunk::VectorType::Code(chunk::OpCode::OpNegate) => {
                    let value = self.pop();
                    self.push(-value);
                }
                chunk::VectorType::Code(chunk::OpCode::OpNil) => {
                    self.push(ValueType::Nil);
                }
                chunk::VectorType::Code(chunk::OpCode::OpTrue) => {
                    self.push(ValueType::Boolean(true));
                }
                chunk::VectorType::Code(chunk::OpCode::OpFalse) => {
                    self.push(ValueType::Boolean(false));
                }
                chunk::VectorType::Code(chunk::OpCode::OpNot) => {
                    let value = self.pop();
                    self.push(!value);
                }
                chunk::VectorType::Code(chunk::OpCode::OpEqualEqual) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a == b));
                }
                chunk::VectorType::Code(chunk::OpCode::OpGreater) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a > b));
                }
                chunk::VectorType::Code(chunk::OpCode::OpLess) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a < b));
                }
                chunk::VectorType::Code(chunk::OpCode::OpPrint) => {
                    let value = self.pop();
                    print_outputs.push(value);
                    println!("{}", value);
                }
                chunk::VectorType::Code(chunk::OpCode::OpPop) => {
                    self.pop();
                }
                chunk::VectorType::Code(chunk::OpCode::OpConstant) => {
                    let index = self.read_byte();

                    match index {
                        chunk::VectorType::Constant(idx) => {
                            let constant = self.read_constant(idx as usize);
                            self.push(constant);
                        }
                        _ => {
                            println!("op constant: Invalid constant index : {:?}", index);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                }
                chunk::VectorType::Code(chunk::OpCode::OpDefineGlobal) => {
                    let index = self.read_byte();
                    let constant = match index {
                        chunk::VectorType::Constant(idx) => self.read_constant(idx as usize),
                        _ => {
                            println!("define global: Invalid constant index : {:?}", index);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };

                    let value = self.peek(0);

                    if let ValueType::Identifier(idx) = constant {
                        self.globals.insert(idx, value);
                    }

                    self.pop();
                }
                chunk::VectorType::Code(chunk::OpCode::OpGetGlobal) => {
                    let index = self.read_byte();
                    let constant = match index {
                        chunk::VectorType::Constant(idx) => self.read_constant(idx as usize),
                        _ => {
                            println!("get global: Invalid constant index : {:?}", index);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };
                    match constant {
                        ValueType::Identifier(idx) => {
                            let value = self.globals.get(&idx);
                            match value {
                                Some(value) => {
                                    self.push(*value);
                                }
                                None => {
                                    println!("Undefined variable");
                                    return InterpretResult::InterpretRuntimeError;
                                }
                            }
                        }
                        _ => {
                            println!("Invalid global variable");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                }
                chunk::VectorType::Code(chunk::OpCode::OpSetGlobal) => {
                    let index = self.read_byte();
                    let constant = match index {
                        chunk::VectorType::Constant(idx) => self.read_constant(idx as usize),
                        _ => {
                            println!("set global: Invalid constant index : {:?}", index);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };

                    match constant {
                        ValueType::Identifier(idx) => {
                            let value = self.peek(0);
                            self.globals.insert(idx, value);
                            // TODO - only set the value if it exists
                        }
                        _ => {
                            println!("Invalid global variable");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                }
                chunk::VectorType::Code(chunk::OpCode::OpCall) => {
                    // relu
                    let callee = self.read_byte();

                    // a ?
                    let caller = self.pop();

                    let str_idx = match callee {
                        VectorType::Constant(idx) => match self.read_constant(idx as usize) {
                            ValueType::Identifier(idx) => idx,
                            _ => {
                                println!("Invalid function");
                                return InterpretResult::InterpretRuntimeError;
                            }
                        },
                        _ => {
                            println!("Invalid function");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };
                    let calle_str = self.interner.lookup(str_idx);

                    match calle_str {
                        "relu" => match caller {
                            ValueType::Tensor(a) => {
                                self.push(ValueType::Tensor(a.relu()));
                            }
                            _ => {
                                println!("Invalid function");
                                return InterpretResult::InterpretRuntimeError;
                            }
                        },
                        _ => {
                            println!("Undefined function");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                }
                VectorType::Constant(_) => {}
            }
        }
    }

    // The READ_BYTE macro reads the byte currently pointed at by ip and then advances the instruction pointer - book
    fn read_byte(&mut self) -> VectorType {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        return byte;
    }

    fn read_constant(&mut self, index: usize) -> ValueType {
        self.chunk.constants[index]
    }

    fn push(&mut self, value: ValueType) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> ValueType {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn peek(&self, distance: usize) -> ValueType {
        self.stack[self.stack_top - 1 - distance]
    }

    fn concatenate(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if let ValueType::String(a) = a {
            if let ValueType::String(b) = b {
                let b_str = self.interner.lookup(b);
                let a_str = self.interner.lookup(a);
                let res = a_str.to_owned() + b_str;
                let res_idx = self.interner.intern_string(res);
                self.push(ValueType::String(res_idx));
            }
        }
    }
}
