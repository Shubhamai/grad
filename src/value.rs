// pub enum ValueType {
//     Number(f64), // TODO: Ideally, it should be seperate types for int and float (maybe?)
//     // String(String),
//     // Boolean(bool),
//     // Nil,
//     // Lists, Dicts, Tensors, etc.
// }

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Number(f32), // TODO: Ideally, it should be seperate types for int and float (maybe?)
    // String(String),
    Boolean(bool),
    Nil,
    // Lists, Dicts, Tensors, etc.
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueType::Number(n) => write!(f, "{}", n),
            // ValueType::String(s) => write!(f, "{}", s),
            ValueType::Boolean(b) => write!(f, "{}", b),
            ValueType::Nil => write!(f, "nil"),
        }
    }
}

// impl +,-,*,/ for ValueType
impl std::ops::Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a + b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a - b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a * b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a / b),
            _ => panic!("Operands must be numbers."),
        }
    }
}

impl std::ops::Neg for ValueType {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            ValueType::Number(n) => ValueType::Number(-n),
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
            (ValueType::Number(a), ValueType::Number(b)) => a == b,
            (ValueType::Boolean(a), ValueType::Boolean(b)) => a == b,
            (ValueType::Nil, ValueType::Nil) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for ValueType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueArray {
    // pub count: usize,
    // capacity: usize,
    pub values: Vec<ValueType>,
}

impl ValueArray {
    pub(crate) fn new() -> Self {
        Self {
            // count: 0,
            // capacity: 0,
            values: Vec::new(),
        }
    }

    pub(crate) fn write(&mut self, value: ValueType) {
        // if self.capacity < self.count + 1 {
        //     self.capacity = std::cmp::max(8, self.capacity * 2); // grow capacity by 2x
        //     self.values.resize(self.capacity, 0.); // resize the code vector to the new capacity
        // }

        // self.values[self.count] = value;
        // self.count += 1;

        self.values.push(value);
    }

    pub(crate) fn free(&mut self) {
        self.values.clear();
    }
}
