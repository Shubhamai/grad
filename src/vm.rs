use crate::{
    chunk::{self, Chunk, OpCode},
    compiler::Compiler,
    debug::Disassemble,
    interner::Interner,
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

    interner: Interner,
}

pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl VM {
    // pub(crate) fn init(chunk: Chunk) -> VM {
    pub(crate) fn init() -> VM {
        let chunk = Chunk::new();

        VM {
            chunk,
            ip: 0,
            stack: core::array::from_fn(|i| ValueType::Nil),
            stack_top: 0,
            interner: Interner::default(),
        }
    }

    // delete the vm and free the chunk
    pub(crate) fn free(&mut self) {
        todo!()
    }

    pub(crate) fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(String::from(source), &mut self.interner);

        let compiled_output = compiler.compile();
        let chunk = match compiled_output {
            Ok(chunk) => chunk,
            Err(e) => {
                self.free();
                return InterpretResult::INTERPRET_COMPILE_ERROR;
            }
        };

        self.chunk = chunk;
        self.ip = 0;

        let result = self.run();

        self.chunk.free();
        result
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            // debug
            // for i in 0..self.stack_top {
            //     print!("{} ", self.stack[i]);
            // }
            // self.chunk.disassemble_instruction(self.ip);

            let instruction = OpCode::from(self.read_byte());

            match instruction {
                chunk::OpCode::OpReturn => {
                    println!("{}", self.pop());
                    return InterpretResult::INTERPRET_OK;
                }
                chunk::OpCode::OpAdd => {
                    if let ValueType::String(_) = self.peek(0) {
                        self.concatenate();
                    } else {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a + b);
                    }
                }
                chunk::OpCode::OpSubtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                chunk::OpCode::OpMultiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                chunk::OpCode::OpDivide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
                chunk::OpCode::OpNegate => {
                    let value = self.pop();
                    self.push(-value);
                }
                chunk::OpCode::OpConstant => {
                    let index = self.read_byte();
                    let constant = self.read_constant(index);
                    self.push(constant);
                }
                chunk::OpCode::OpNil => {
                    self.push(ValueType::Nil);
                }
                chunk::OpCode::OpTrue => {
                    self.push(ValueType::Boolean(true));
                }
                chunk::OpCode::OpFalse => {
                    self.push(ValueType::Boolean(false));
                }
                chunk::OpCode::OpNot => {
                    let value = self.pop();
                    self.push(!value);
                }
                chunk::OpCode::OpEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a == b));
                }
                chunk::OpCode::OpGreater => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a > b));
                }
                chunk::OpCode::OpLess => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(ValueType::Boolean(a < b));
                }
                chunk::OpCode::OpPrint => {
                    println!("{}", self.pop());
                }
            }
        }
    }

    // The READ_BYTE macro reads the byte currently pointed at by ip and then advances the instruction pointer - book
    fn read_byte(&mut self) -> usize {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        return byte;
    }

    fn read_constant(&mut self, index: usize) -> ValueType {
        self.chunk.constants.values[index]
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
                // debug
                println!("Concatenated: {}", res);
                let res_idx = self.interner.intern_string(res);
                self.push(ValueType::String(res_idx));
            }
        }
    }
}
