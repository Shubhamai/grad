#[derive(Debug, Clone, Copy)]
pub struct Tensor {
    data: f64,
    pub grad: f64,
    pub shape: [usize; 1],
}

// display tensor
impl std::fmt::Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, grad: {}", self.data, self.grad)
    }
}

impl Tensor {
    pub fn new(data: f64) -> Self {
        Tensor {
            data,
            grad: 0.0,
            shape: [0],
        }
    }

    pub fn powf(&self, exp: Tensor) -> Self {
        Tensor::new(self.data.powf(exp.data))
    }

    pub fn relu(&self) -> Self {
        Tensor::new(self.data.max(0.0))
    }

    pub fn backward(&mut self) {
        Tensor::new(self.grad);
    }

    pub fn grad(&mut self) {
        self.grad = 1.;
    }
}

impl std::ops::Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tensor::new(self.data + other.data)
    }
}

impl std::ops::Sub for Tensor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tensor::new(self.data - other.data)
    }
}

impl std::ops::Mul for Tensor {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tensor::new(self.data * other.data)
    }
}

impl std::ops::Div for Tensor {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Tensor::new(self.data / other.data)
    }
}

impl std::ops::Neg for Tensor {
    type Output = Self;

    fn neg(self) -> Self {
        Tensor::new(-self.data)
    }
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl PartialOrd for Tensor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}
