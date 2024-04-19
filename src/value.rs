// pub enum ValueType {
//     Number(f64), // TODO: Ideally, it should be seperate types for int and float (maybe?)
//     // String(String),
//     // Boolean(bool),
//     // Nil,
//     // Lists, Dicts, Tensors, etc.
// }

pub type Value = f32;

#[derive(Debug)]
pub struct ValueArray {
    pub count: usize,
    capacity: usize,
    pub values: Vec<Value>,
}

impl ValueArray {
    pub(crate) fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            values: Vec::new(),
        }
    }

    pub(crate) fn write(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            self.capacity = std::cmp::max(8, self.capacity * 2); // grow capacity by 2x
            self.values.resize(self.capacity, 0.); // resize the code vector to the new capacity
        }

        self.values[self.count] = value;
        self.count += 1;
    }

    pub(crate) fn free(&mut self) {
        self.values.clear();
    }
}
