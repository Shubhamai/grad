use std::alloc::realloc;

use crate::value::{ValueArray, ValueType};

pub enum OpCode {
    OpConstant,
    OpNil,
    OpTrue,
    OpFalse,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpReturn,
    OpPrint,
    OpPop,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpPower
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
            OpCode::OpNil => 7,
            OpCode::OpTrue => 8,
            OpCode::OpFalse => 9,
            OpCode::OpNot => 10,
            OpCode::OpEqual => 11,
            OpCode::OpGreater => 12,
            OpCode::OpLess => 13,
            OpCode::OpPrint => 14,
            OpCode::OpPop => 15,
            OpCode::OpDefineGlobal => 16,
            OpCode::OpGetGlobal => 17,
            OpCode::OpSetGlobal => 18,
            OpCode::OpPower => 19
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
            7 => OpCode::OpNil,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            11 => OpCode::OpEqual,
            12 => OpCode::OpGreater,
            13 => OpCode::OpLess,
            14 => OpCode::OpPrint,
            15 => OpCode::OpPop,
            16 => OpCode::OpDefineGlobal,
            17 => OpCode::OpGetGlobal,
            18 => OpCode::OpSetGlobal,
            19 => OpCode::OpPower,
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
            OpCode::OpNil => write!(f, "OP_NIL"),
            OpCode::OpTrue => write!(f, "OP_TRUE"),
            OpCode::OpFalse => write!(f, "OP_FALSE"),
            OpCode::OpNot => write!(f, "OP_NOT"),
            OpCode::OpEqual => write!(f, "OP_EQUAL"),
            OpCode::OpGreater => write!(f, "OP_GREATER"),
            OpCode::OpLess => write!(f, "OP_LESS"),
            OpCode::OpPrint => write!(f, "OP_PRINT"),
            OpCode::OpPop => write!(f, "OP_POP"),
            OpCode::OpDefineGlobal => write!(f, "OP_DEFINE_GLOBAL"),
            OpCode::OpGetGlobal => write!(f, "OP_GET_GLOBAL"),
            OpCode::OpSetGlobal => write!(f, "OP_SET_GLOBAL"),
            OpCode::OpPower => write!(f, "OP_POWER"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    // pub count: usize,
    // capacity: usize,

    // TODO: I am using 4/8 bytes for the instruction, but I should be using a byte
    pub code: Vec<usize>,
    pub lines: Vec<usize>,

    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            // count: 0,
            // capacity: 0,
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: usize, line: usize) {
        // if self.capacity < self.count + 1 {
        //     self.capacity = std::cmp::max(8, self.capacity * 2); // grow capacity by 2x
        //     self.code.resize(self.capacity, 0); // resize the code vector to the new capacity
        //     self.lines.resize(self.capacity, 0); // resize the lines vector to the new capacity
        // }

        // self.code[self.count] = byte;
        // self.lines[self.count] = line;
        // self.count += 1;

        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: ValueType) -> usize {
        self.constants.write(value);
        self.constants.values.len() - 1 // return the index of the constant
    }

    pub fn free(&mut self) {
        self.code.clear();

        // self.capacity = 0;
        // self.count = 0;

        self.constants.free();
    }
}
