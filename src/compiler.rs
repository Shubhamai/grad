use crate::{
    ast::{ASTNode, BinaryOp, Ops, PostfixOp, UnaryOp},
    chunk::{Chunk, OpCode, VectorType},
    interner::Interner,
    tensor::Tensor,
    value::ValueType,
};

pub struct Compiler {
    chunk: Chunk,
    interner: Interner,
}

// write a macro that can take single of multiple opcode and write it to the chunk, ( without mentioning self.chunk )
macro_rules! write_op {
    ($chunk:expr, $($op:expr),*) => {
        $( $chunk.write(VectorType::Code($op)))*
    };
}

macro_rules! add_con {
    ($chunk:expr, $constant:expr) => {
        $chunk.add_constant($constant)
    };
}

macro_rules! write_cons {
    ($chunk:expr, $code:expr) => {
        $chunk.write(VectorType::Constant($code))
    };
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            interner: Interner::default(),
        }
    }

    pub fn compile(&mut self, ast: Vec<ASTNode>) -> (Chunk, Interner) {
        ast.iter().for_each(|stmt| self.visit(stmt.clone()));

        // add return
        self.chunk.write(VectorType::Code(OpCode::OpReturn));

        (self.chunk.clone(), self.interner.clone())
    }

    fn visit(&mut self, node: ASTNode) {
        match node {
            ASTNode::Number(n) => {
                write_op!(self.chunk, OpCode::OpConstant);
                add_con!(self.chunk, ValueType::Tensor(Tensor::from(n)));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
            }
            ASTNode::Boolean(b) => {
                write_op!(self.chunk, if b { OpCode::OpTrue } else { OpCode::OpFalse })
            }

            ASTNode::String(s) => {
                write_op!(self.chunk, OpCode::OpConstant);
                add_con!(
                    self.chunk,
                    ValueType::String(self.interner.intern_string(s))
                );
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
            }
            ASTNode::Identifier(iden) => {
                write_op!(self.chunk, OpCode::OpGetGlobal);
                let global = self
                    .chunk
                    .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                write_cons!(self.chunk, global);
            }
            ASTNode::Op(op, vec) => {
                for node in vec {
                    self.visit(node);
                }

                match op {
                    Ops::BinaryOp(BinaryOp::Add) => write_op!(self.chunk, OpCode::OpAdd),
                    Ops::BinaryOp(BinaryOp::Sub) => write_op!(self.chunk, OpCode::OpSubtract),
                    Ops::BinaryOp(BinaryOp::Mul) => write_op!(self.chunk, OpCode::OpMultiply),
                    // @ - dot product - TODO: need to implement
                    Ops::BinaryOp(BinaryOp::At) => write_op!(self.chunk, OpCode::OpMultiply),
                    Ops::BinaryOp(BinaryOp::Div) => write_op!(self.chunk, OpCode::OpDivide),
                    Ops::BinaryOp(BinaryOp::Eq) => write_op!(self.chunk, OpCode::OpEqualEqual),
                    Ops::BinaryOp(BinaryOp::Ne) => {
                        write_op!(self.chunk, OpCode::OpEqualEqual);
                        write_op!(self.chunk, OpCode::OpNot)
                    }
                    Ops::BinaryOp(BinaryOp::Lt) => write_op!(self.chunk, OpCode::OpLess),
                    Ops::BinaryOp(BinaryOp::Le) => {
                        write_op!(self.chunk, OpCode::OpGreater);
                        write_op!(self.chunk, OpCode::OpNot)
                    }
                    Ops::BinaryOp(BinaryOp::Gt) => write_op!(self.chunk, OpCode::OpGreater),
                    Ops::BinaryOp(BinaryOp::Ge) => {
                        write_op!(self.chunk, OpCode::OpLess);
                        write_op!(self.chunk, OpCode::OpNot)
                    }
                    Ops::UnaryOp(UnaryOp::Negate) => write_op!(self.chunk, OpCode::OpNegate),

                    Ops::PostfixOp(PostfixOp::StarStar) => write_op!(self.chunk, OpCode::OpPower),
                    Ops::PostfixOp(PostfixOp::Call) => {
                        // self.chunk.write(VectorType::Code(OpCode::OpCall));
                        write_op!(self.chunk, OpCode::OpCall);
                        self.chunk
                            .write(VectorType::Constant(self.chunk.constants.len() - 1));
                        // TODO: need for testing for this - a.relu(c.relu()), a.relu().relu()
                    }
                    Ops::UnaryOp(UnaryOp::Not) | Ops::PostfixOp(PostfixOp::Index) => todo!(),
                }
            }
            ASTNode::Print(expr) => {
                assert!(expr.len() == 1);
                self.visit(expr[0].clone());
                write_op!(self.chunk, OpCode::OpPrint);
            }
            ASTNode::Let(iden, expr) => {
                assert!(expr.len() == 1);

                let global = add_con!(
                    self.chunk,
                    ValueType::Identifier(self.interner.intern_string(iden))
                );
                self.visit(expr[0].clone());
                write_op!(self.chunk, OpCode::OpDefineGlobal);
                write_cons!(self.chunk, global);
            }
            ASTNode::Assign(iden, expr) => {
                assert!(expr.len() == 1);

                let global = add_con!(
                    self.chunk,
                    ValueType::Identifier(self.interner.intern_string(iden))
                );
                self.visit(expr[0].clone());
                write_op!(self.chunk, OpCode::OpSetGlobal);
                write_cons!(self.chunk, global);
            }
            ASTNode::Callee(iden, _) => {
                let global = add_con!(
                    self.chunk,
                    ValueType::Identifier(self.interner.intern_string(iden))
                );
                write_cons!(self.chunk, global);
            }
        }
    }
}
