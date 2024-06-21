use std::collections::HashMap;
use thiserror::Error;

use crate::{
    chunk::{self, Chunk, VectorType},
    interner::{Interner, StringObjIdx},
    tensor::Tensor,
    value::ValueType,
};

const STACK_MAX: usize = 256;

struct CallFrame {
    ip: usize,
    stack_top: usize,
}

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

    call_frames: Vec<CallFrame>,
    frame_index: usize,
}

#[derive(Debug, PartialEq, Error)]
pub enum Result {
    #[error("Ok")]
    Ok(Vec<ValueType>),

    #[error("Compile error : {0}")]
    CompileErr(String),

    #[error("Runtime error : {0}")]
    RuntimeErr(String),
}

impl VM {
    pub(crate) fn init(chunk: Chunk, interner: Interner) -> VM {
        // TODO: serialize and cache chunk and interner and save it as a file hash
        VM {
            chunk,
            ip: 0,
            stack: core::array::from_fn(|_| ValueType::Nil),
            stack_top: 0,
            interner,
            globals: HashMap::new(),
            call_frames: Vec::new(),
            frame_index: 0,
        }
    }

    pub fn run(&mut self) -> Result {
        let mut print_outputs = Vec::new();

        macro_rules! push {
            ($value:expr) => {
                self.push($value)
            };
        }

        macro_rules! pop {
            () => {
                self.pop()
            };
        }

        macro_rules! opcode {
            ($op:ident) => {
                chunk::VectorType::Code(chunk::OpCode::$op)
            };
        }

        macro_rules! get_constant {
            ($index:expr) => {
                match $index {
                    chunk::VectorType::Constant(idx) => self.read_constant(idx as usize),
                    _ => {
                        return Result::RuntimeErr(format!("Invalid constant '{}'", $index));
                    }
                }
            };
        }

        loop {
            let instruction = self.read_byte();

            match instruction {
                opcode!(OpReturn) => {
                    return Result::Ok(print_outputs);
                }
                opcode!(OpAdd) => {
                    if let ValueType::String(_) = self.peek(0) {
                        self.concatenate();
                    } else {
                        let b = pop!();
                        let a = pop!();
                        push!(a + b);
                    }
                }
                opcode!(OpSubtract) => {
                    let b = pop!();
                    let a = pop!();
                    push!(a - b);
                }
                opcode!(OpMultiply) => {
                    let b = pop!();
                    let a = pop!();
                    push!(a * b);
                }
                opcode!(OpDivide) => {
                    let b = pop!();
                    let a = pop!();
                    push!(a / b);
                }
                opcode!(OpPower) => {
                    let b = pop!();
                    let a = pop!();
                    push!(a.pow(&b));
                }
                opcode!(OpNegate) => {
                    let value = pop!();
                    push!(-value);
                }
                opcode!(OpNil) => push!(ValueType::Nil),
                opcode!(OpTrue) => push!(ValueType::Boolean(true)),
                opcode!(OpFalse) => push!(ValueType::Boolean(false)),
                opcode!(OpNot) => {
                    let value = pop!();
                    push!(!value)
                }
                opcode!(OpEqualEqual) => {
                    let b = pop!();
                    let a = pop!();
                    push!(ValueType::Boolean(a == b));
                }
                // TODO: Not working for now
                opcode!(OpGreater) => {
                    let b = pop!();
                    let a = pop!();
                    push!(ValueType::Boolean(a > b));
                }
                opcode!(OpLess) => {
                    let b = pop!();
                    let a = pop!();
                    push!(ValueType::Boolean(a < b));
                }
                opcode!(OpPrint) => {
                    let value = pop!();

                    print_outputs.push(value.clone());
                    println!("{}", value.display(&self.interner));
                }
                opcode!(OpPop) => {
                    pop!();
                }
                opcode!(OpConstant) => {
                    let constant = get_constant!(self.read_byte());
                    push!(constant);
                }
                opcode!(OpJumpIfFalse) => {
                    self.read_byte();
                    let offset = self.read_byte();
                    let value = self.peek(0);

                    if let ValueType::Boolean(false) = value {
                        if let VectorType::Constant(idx) = offset {
                            if let ValueType::JumpOffset(offset) = self.read_constant(idx as usize)
                            {
                                self.ip = offset;
                            }
                        }
                    }
                }
                opcode!(OpJump) => {
                    self.read_byte();
                    let offset = self.read_byte();
                    if let VectorType::Constant(idx) = offset {
                        if let ValueType::JumpOffset(offset) = self.read_constant(idx as usize) {
                            self.ip = offset
                        }
                    }
                }
                opcode!(OpLoop) => {
                    self.read_byte();
                    let offset = self.read_byte();
                    if let VectorType::Constant(idx) = offset {
                        if let ValueType::JumpOffset(offset) = self.read_constant(idx as usize) {
                            self.ip = offset
                        }
                    }
                }
                opcode!(OpDefineGlobal) => {
                    let constant = get_constant!(self.read_byte());
                    let value = self.peek(0);

                    if let ValueType::Identifier(idx) = constant {
                        self.globals.insert(idx, value);
                    }

                    pop!();
                }
                opcode!(OpGetGlobal) => {
                    let constant = get_constant!(self.read_byte());
                    match constant {
                        ValueType::Identifier(idx) => {
                            let value = self.globals.get(&idx);
                            if let Some(value) = value {
                                push!(value.clone());
                            } else {
                                return Result::RuntimeErr(format!(
                                    "Undefined variable '{}'",
                                    self.interner.lookup(idx)
                                ));
                            }
                        }
                        _ => {
                            return Result::RuntimeErr(format!(
                                "Invalid global variable '{}'",
                                constant.display(&self.interner)
                            ));
                        }
                    }
                }
                opcode!(OpSetGlobal) => {
                    let index = self.read_byte();
                    let constant = get_constant!(index);

                    match constant {
                        ValueType::Identifier(idx) => {
                            let value = self.peek(0);
                            self.globals.insert(idx, value);
                            // TODO - only set the value if it exists
                        }
                        _ => {
                            return Result::RuntimeErr(format!(
                                "Invalid global variable '{}'",
                                constant.display(&self.interner)
                            ));
                        }
                    }
                }
                opcode!(OpGetLocal) => {
                    let slot = self.read_byte();

                    match slot {
                        VectorType::Constant(idx) => {
                            let value = self.stack[idx as usize].clone();
                            push!(value);
                        }
                        _ => {
                            return Result::RuntimeErr(format!("Invalid slot '{}'", slot));
                        }
                    }
                }
                opcode!(OpSetLocal) => {
                    let slot = self.read_byte();

                    match slot {
                        VectorType::Constant(idx) => {
                            let value = self.peek(0);
                            self.stack[idx as usize] = value;
                        }
                        _ => {
                            return Result::RuntimeErr(format!("Invalid slot '{}'", slot));
                        }
                    }
                }
                // opcode!(OpCall) => {
                //     let callee = self.read_byte();
                //     let caller = pop!();

                //     let constant = get_constant!(callee);
                //     let str_idx = match constant {
                //         ValueType::Identifier(idx) => idx,
                //         _ => {
                //             return Result::RuntimeErr("Invalid function".to_string());
                //         }
                //     };
                //     let calle_str = self.interner.lookup(str_idx);

                //     let tensor = match caller {
                //         ValueType::Tensor(tensor) => tensor,
                //         _ => {
                //             return Result::RuntimeErr("Invalid function".to_string());
                //         }
                //     };

                //     match calle_str {
                //         "relu" => push!(ValueType::Tensor(Tensor::from(tensor.relu()))),
                //         "backward" => tensor.backward(),
                //         "grad" => push!(ValueType::Tensor(Tensor::from(tensor.gradient()))),
                //         _ => {
                //             return Result::RuntimeErr("Undefined function. Currently only supports relu, backward and grad".to_string());
                //         }
                //     }
                // }
                _ => {
                    return {
                        if let chunk::VectorType::Constant(idx) = instruction {
                            let value = self.read_constant(idx as usize);
                            println!("Constant: {:?}", value);
                        }

                        Result::RuntimeErr(format!("Invalid opcode '{}'", instruction))
                    };
                }
            }
        }
    }

    fn read_byte(&mut self) -> VectorType {
        let byte = self.chunk.code[self.ip].clone();
        self.ip += 1;
        return byte;
    }

    fn read_constant(&mut self, index: usize) -> ValueType {
        self.chunk.constants[index].clone()
    }

    fn push(&mut self, value: ValueType) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> ValueType {
        self.stack_top -= 1;
        self.stack[self.stack_top].clone()
    }

    fn peek(&self, distance: usize) -> ValueType {
        self.stack[self.stack_top - 1 - distance].clone()
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
