use crate::chunk::{self, OpCode};

// impl disassemble chunk trait for  chunk class

pub trait Disassemble {
    fn disassemble_instruction(&self, offset: usize) -> usize;
    fn disassemble(&self, name: &str);
}

impl Disassemble for chunk::Chunk {
    fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.count {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        let instruction = OpCode::from(self.code[offset]);

        match instruction {
            chunk::OpCode::OpReturn => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpNegate => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpAdd => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpSubtract => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpMultiply => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpDivide => {
                println!("{}", instruction);
                return offset + 1;
            }
            chunk::OpCode::OpConstant => {
                let constant = self.code[offset + 1];
                println!("{} {:04} | {}", instruction, constant, self.constants.values[constant as usize]);
                return offset + 2;
            }
        }
    }
}
