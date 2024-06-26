use serde::{Deserialize, Serialize};

use crate::{interner::StringObjIdx, tensor::Tensor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    // Tensor(Tensor), // TODO: Ideally, it should be seperate types for int and float (maybe?)
    String(StringObjIdx),
    Identifier(StringObjIdx),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Nil,
    // Lists, Dicts, Tensors, etc.
    JumpOffset(usize),

    Function(String),
}

// impl std::fmt::Display for ValueType {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             ValueType::Tensor(n) => write!(f, "{}", n),
//             ValueType::String(s) => write!(f, "{}", s),
//             ValueType::Identifier(s) => write!(f, "iden->{}", s),
//             ValueType::Boolean(b) => write!(f, "{}", b),
//             ValueType::Nil => write!(f, "nil"),
//             ValueType::JumpOffset(j) => write!(f, "jmp->{}", j),
//             ValueType::Function(s) => write!(f, "fn->{}", s),
//         }
//     }
// }

// impl custom display for ValueType which also takes interner
impl ValueType {
    pub fn display(&self, interner: &crate::interner::Interner) -> String {
        match self {
            // ValueType::Tensor(n) => format!("{}", n),
            ValueType::String(s) => interner.lookup(*s).to_string(),
            ValueType::Identifier(s) => interner.lookup(*s).to_string(),
            ValueType::Boolean(b) => format!("{}", b),
            ValueType::Integer(n) => format!("{}", n),
            ValueType::Float(n) => format!("{}", n),
            ValueType::Nil => format!("nil"),
            ValueType::JumpOffset(j) => format!("jmp->{}", j),
            ValueType::Function(s) => format!("fn->{}", s),
        }
    }
}

// impl +,-,*,/ for ValueType
impl std::ops::Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a + b),
            (ValueType::Integer(a), ValueType::Integer(b)) => ValueType::Integer(a + b),
            (ValueType::Float(a), ValueType::Float(b)) => ValueType::Float(a + b),
            (ValueType::Float(a), ValueType::Integer(b)) => ValueType::Float(a + b as f64),
            (ValueType::Integer(a), ValueType::Float(b)) => ValueType::Float(a as f64 + b),

            (a, b) => panic!("Operands must be numbers. Got: {:?} and {:?}", a, b),
        }
    }
}

impl std::ops::Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a - b),
            (ValueType::Integer(a), ValueType::Integer(b)) => ValueType::Integer(a - b),
            (ValueType::Float(a), ValueType::Float(b)) => ValueType::Float(a - b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a * b),
            (ValueType::Integer(a), ValueType::Integer(b)) => ValueType::Integer(a * b),
            (ValueType::Float(a), ValueType::Float(b)) => ValueType::Float(a * b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a / b),
            (ValueType::Integer(a), ValueType::Integer(b)) => ValueType::Integer(a / b),
            (ValueType::Float(a), ValueType::Float(b)) => ValueType::Float(a / b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Neg for ValueType {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            // ValueType::Tensor(n) => ValueType::Tensor(-n),
            ValueType::Integer(n) => ValueType::Integer(-n),
            ValueType::Float(n) => ValueType::Float(-n),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl std::ops::Not for ValueType {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            ValueType::Boolean(b) => ValueType::Boolean(!b),
            ValueType::Nil => ValueType::Boolean(true), // NOTE: nil is falsey, should likely be removed perhaps ?
            _ => panic!("Operand must be a boolean."),
        }
    }
}

impl std::cmp::PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => a == b,
            (ValueType::Integer(a), ValueType::Integer(b)) => a == b,
            (ValueType::Float(a), ValueType::Float(b)) => a == b,
            (ValueType::Boolean(a), ValueType::Boolean(b)) => a == b,
            (ValueType::Nil, ValueType::Nil) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for ValueType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => a.partial_cmp(b),
            // _ => None,
            (ValueType::Integer(a), ValueType::Integer(b)) => a.partial_cmp(b),
            (ValueType::Float(a), ValueType::Float(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// impl powf value
impl ValueType {
    pub fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            // (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a.pow(b)),
            (ValueType::Integer(a), ValueType::Integer(b)) => ValueType::Integer(a.pow(*b as u32)),
            (ValueType::Float(a), ValueType::Float(b)) => ValueType::Float(a.powf(*b)),
            (ValueType::Float(a), ValueType::Integer(b)) => ValueType::Float(a.powf(*b as f64)),
            (a, b) => panic!("{}", format!("Operands must be numbers. Got: {:?} and {:?}", a, b)),
        }
    }
}
