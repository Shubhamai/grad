use crate::{
    ast::{ASTNode, BinaryOp, Ops},
    chunk::{Chunk, OpCode, VectorType},
    interner::Interner,
    value::ValueType,
};

pub struct Compiler {
    chunk: Chunk,
    interner: Interner,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            interner: Interner::default(),
        }
    }

    pub fn compile(&mut self, ast: ASTNode) -> (Chunk, Interner) {
        self.visit(ast);

        // add return
        self.chunk.write(VectorType::Code(OpCode::OpReturn));

        return (self.chunk.clone(), self.interner.clone());
    }

    fn visit(&mut self, node: ASTNode) {
        match node {
            ASTNode::Number(n) => {
                self.chunk.write(VectorType::Code(OpCode::OpConstant));

                let constant = self.chunk.add_constant(ValueType::Number(n));
                self.chunk.write(VectorType::Constant(constant));
            }
            ASTNode::Boolean(b) => {
                if b {
                    self.chunk.write(VectorType::Code(OpCode::OpTrue));
                } else {
                    self.chunk.write(VectorType::Code(OpCode::OpFalse));
                }
            }
            ASTNode::String(s) => {
                self.chunk.write(VectorType::Code(OpCode::OpConstant));
                let constant = self
                    .chunk
                    .add_constant(ValueType::String(self.interner.intern_string(s)));
                self.chunk.write(VectorType::Constant(constant));
            }
            ASTNode::Op(op, vec) => {
                for node in vec {
                    self.visit(node);
                }

                match op {
                    Ops::BinaryOp(BinaryOp::Add) => {
                        self.chunk.write(VectorType::Code(OpCode::OpAdd))
                    }
                    Ops::BinaryOp(BinaryOp::Sub) => {
                        self.chunk.write(VectorType::Code(OpCode::OpSubtract))
                    }
                    Ops::BinaryOp(BinaryOp::Mul) => {
                        self.chunk.write(VectorType::Code(OpCode::OpMultiply))
                    }
                    Ops::BinaryOp(BinaryOp::Div) => {
                        self.chunk.write(VectorType::Code(OpCode::OpDivide))
                    }
                    Ops::BinaryOp(BinaryOp::Eq) => {
                        self.chunk.write(VectorType::Code(OpCode::OpEqualEqual))
                    }
                    Ops::BinaryOp(BinaryOp::Ne) => {
                        self.chunk.write(VectorType::Code(OpCode::OpEqualEqual));
                        self.chunk.write(VectorType::Code(OpCode::OpNot))
                    }
                    Ops::BinaryOp(BinaryOp::Lt) => {
                        self.chunk.write(VectorType::Code(OpCode::OpLess))
                    }
                    Ops::BinaryOp(BinaryOp::Le) => {
                        self.chunk.write(VectorType::Code(OpCode::OpGreater));
                        self.chunk.write(VectorType::Code(OpCode::OpNot))
                    }
                    Ops::BinaryOp(BinaryOp::Gt) => {
                        self.chunk.write(VectorType::Code(OpCode::OpGreater))
                    }
                    Ops::BinaryOp(BinaryOp::Ge) => {
                        self.chunk.write(VectorType::Code(OpCode::OpLess));
                        self.chunk.write(VectorType::Code(OpCode::OpNot))
                    }

                    _ => println!("Invalid operator"), // TODO: handle this error
                }
            }
            _ => println!("Invalid ASTNode"), // TODO: handle this error
        }
    }
}
