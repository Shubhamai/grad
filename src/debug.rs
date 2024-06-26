use crate::{chunk, interner::Interner, value::ValueType};
use colored::*;

pub struct Debug {
    name: String,
    interner: Interner,
    chunk: chunk::Chunk,
    use_colors: bool,
}

impl Debug {
    pub fn new(name: &str, chunk: chunk::Chunk, interner: Interner) -> Self {
        Self {
            name: name.to_string(),
            chunk,
            interner,
            use_colors: true,
        }
    }

    pub fn set_color_usage(&mut self, use_colors: bool) {
        self.use_colors = use_colors;
    }

    pub fn disassemble(&self) -> String {
        let mut output = Vec::new();
        output.push(self.format_header());

        let mut offset = 0;
        while offset < self.chunk.code.len() {
            let (new_offset, instruction) = self.disassemble_instruction(offset);
            output.push(instruction);
            offset = new_offset;
        }

        output.push(self.format_footer());
        output.join("\n")
    }

    pub fn disassemble_instruction(&self, offset: usize) -> (usize, String) {
        let instruction = self.chunk.code.get(offset).ok_or_else(|| {
            format!("Invalid offset {} in chunk of length {}", offset, self.chunk.code.len())
        }).unwrap();

        match instruction {
            chunk::VectorType::Code(op) if op.is_simple() => {
                (offset + 1, self.format_simple_instruction(offset, op))
            },
            chunk::VectorType::Code(op) if op.uses_constant() => {
                self.format_constant_instruction(offset, op)
            },
            chunk::VectorType::Code(op) if op.is_jump() => {
                self.format_jump_instruction(offset, op)
            },
            chunk::VectorType::Constant(_) => {
                (offset + 1, "Unexpected constant in code vector".to_string())
            },
            _ => (offset + 1, "Unknown instruction type".to_string()),
        }
    }

    fn format_simple_instruction(&self, offset: usize, op: &chunk::OpCode) -> String {
        format!("{} {}", self.colorize_offset(offset), self.colorize_op(op))
    }

    fn format_constant_instruction(&self, offset: usize, op: &chunk::OpCode) -> (usize, String) {
        let constant_idx = self.chunk.code.get(offset + 1)
            .and_then(|v| if let chunk::VectorType::Constant(idx) = v { Some(*idx) } else { None })
            .ok_or_else(|| "Invalid constant index".to_string())
            .unwrap();

        let constant = &self.chunk.constants[constant_idx];
        let constant_str = self.format_constant(constant_idx);
        
        (offset + 2, format!("{} {} {} | {}",
            self.colorize_offset(offset),
            self.colorize_op(op),
            self.colorize_constant_idx(constant_idx),
            self.colorize_constant_str(&constant_str)))
    }

    fn format_jump_instruction(&self, offset: usize, op: &chunk::OpCode) -> (usize, String) {
        let current_loc = self.get_constant_value(offset + 1);
        let jump_offset = self.get_constant_value(offset + 2);
        
        (offset + 3, format!("{} {} | {}->{}",
            self.colorize_offset(offset),
            self.colorize_op(op),
            self.colorize_jump_loc(&current_loc),
            self.colorize_jump_offset(&jump_offset)))
    }

    pub fn format_constant(&self, idx: usize) -> String {
        let constant = &self.chunk.constants[idx];
        match constant {
            ValueType::String(s) | ValueType::Identifier(s) => {
                format!("intr->{}", self.interner.lookup(*s))
            },
            _ => constant.display(&self.interner),
        }
    }

    fn get_constant_value(&self, offset: usize) -> String {
        self.chunk.code.get(offset)
            .and_then(|v| if let chunk::VectorType::Constant(idx) = v {
                Some(self.chunk.constants[*idx].display(&self.interner))
            } else { None })
            .unwrap_or_else(|| "Invalid constant".to_string())
    }

    fn format_header(&self) -> String {
        self.colorize_header(&format!("======== {} ========", self.name))
    }

    fn format_footer(&self) -> String {
        self.colorize_footer("====================")
    }

    fn colorize_offset(&self, offset: usize) -> String {
        if self.use_colors {
            format!("{:04}", offset).yellow().to_string()
        } else {
            format!("{:04}", offset)
        }
    }

    fn colorize_op(&self, op: &chunk::OpCode) -> String {
        if self.use_colors {
            op.to_string().red().to_string()
        } else {
            op.to_string()
        }
    }

    fn colorize_constant_idx(&self, idx: usize) -> String {
        if self.use_colors {
            format!("{:20}", idx).green().italic().to_string()
        } else {
            format!("{:20}", idx)
        }
    }

    fn colorize_constant_str(&self, s: &str) -> String {
        if self.use_colors {
            s.purple().magenta().italic().to_string()
        } else {
            s.to_string()
        }
    }

    fn colorize_jump_loc(&self, loc: &str) -> String {
        if self.use_colors {
            loc.purple().magenta().italic().to_string()
        } else {
            loc.to_string()
        }
    }

    fn colorize_jump_offset(&self, offset: &str) -> String {
        if self.use_colors {
            offset.purple().magenta().italic().to_string()
        } else {
            offset.to_string()
        }
    }

    fn colorize_header(&self, s: &str) -> String {
        if self.use_colors {
            s.blue().bold().to_string()
        } else {
            s.to_string()
        }
    }

    fn colorize_footer(&self, s: &str) -> String {
        if self.use_colors {
            s.blue().bold().to_string()
        } else {
            s.to_string()
        }
    }
}

trait OpCodeExt {
    fn is_simple(&self) -> bool;
    fn uses_constant(&self) -> bool;
    fn is_jump(&self) -> bool;
}

impl OpCodeExt for chunk::OpCode {
    fn is_simple(&self) -> bool {
        matches!(self, 
            chunk::OpCode::OpReturn | chunk::OpCode::OpNegate | chunk::OpCode::OpAdd |
            chunk::OpCode::OpSubtract | chunk::OpCode::OpMultiply | chunk::OpCode::OpDivide |
            chunk::OpCode::OpPower | chunk::OpCode::OpNil | chunk::OpCode::OpTrue |
            chunk::OpCode::OpFalse | chunk::OpCode::OpNot | chunk::OpCode::OpEqualEqual |
            chunk::OpCode::OpGreater | chunk::OpCode::OpLess | chunk::OpCode::OpPrint |
            chunk::OpCode::OpPop
        )
    }

    fn uses_constant(&self) -> bool {
        matches!(self,
            chunk::OpCode::OpConstant | chunk::OpCode::OpDefineGlobal |
            chunk::OpCode::OpGetGlobal | chunk::OpCode::OpSetGlobal |
            chunk::OpCode::OpDefineLocal | chunk::OpCode::OpGetLocal |
            chunk::OpCode::OpSetLocal
        )
    }

    fn is_jump(&self) -> bool {
        matches!(self,
            chunk::OpCode::OpJump | chunk::OpCode::OpJumpIfFalse | chunk::OpCode::OpLoop
        )
    }
}