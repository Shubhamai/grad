use crate::{interner::StringObjIdx, tensor::Tensor};

#[derive(Debug, Clone)]
pub enum ValueType {
    Tensor(Tensor), // TODO: Ideally, it should be seperate types for int and float (maybe?)
    String(StringObjIdx),
    Identifier(StringObjIdx),
    Boolean(bool),
    Nil,
    // Lists, Dicts, Tensors, etc.
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueType::Tensor(n) => write!(f, "tensor->{}", n),
            ValueType::String(s) => write!(f, "str->{}", s),
            ValueType::Identifier(s) => write!(f, "iden->{}", s),
            ValueType::Boolean(b) => write!(f, "bool->{}", b),
            ValueType::Nil => write!(f, "nil"),
        }
    }
}

// impl +,-,*,/ for ValueType
impl std::ops::Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a + b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a - b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a * b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a / b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Neg for ValueType {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            ValueType::Tensor(n) => ValueType::Tensor(-n),
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
            (ValueType::Tensor(a), ValueType::Tensor(b)) => a == b,
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
            _ => None,
        }
    }
}

// impl powf value
impl ValueType {
    pub fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            (ValueType::Tensor(a), ValueType::Tensor(b)) => ValueType::Tensor(a.pow(b)),
            _ => panic!("Operands must be numbers."),
        }
    }
}
