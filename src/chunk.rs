use std::alloc::realloc;

use crate::value::{Value, ValueArray};

pub enum OpCode {
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpReturn,
}

impl From<OpCode> for usize {
    fn from(op: OpCode) -> usize {
        match op {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,    
            OpCode::OpDivide => 6,

        }
    }
}

impl From<usize> for OpCode {
    fn from(byte: usize) -> OpCode {
        match byte {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,

            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpConstant => write!(f, "OP_CONSTANT"),
            OpCode::OpNegate => write!(f, "OP_NEGATE"),
            OpCode::OpAdd => write!(f, "OP_ADD"),
            OpCode::OpSubtract => write!(f, "OP_SUBTRACT"),
            OpCode::OpMultiply => write!(f, "OP_MULTIPLY"),
            OpCode::OpDivide => write!(f, "OP_DIVIDE"),
            
        }
    }
}

pub struct Chunk {
    pub count: usize,
    capacity: usize,
    pub code: Vec<usize>,

    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: usize) {
        if self.capacity < self.count + 1 {
            self.capacity = std::cmp::max(8, self.capacity * 2); // grow capacity by 2x
            self.code.resize(self.capacity, 0); // resize the code vector to the new capacity
        }

        self.code[self.count] = byte;
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.count - 1 // return the index of the constant
    }

    pub fn free(&mut self) {
        self.code.clear();

        self.capacity = 0;
        self.count = 0;

        self.constants.free();
    }
}
