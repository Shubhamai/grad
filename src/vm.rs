use crate::{
    chunk::{self, Chunk, OpCode},
    debug::Disassemble,
    value::Value,
};

const STACK_MAX: usize = 256;

pub(crate) struct VM {
    pub chunk: Chunk,

    // instruction pointer
    ip: usize,

    // TODO - implement JIT instead of stack perhaps ?
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl VM {
    pub(crate) fn init(chunk: Chunk) -> VM {
        // let chunk = Chunk::new();

        VM {
            chunk,
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    // delete the vm and free the chunk
    pub(crate) fn free(&mut self) {
        todo!()
    }

    pub(crate) fn interpret(&mut self) -> InterpretResult {
        return self.run();
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
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
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
            }
        }
    }

    // The READ_BYTE macro reads the byte currently pointed at by ip and then advances the instruction pointer - book
    fn read_byte(&mut self) -> usize {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        return byte;
    }

    fn read_constant(&mut self, index: usize) -> f32 {
        self.chunk.constants.values[index]
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }
}
