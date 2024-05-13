use crate::{chunk, interner::Interner, value::ValueType};
use colored::*;

pub struct Debug {
    name: String,
    interner: Interner,
    chunk: chunk::Chunk,
}

impl Debug {
    pub fn new(name: &str, chunk: chunk::Chunk, interner: Interner) -> Self {
        Self {
            name: name.to_string(),
            chunk,
            interner,
        }
    }

    pub fn disassemble(&self) {
        println!(
            "{} {} {}",
            "======== ".blue().bold(),
            self.name.blue().bold(),
            " ========".blue().bold()
        );

        let mut offset = 0;
        while offset < self.chunk.code.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("{}", "====================".blue().bold());
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
                println!(
                    "{:04} {}",
                    offset.to_string().yellow(),
                    instruction.to_string().red()
                );

                return offset + 1;
            }
            chunk::VectorType::Code(
                chunk::OpCode::OpConstant
                | chunk::OpCode::OpDefineGlobal
                | chunk::OpCode::OpGetGlobal
                | chunk::OpCode::OpSetGlobal
                | chunk::OpCode::OpDefineLocal
                | chunk::OpCode::OpGetLocal
                | chunk::OpCode::OpSetLocal
                | chunk::OpCode::OpCall,
            ) => {
                let constant = self.chunk.code[offset + 1];
                match constant {
                    chunk::VectorType::Constant(idx) => match self.chunk.constants[idx] {
                        ValueType::String(s) | ValueType::Identifier(s) => {
                            println!(
                                "{:04} {} {:20} | {}{}",
                                offset.to_string().yellow(),
                                instruction.to_string().red(),
                                constant.to_string().green().italic(),
                                "intr->".purple().magenta().italic(),
                                self.interner.lookup(s).purple().magenta().italic()
                            );
                        }
                        _ => {
                            println!(
                                "{:04} {} {:20} | {}",
                                offset.to_string().yellow(),
                                instruction.to_string().red(),
                                constant.to_string().green().italic(),
                                self.chunk.constants[idx]
                                    .to_string()
                                    .purple()
                                    .magenta()
                                    .italic()
                            );
                        }
                    },
                    _ => {
                        unreachable!();
                    }
                }
                return offset + 2;
            }
            chunk::VectorType::Constant(_) => {
                return offset + 1;
            }
        }
    }
}
