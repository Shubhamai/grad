
use crate::{
    ast::{ASTNode, BinaryOp, Ops, PostfixOp, UnaryOp},
    chunk::{Chunk, OpCode, VectorType},
    interner::Interner,
    tensor::Tensor,
    value::ValueType,
};

#[derive(Debug, Clone, Default)]
struct Local {
    name: String,
    depth: u8,
}

// impl display for Local
impl std::fmt::Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.name, self.depth)
    }
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    arity: u8,
    chunk: Chunk,
}

impl Function {
    pub fn new(name: String, arity: u8) -> Self {
        Self {
            name,
            arity,
            chunk: Chunk::new(),
        }
    }
}

pub struct Compiler {
    chunk: Chunk,
    interner: Interner,

    locals: Vec<Local>,
    local_count: usize,
    scope_depth: u8,

    functions: Vec<Function>,
    function_count: usize,
}

// write a macro that can take single or multiple opcodes and write them to the chunk, (without mentioning self.chunk)
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
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
            functions: Vec::new(),
            function_count: 0,
        }
    }

    pub fn compile(&mut self, ast: Vec<ASTNode>) -> (Chunk, Interner) {
        ast.iter().for_each(|stmt| self.visit(stmt.clone()));

        // add return
        self.chunk.write(VectorType::Code(OpCode::OpReturn));

        (self.chunk.clone(), self.interner.clone())
    }

    fn visit_function(&mut self, name: String, params: Vec<String>, body: Vec<ASTNode>) {
        let function = Function::new(name.clone(), params.len() as u8);
        self.functions.push(function);
        self.function_count += 1;

        // Compile function body
        for stmt in body {
            self.visit(stmt);
        }

        // Add return
        self.chunk.write(VectorType::Code(OpCode::OpReturn));

        let function_idx = self.chunk.add_constant(ValueType::Function(name));
        write_cons!(self.chunk, function_idx);
    }

    fn visit(&mut self, node: ASTNode) {
        match node {
            // ASTNode::Number(n) => {
            //     write_op!(self.chunk, OpCode::OpConstant);
            //     // add_con!(self.chunk, ValueType::Tensor(Tensor::from(n)));
            //     add_con!(self.chunk, ValueType::Float(n));
            //     write_cons!(self.chunk, self.chunk.constants.len() - 1);
            // }
            ASTNode::IntNumber(n) => {
                write_op!(self.chunk, OpCode::OpConstant);
                add_con!(self.chunk, ValueType::Integer(n));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
            }
            ASTNode::FloatNumber(n) => {
                write_op!(self.chunk, OpCode::OpConstant);
                add_con!(self.chunk, ValueType::Float(n));
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
                if let Some(local) = self.resolve_local(&iden) {
                    write_op!(self.chunk, OpCode::OpGetLocal);
                    write_cons!(self.chunk, local);
                } else {
                    write_op!(self.chunk, OpCode::OpGetGlobal);
                    let global = self
                        .chunk
                        .add_constant(ValueType::Identifier(self.interner.intern_string(iden)));
                    write_cons!(self.chunk, global);
                }
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
                        write_op!(self.chunk, OpCode::OpNot);
                    }
                    Ops::BinaryOp(BinaryOp::Lt) => write_op!(self.chunk, OpCode::OpLess),
                    Ops::BinaryOp(BinaryOp::Le) => {
                        write_op!(self.chunk, OpCode::OpGreater);
                        write_op!(self.chunk, OpCode::OpNot);
                    }
                    Ops::BinaryOp(BinaryOp::Gt) => {
                        write_op!(self.chunk, OpCode::OpGreater);
                    }
                    Ops::BinaryOp(BinaryOp::Ge) => {
                        write_op!(self.chunk, OpCode::OpLess);
                        write_op!(self.chunk, OpCode::OpNot);
                    }
                    Ops::UnaryOp(UnaryOp::Negate) => {
                        write_op!(self.chunk, OpCode::OpNegate);
                    }

                    Ops::PostfixOp(PostfixOp::StarStar) => {
                        write_op!(self.chunk, OpCode::OpPower);
                    }
                    Ops::PostfixOp(PostfixOp::Call) => {
                        panic!("Call not implemented");

                        // write_op!(self.chunk, OpCode::OpCall);
                        // self.chunk
                        //     .write(VectorType::Constant(self.chunk.constants.len() - 1));
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

                if self.scope_depth > 0 {
                    if self.local_count == 256 {
                        panic!("Too many local variables.");
                    }
                    self.locals.push(Local {
                        name: iden,
                        depth: self.scope_depth,
                    });
                    self.local_count += 1;
                    self.visit(expr[0].clone());
                    return;
                }

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
                self.visit(expr[0].clone());

                if let Some(local) = self.resolve_local(&iden) {
                    write_op!(self.chunk, OpCode::OpSetLocal);
                    write_cons!(self.chunk, local);
                } else {
                    let global = add_con!(
                        self.chunk,
                        ValueType::Identifier(self.interner.intern_string(iden))
                    );
                    write_op!(self.chunk, OpCode::OpSetGlobal);
                    write_cons!(self.chunk, global);
                }
            }
            ASTNode::Block(stmts) => {
                self.scope_depth += 1;
                for stmt in stmts {
                    self.visit(stmt);
                }
                self.scope_depth -= 1;

                while self.local_count > 0
                    && self.locals[self.local_count - 1].depth > self.scope_depth
                {
                    self.local_count -= 1;
                    write_op!(self.chunk, OpCode::OpPop);
                }
            }
            ASTNode::Callee(iden, _) => {
                let global = add_con!(
                    self.chunk,
                    ValueType::Identifier(self.interner.intern_string(iden))
                );
                write_cons!(self.chunk, global);
            }
            ASTNode::If(cond, then, els) => {
                assert_eq!(cond.len(), 1);
                self.visit(cond[0].clone());

                let else_jump_offset = self.chunk.code.len();
                write_op!(self.chunk, OpCode::OpJumpIfFalse);
                add_con!(self.chunk, ValueType::JumpOffset(else_jump_offset));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                let else_jump_const_idx = add_con!(self.chunk, ValueType::JumpOffset(0));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                write_op!(self.chunk, OpCode::OpPop);

                then.iter().for_each(|stmt| {
                    self.visit(stmt.clone())
                });

                let jump_to_end = self.chunk.code.len();
                write_op!(self.chunk, OpCode::OpJump);
                add_con!(self.chunk, ValueType::JumpOffset(jump_to_end));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                let jump_const_idx = add_con!(self.chunk, ValueType::JumpOffset(0));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                write_op!(self.chunk, OpCode::OpPop);

                let else_offset = self.chunk.code.len();
                self.chunk.constants[else_jump_const_idx] = ValueType::JumpOffset(else_offset - 1);

                // Compile the "else" block if it exists
                if let Some(els) = els {
                    els.iter().for_each(|stmt| self.visit(stmt.clone()));
                }

                let end_offset = self.chunk.code.len();
                self.chunk.constants[jump_const_idx] = ValueType::JumpOffset(end_offset);
            }
            ASTNode::While(cond, body) => {
                let loop_start = self.chunk.code.len();

                assert_eq!(cond.len(), 1);
                self.visit(cond[0].clone());

                let exit_jump_offset = self.chunk.code.len();
                write_op!(self.chunk, OpCode::OpJumpIfFalse);
                add_con!(self.chunk, ValueType::JumpOffset(exit_jump_offset));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                let exit_jump_const_idx = add_con!(self.chunk, ValueType::JumpOffset(0));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                write_op!(self.chunk, OpCode::OpPop);

                body.iter().for_each(|stmt| self.visit(stmt.clone()));

                let loop_jump_offset = self.chunk.code.len();
                write_op!(self.chunk, OpCode::OpLoop);
                add_con!(self.chunk, ValueType::JumpOffset(loop_jump_offset));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                add_con!(self.chunk, ValueType::JumpOffset(loop_start));
                write_cons!(self.chunk, self.chunk.constants.len() - 1);
                write_op!(self.chunk, OpCode::OpPop);

                let exit_offset = self.chunk.code.len();
                self.chunk.constants[exit_jump_const_idx] = ValueType::JumpOffset(exit_offset - 1);
            }
            ASTNode::Function(name, params, body) => {
                self.visit_function(name, params, body);
            }
        }
    }

    fn resolve_local(&self, name: &String) -> Option<usize> {
        for i in (0..self.local_count).rev() {
            if self.locals[i].name == *name {
                return Some(i);
            }
        }
        None
    }
}
