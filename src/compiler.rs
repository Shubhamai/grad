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

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            interner: Interner::default(),
        }
    }

    pub fn compile(&mut self, ast: Vec<ASTNode>) -> (Chunk, Interner) {
        // NOTE: stmts or exprs or whatever
        ast.iter().for_each(|stmt| self.visit(stmt.clone()));

        // add return
        self.chunk.write(VectorType::Code(OpCode::OpReturn));

        return (self.chunk.clone(), self.interner.clone());
    }

    fn visit(&mut self, node: ASTNode) {
        match node {
            ASTNode::Number(n) => {
                self.chunk.write(VectorType::Code(OpCode::OpConstant));

                let constant = self.chunk.add_constant(ValueType::Tensor(Tensor::new(n)));
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
            ASTNode::Identifier(iden) => {
                self.chunk.write(VectorType::Code(OpCode::OpGetGlobal));

                let global = self
                    .chunk
                    .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                self.chunk.write(VectorType::Constant(global));
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

                    Ops::UnaryOp(UnaryOp::Negate) => {
                        self.chunk.write(VectorType::Code(OpCode::OpNegate))
                    }

                    Ops::PostfixOp(PostfixOp::STAR_STAR) => {
                        self.chunk.write(VectorType::Code(OpCode::OpPower))
                    }
                    Ops::PostfixOp(PostfixOp::Call) => {
                        println!("Call");
                        self.chunk.write(VectorType::Code(OpCode::OpCall));
                        self.chunk
                            .write(VectorType::Constant(self.chunk.constants.len() - 1));
                        // TODO: need for testing for this - a.relu(c.relu()), a.relu().relu()
                    }
                    x => println!("Invalid operator {:?}", x),
                }
            }
            ASTNode::Print(expr) => {
                assert!(expr.len() == 1);
                self.visit(expr[0].clone());
                self.chunk.write(VectorType::Code(OpCode::OpPrint));
            }
            ASTNode::Let(iden, expr) => {
                assert!(expr.len() == 1);

                let global = self
                    .chunk
                    .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                // self.chunk.write(VectorType::Constant(global));

                self.visit(expr[0].clone());

                self.chunk.write(VectorType::Code(OpCode::OpDefineGlobal));
                self.chunk.write(VectorType::Constant(global));
            }
            ASTNode::Assign(iden, expr) => {
                assert!(expr.len() == 1);

                let global = self
                    .chunk
                    .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                // self.chunk.write(VectorType::Constant(global));

                self.visit(expr[0].clone());

                self.chunk.write(VectorType::Code(OpCode::OpSetGlobal));
                self.chunk.write(VectorType::Constant(global));
            }
            ASTNode::Callee(iden, args) => {
                println!("Callee");
                let global = self
                    .chunk
                    .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                self.chunk.write(VectorType::Constant(global));

                // for arg in args {
                //     self.visit(arg.clone());
                // }

                // self.chunk.write(VectorType::Code(OpCode::OpCall));
                // self.chunk.write(VectorType::Constant(args.len()));
            }
            _ => println!("Invalid ASTNode"), // TODO: handle this error
        }
    }
}
