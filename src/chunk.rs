use serde::{Deserialize, Serialize};

/// This module contains the implementation of the Chunk struct and its methods.
/// The Chunk struct is used to store the bytecode and the constants.
use crate::value::ValueType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
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
    OpPower,

    OpNot,
    OpEqualEqual,
    OpGreater,
    OpLess,
    OpReturn,
    OpPrint,
    OpPop,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,

    OpDefineLocal,
    OpGetLocal,
    OpSetLocal,

    OpJumpIfFalse,
    OpJump,
    OpLoop,

    // OpCall,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VectorType {
    Constant(usize),
    Code(OpCode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// VectorType is either a index to the constants or an OpCode, see `VectorType` enum
    pub code: Vec<VectorType>,
    pub constants: Vec<ValueType>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: VectorType) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: ValueType) -> usize {
        self.constants.push(value);
        self.constants.len() - 1 // return the index of the constant
    }
}

////////////////////////
/// Display impls
////////////////////////

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
            OpCode::OpPower => write!(f, "OP_POWER"),

            OpCode::OpNil => write!(f, "OP_NIL"),
            OpCode::OpTrue => write!(f, "OP_TRUE"),
            OpCode::OpFalse => write!(f, "OP_FALSE"),
            OpCode::OpNot => write!(f, "OP_NOT"),
            OpCode::OpEqualEqual => write!(f, "OP_EQUAL_EQUAL"),
            OpCode::OpGreater => write!(f, "OP_GREATER"),
            OpCode::OpLess => write!(f, "OP_LESS"),
            OpCode::OpPrint => write!(f, "OP_PRINT"),
            OpCode::OpPop => write!(f, "OP_POP"),
            OpCode::OpDefineGlobal => write!(f, "OP_DEFINE_GLOBAL"),
            OpCode::OpGetGlobal => write!(f, "OP_GET_GLOBAL"),
            OpCode::OpSetGlobal => write!(f, "OP_SET_GLOBAL"),

            OpCode::OpDefineLocal => write!(f, "OP_DEFINE_LOCAL"),
            OpCode::OpGetLocal => write!(f, "OP_GET_LOCAL"),
            OpCode::OpSetLocal => write!(f, "OP_SET_LOCAL"),

            OpCode::OpJumpIfFalse => write!(f, "OP_JUMP_IF_FALSE"),
            OpCode::OpJump => write!(f, "OP_JUMP"),
            OpCode::OpLoop => write!(f, "OP_LOOP"),

            // OpCode::OpCall => write!(f, "OP_CALL"),
        }
    }
}

impl std::fmt::Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VectorType::Constant(c) => write!(f, "cons->[{}]", c),
            VectorType::Code(op) => write!(f, "{}", op),
        }
    }
}
