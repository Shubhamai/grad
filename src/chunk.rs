use crate::{tensor::Tensor, value::ValueType};

#[derive(Debug, Clone, Copy)]
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

    OpCall,
}

#[derive(Debug, Clone, Copy)]
pub enum VectorType {
    Constant(usize),
    Code(OpCode),
}

#[derive(Debug, Clone)]
pub struct Chunk {
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

    pub fn write_jump(&mut self, op_code: OpCode, jump_to: usize) -> usize {
        // Write the jump instruction with a placeholder offset
        let offset = self.code.len(); // Placeholder offset
        self.write(VectorType::Code(op_code));
        self.add_constant(ValueType::Tensor(Tensor::from(jump_to as f64))); // Placeholder jump offset
        offset // Return the offset to be patched later
    }

    pub fn patch_jump(&mut self, offset: usize) {
        // Calculate the jump offset from the beginning of the code
        let jump_offset = self.code.len() - offset - 1;
        // Patch the jump instruction with the calculated offset
        println!("Patching jump at offset {} with jump offset {}", offset, jump_offset);
        // if let VectorType::Code(OpCode::OpJump) = self.code[offset].clone() {
        //     if let VectorType::Constant(ValueType::Tensor(mut tensor)) = self.code[offset + 1].clone() {
        //         tensor[0] = jump_offset as f64;
        //         self.code[offset + 1] = VectorType::Constant(ValueType::Tensor(tensor));
        //     }
        // } else if let VectorType::Code(OpCode::OpJumpIfFalse) = self.code[offset].clone() {
        //     if let VectorType::Constant(ValueType::Tensor(mut tensor)) = self.code[offset + 1].clone() {
        //         tensor[0] = jump_offset as f64;
        //         self.code[offset + 1] = VectorType::Constant(ValueType::Tensor(tensor));
        //     }
        // }
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

            OpCode::OpCall => write!(f, "OP_CALL"),
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
