use crate::chunk;

pub struct Debug {
    name: String,
    chunk: chunk::Chunk,
}

impl Debug {
    pub fn new(name: &str, chunk: chunk::Chunk) -> Self {
        Self {
            name: name.to_string(),
            chunk,
        }
    }

    pub fn disassemble(&self) {
        println!("======== {} ========", self.name);

        let mut offset = 0;
        while offset < self.chunk.code.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("====================");
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        let instruction = self.chunk.code[offset];

        match instruction {
            chunk::VectorType::Code(chunk::OpCode::OpReturn)
            | chunk::VectorType::Code(chunk::OpCode::OpNegate)
            | chunk::VectorType::Code(chunk::OpCode::OpAdd)
            | chunk::VectorType::Code(chunk::OpCode::OpSubtract)
            | chunk::VectorType::Code(chunk::OpCode::OpMultiply)
            | chunk::VectorType::Code(chunk::OpCode::OpDivide)
            | chunk::VectorType::Code(chunk::OpCode::OpPower)
            | chunk::VectorType::Code(chunk::OpCode::OpNil)
            | chunk::VectorType::Code(chunk::OpCode::OpTrue)
            | chunk::VectorType::Code(chunk::OpCode::OpFalse)
            | chunk::VectorType::Code(chunk::OpCode::OpNot)
            | chunk::VectorType::Code(chunk::OpCode::OpEqualEqual)
            | chunk::VectorType::Code(chunk::OpCode::OpGreater)
            | chunk::VectorType::Code(chunk::OpCode::OpLess)
            | chunk::VectorType::Code(chunk::OpCode::OpPrint)
            | chunk::VectorType::Code(chunk::OpCode::OpPop) => {
                println!("{:04} {}", offset, instruction);

                return offset + 1;
            }
            chunk::VectorType::Code(
                chunk::OpCode::OpConstant
                | chunk::OpCode::OpDefineGlobal
                | chunk::OpCode::OpGetGlobal
                | chunk::OpCode::OpSetGlobal,
            ) => {
                let constant = self.chunk.code[offset + 1];
                match constant {
                    chunk::VectorType::Constant(idx) => {
                        println!(
                            "{:04} {} {:04} | {}",
                            offset, instruction, constant, self.chunk.constants[idx]
                        );
                    }
                    _ => {
                        println!("{:04} {} {:04}", offset, instruction, constant);
                    }
                }
                return offset + 2;
            }
            chunk::VectorType::Constant(_) => {
                return offset +1;
            }
        }
    }
}
